use derive_more::{Deref, DerefMut};
use futures::Stream;
use joinerror::{OptionExt, ResultExt};
use moss_applib::{AppRuntime, PublicServiceMarker, ServiceMarker};
use moss_collection::{
    Collection as CollectionHandle, CollectionBuilder, CollectionModifyParams,
    builder::{
        CollectionCloneGitParams, CollectionCloneParams, CollectionCreateGitParams,
        CollectionCreateParams, CollectionLoadParams,
    },
    collection::VcsSummary,
};
use moss_fs::{FileSystem, RemoveOptions, error::FsResultExt};
use moss_git_hosting_provider::{
    github::client::GitHubClient, gitlab::client::GitLabClient, models::primitives::GitProviderType,
};
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    pin::Pin,
    sync::Arc,
};
use tokio::sync::RwLock;

use crate::{
    dirs,
    models::{
        primitives::CollectionId,
        types::{CreateCollectionGitParams, CreateCollectionParams, UpdateCollectionParams},
    },
    services::storage_service::StorageService,
    storage::segments::SEGKEY_COLLECTION,
};

pub(crate) struct CollectionItemCloneParams {
    pub _name: String,
    pub order: isize,
    pub _icon_path: Option<PathBuf>,
    pub git_params: CollectionItemGitCloneParams,
}

pub(crate) struct CollectionItemGitCloneParams {
    pub repository: String,
    pub git_provider_type: GitProviderType,
    pub branch: Option<String>,
}

#[derive(Deref, DerefMut)]
struct CollectionItem<R: AppRuntime> {
    pub id: CollectionId,
    pub order: Option<isize>,

    #[deref]
    #[deref_mut]
    pub handle: Arc<CollectionHandle<R>>,
}

pub(crate) struct CollectionItemDescription {
    pub id: CollectionId,
    pub name: String,
    pub order: Option<isize>,
    pub expanded: bool,
    #[allow(dead_code)]
    pub repository: Option<String>,

    // FIXME: Do we need this field?
    pub icon_path: Option<PathBuf>,
    pub abs_path: Arc<Path>,
    pub external_path: Option<PathBuf>,
}

#[derive(Default)]
struct ServiceState<R: AppRuntime> {
    collections: HashMap<CollectionId, CollectionItem<R>>,
    expanded_items: HashSet<CollectionId>,
}

pub struct CollectionService<R: AppRuntime> {
    abs_path: PathBuf,
    fs: Arc<dyn FileSystem>,
    storage: Arc<StorageService<R>>,
    state: Arc<RwLock<ServiceState<R>>>,
    github_client: Arc<GitHubClient>,
    gitlab_client: Arc<GitLabClient>,
}

impl<R: AppRuntime> ServiceMarker for CollectionService<R> {}
impl<R: AppRuntime> PublicServiceMarker for CollectionService<R> {}

impl<R: AppRuntime> CollectionService<R> {
    pub(crate) async fn new(
        ctx: &R::AsyncContext,
        abs_path: &Path,
        fs: Arc<dyn FileSystem>,
        storage: Arc<StorageService<R>>,
        github_client: Arc<GitHubClient>,
        gitlab_client: Arc<GitLabClient>,
    ) -> joinerror::Result<Self> {
        let abs_path = abs_path.join(dirs::COLLECTIONS_DIR);
        let expanded_items = if let Ok(expanded_items) = storage.get_expanded_items(ctx).await {
            expanded_items.into_iter().collect::<HashSet<_>>()
        } else {
            HashSet::new()
        };

        let collections = restore_collections(
            ctx,
            &abs_path,
            &fs,
            &storage,
            github_client.clone(),
            gitlab_client.clone(),
        )
        .await?;

        Ok(Self {
            abs_path,
            fs,
            storage,
            state: Arc::new(RwLock::new(ServiceState {
                collections,
                expanded_items,
            })),
            github_client,
            gitlab_client,
        })
    }

    pub async fn collection(&self, id: &CollectionId) -> Option<Arc<CollectionHandle<R>>> {
        let state_lock = self.state.read().await;
        state_lock
            .collections
            .get(id)
            .map(|item| item.handle.clone())
    }

    pub(crate) async fn create_collection(
        &self,
        ctx: &R::AsyncContext,
        id: &CollectionId,
        params: &CreateCollectionParams,
    ) -> joinerror::Result<CollectionItemDescription> {
        let id_str = id.to_string();
        let abs_path: Arc<Path> = self.abs_path.join(id_str).into();
        if abs_path.exists() {
            return Err(joinerror::Error::new::<()>(format!(
                "collection directory `{}` already exists",
                abs_path.display()
            )));
        }

        self.fs
            .create_dir(&abs_path)
            .await
            .join_err_with::<()>(|| {
                format!("failed to create directory `{}`", abs_path.display())
            })?;

        let git_params = match params.git_params.as_ref() {
            None => None,
            Some(CreateCollectionGitParams::GitHub(git_params)) => {
                Some(CollectionCreateGitParams {
                    git_provider_type: GitProviderType::GitHub,
                    repository: git_params.repository.clone(),
                    branch: git_params.branch.clone(),
                })
            }
            Some(CreateCollectionGitParams::GitLab(git_params)) => {
                Some(CollectionCreateGitParams {
                    git_provider_type: GitProviderType::GitLab,
                    repository: git_params.repository.clone(),
                    branch: git_params.branch.clone(),
                })
            }
        };

        let collection_result = CollectionBuilder::new(
            self.fs.clone(),
            self.github_client.clone(),
            self.gitlab_client.clone(),
        )
        .create(
            ctx,
            CollectionCreateParams {
                name: Some(params.name.to_owned()),
                internal_abs_path: abs_path.clone().into(),
                external_abs_path: params.external_path.as_deref().map(|p| p.to_owned().into()),
                git_params,
                icon_path: params.icon_path.to_owned(),
            },
        )
        .await
        .join_err::<()>("failed to build collection");

        // TODO: Use atomic-fs to rollback changes
        // Remove collection folder if collection fails to be created
        let collection = match collection_result {
            Ok(collection) => collection,
            Err(e) => {
                // TODO: Log or tell the frontend we failed to clean up after operation failure
                let _ = self
                    .fs
                    .remove_dir(
                        &abs_path,
                        RemoveOptions {
                            recursive: true,
                            ignore_if_not_exists: true,
                        },
                    )
                    .await
                    .join_err_with::<()>(|| {
                        format!("failed to remove directory `{}`", abs_path.display())
                    });
                return Err(e);
            }
        };

        let icon_path = collection.icon_path();

        // let on_did_change = collection.on_did_change().subscribe(|_event| async move {

        //     // TODO: Save in the database whether the collection was collapsed/expanded
        // });
        // ctx.subscribe(Subscribe::OnCollectionDidChange(id, on_did_change))
        //     .await;

        let mut state_lock = self.state.write().await;
        state_lock.expanded_items.insert(id.to_owned());
        state_lock.collections.insert(
            id.to_owned(),
            CollectionItem {
                id: id.to_owned(),
                order: Some(params.order),
                handle: Arc::new(collection),
            },
        );

        {
            let mut txn = self
                .storage
                .begin_write(ctx)
                .await
                .join_err::<()>("failed to start transaction")?;

            self.storage
                .put_item_order_txn(ctx, &mut txn, id.as_str(), params.order)
                .await?;
            self.storage
                .put_expanded_items_txn(ctx, &mut txn, &state_lock.expanded_items)
                .await?;

            txn.commit()?;
        }

        Ok(CollectionItemDescription {
            id: id.to_owned(),
            name: params.name.clone(),
            order: Some(params.order),
            expanded: true,
            repository: None,
            icon_path,
            abs_path: abs_path.into(),
            external_path: params.external_path.clone(),
        })
    }

    // FIXME: Setting the cloned collection's name and icon is not yet implemented
    // Since they are currently committed to the repository
    // Updating them here would be a committable change
    pub(crate) async fn clone_collection(
        &self,
        ctx: &R::AsyncContext,
        id: &CollectionId,
        params: CollectionItemCloneParams,
    ) -> joinerror::Result<CollectionItemDescription> {
        let id_str = id.to_string();
        let abs_path: Arc<Path> = self.abs_path.join(id_str).into();
        if abs_path.exists() {
            return Err(joinerror::Error::new::<()>(format!(
                "collection directory `{}` already exists",
                abs_path.display()
            )));
        }

        self.fs
            .create_dir(&abs_path)
            .await
            .join_err_with::<()>(|| {
                format!("failed to create directory `{}`", abs_path.display())
            })?;

        let git_params = &params.git_params;
        let collection_result = CollectionBuilder::new(
            self.fs.clone(),
            self.github_client.clone(),
            self.gitlab_client.clone(),
        )
        .clone(
            ctx,
            CollectionCloneParams {
                internal_abs_path: abs_path.clone(),
                git_params: CollectionCloneGitParams {
                    git_provider_type: git_params.git_provider_type.clone(),
                    repository: git_params.repository.clone(),
                    branch: git_params.branch.clone(),
                },
            },
        )
        .await
        .join_err::<()>("failed to clone collection");

        // TODO: Use atomic-fs to rollback changes
        // Remove collection folder if collection fails to be cloned
        let collection = match collection_result {
            Ok(collection) => collection,
            Err(e) => {
                // TODO: Log or tell the frontend we failed to clean up after operation failure
                let _ = self
                    .fs
                    .remove_dir(
                        &abs_path,
                        RemoveOptions {
                            recursive: true,
                            ignore_if_not_exists: true,
                        },
                    )
                    .await
                    .join_err_with::<()>(|| {
                        format!("failed to remove directory `{}`", abs_path.display())
                    });
                return Err(e);
            }
        };

        let desc = collection.details().await?;

        // FIXME: Should we allow user to set local icon when cloning a collection?
        let icon_path = collection.icon_path();

        let mut state_lock = self.state.write().await;
        state_lock.expanded_items.insert(id.clone());
        state_lock.collections.insert(
            id.clone(),
            CollectionItem {
                id: id.clone(),
                order: Some(params.order),
                handle: Arc::new(collection),
            },
        );

        let mut txn = self
            .storage
            .begin_write(ctx)
            .await
            .join_err::<()>("failed to start transaction")?;

        self.storage
            .put_item_order_txn(ctx, &mut txn, &id, params.order)
            .await?;
        self.storage
            .put_expanded_items_txn(ctx, &mut txn, &state_lock.expanded_items)
            .await?;

        txn.commit()?;

        Ok(CollectionItemDescription {
            id: id.clone(),
            name: desc.name,
            order: Some(params.order),
            expanded: true,
            // FIXME: Rethink Manifest file and repository storage
            repository: desc.vcs.and_then(|vcs| vcs.url()),
            icon_path,
            abs_path,
            external_path: None,
        })
    }

    pub(crate) async fn delete_collection(
        &self,
        ctx: &R::AsyncContext,
        id: &CollectionId,
    ) -> joinerror::Result<Option<PathBuf>> {
        let id_str = id.to_string();
        let abs_path = self.abs_path.join(id_str);

        let mut state_lock = self.state.write().await;

        let item = state_lock.collections.remove(&id);
        let item_existed = item.is_some();

        if abs_path.exists() {
            if let Some(item) = item {
                item.dispose().await?;
            }
            self.fs
                .remove_dir(
                    &abs_path,
                    RemoveOptions {
                        recursive: true,
                        ignore_if_not_exists: true,
                    },
                )
                .await
                .join_err_with::<()>(|| {
                    format!("failed to remove directory `{}`", abs_path.display())
                })?;
        }

        state_lock.expanded_items.remove(&id);

        {
            let mut txn = self.storage.begin_write(ctx).await?;

            self.storage
                .remove_item_metadata_txn(ctx, &mut txn, SEGKEY_COLLECTION.join(&id.to_string()))
                .await?;
            self.storage
                .put_expanded_items_txn(ctx, &mut txn, &state_lock.expanded_items)
                .await?;

            txn.commit()?;
        }

        if item_existed {
            Ok(Some(abs_path))
        } else {
            Ok(None)
        }
    }

    pub(crate) async fn update_collection(
        &self,
        ctx: &R::AsyncContext,
        id: &CollectionId,
        params: UpdateCollectionParams,
    ) -> joinerror::Result<()> {
        let mut state_lock = self.state.write().await;
        let item = state_lock
            .collections
            .get_mut(&id)
            .ok_or_join_err_with::<()>(|| {
                format!("failed to find collection with id `{}`", id.to_string())
            })?;

        let mut txn = self.storage.begin_write(ctx).await?;
        if let Some(order) = params.order {
            item.order = Some(order);
            self.storage
                .put_item_order_txn(ctx, &mut txn, id, order)
                .await?;
        }

        // TODO: Implement relinking and unlinking remote repo when the user update it

        item.modify(CollectionModifyParams {
            name: params.name,
            repository: params.repository,
            icon_path: params.icon_path,
        })
        .await
        .join_err_with::<()>(|| {
            format!("failed to modify collection with id `{}`", id.to_string())
        })?;

        if let Some(expanded) = params.expanded {
            if expanded {
                state_lock.expanded_items.insert(id.to_owned());
            } else {
                state_lock.expanded_items.remove(id);
            }

            self.storage
                .put_expanded_items_txn(ctx, &mut txn, &state_lock.expanded_items)
                .await?;
        }

        txn.commit()?;

        Ok(())
    }

    pub(crate) async fn list_collections(
        &self,
        _ctx: &R::AsyncContext,
    ) -> Pin<Box<dyn Stream<Item = CollectionItemDescription> + Send + '_>> {
        let state = self.state.clone();

        Box::pin(async_stream::stream! {
            let state_lock = state.read().await;
            for (id, item) in state_lock.collections.iter() {
                let desc = item.details().await;
                if desc.is_err() {
                    // TODO: log error
                    println!("failed to parse collection {} manifest file", id.to_string());
                    continue;
                }
                let summary = desc.unwrap();

                let expanded = state_lock.expanded_items.contains(id);
                let icon_path = item.icon_path();

                yield CollectionItemDescription {
                    id: item.id.clone(),
                    name: summary.name,
                    order: item.order,
                    expanded,
                    repository: summary.vcs.map(|vcs| match vcs {
                        VcsSummary::GitHub { url, .. } => url,
                        VcsSummary::GitLab { url, .. } => url,
                    }),
                    icon_path,
                    abs_path: item.handle.abs_path().clone(),
                    external_path: None, // TODO: implement
                };
            }
        })
    }
}
async fn restore_collections<R: AppRuntime>(
    ctx: &R::AsyncContext,
    abs_path: &Path,
    fs: &Arc<dyn FileSystem>,
    storage: &Arc<StorageService<R>>,
    github_client: Arc<GitHubClient>,
    gitlab_client: Arc<GitLabClient>,
) -> joinerror::Result<HashMap<CollectionId, CollectionItem<R>>> {
    if !abs_path.exists() {
        return Ok(HashMap::new());
    }

    let mut collections = Vec::new();
    let mut read_dir = fs
        .read_dir(&abs_path)
        .await
        .join_err_with::<()>(|| format!("failed to read directory `{}`", abs_path.display()))?;
    while let Some(entry) = read_dir.next_entry().await? {
        if !entry.file_type().await?.is_dir() {
            continue;
        }

        let id_str = entry.file_name().to_string_lossy().to_string();
        let id: CollectionId = id_str.clone().into();

        let collection = {
            let collection_abs_path: Arc<Path> = entry.path().to_owned().into();

            let collection_result =
                CollectionBuilder::new(fs.clone(), github_client.clone(), gitlab_client.clone())
                    .load(CollectionLoadParams {
                        internal_abs_path: collection_abs_path,
                    })
                    .await;
            match collection_result {
                Ok(collection) => collection,
                Err(e) => {
                    // TODO: Let the frontend know a collection is invalid
                    println!("failed to rebuild collection `{}`: {}", id_str, e);
                    continue;
                }
            }
        };

        collections.push((id, collection));
    }

    let metadata = storage
        .list_items_metadata(ctx, SEGKEY_COLLECTION.to_segkey_buf())
        .await?;

    let mut result = HashMap::new();
    for (id, collection) in collections {
        let segkey_prefix = SEGKEY_COLLECTION.join(&id);

        let order = metadata
            .get(&segkey_prefix.join("order"))
            .and_then(|v| v.deserialize().ok());

        result.insert(
            id.clone(),
            CollectionItem {
                id,
                order,
                handle: Arc::new(collection),
            },
        );
    }

    Ok(result)
}
