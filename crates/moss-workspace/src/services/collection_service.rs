use derive_more::{Deref, DerefMut};
use futures::Stream;
use joinerror::{Error, OptionExt, ResultExt};
use moss_activity_broadcaster::{ActivityBroadcaster, ToLocation};
use moss_applib::{AppHandle, AppRuntime, subscription::EventEmitter};
use moss_collection::{
    Collection as CollectionHandle, CollectionBuilder, CollectionModifyParams,
    builder::{
        CollectionCloneParams, CollectionCreateGitParams, CollectionCreateParams,
        CollectionLoadParams,
    },
    git::GitClient,
    vcs::VcsSummary,
};
use moss_common::continue_if_err;
use moss_fs::{FileSystem, RemoveOptions, error::FsResultExt};
use moss_git::url::GitUrl;
use moss_git_hosting_provider::{
    GitProviderKind, github::GitHubApiClient, gitlab::GitLabApiClient,
};
use moss_logging::session;
use moss_user::{account::Account, models::primitives::AccountId, profile::ActiveProfile};
use reqwest::Client as HttpClient;
use rustc_hash::FxHashMap;
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    pin::Pin,
    sync::Arc,
};
use tokio::sync::RwLock;

use crate::{
    builder::{OnDidAddCollection, OnDidDeleteCollection},
    dirs,
    models::{
        primitives::CollectionId,
        types::{CreateCollectionGitParams, CreateCollectionParams, UpdateCollectionParams},
    },
    services::storage_service::StorageService,
    storage::segments::SEGKEY_COLLECTION,
};

pub(crate) struct CollectionItemCloneParams {
    pub order: isize,
    pub account_id: AccountId,
    pub repository: String,
    pub git_provider_type: GitProviderKind,
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
    pub vcs: Option<VcsSummary>,
    // pub repository: Option<String>,

    // FIXME: Do we need this field?
    pub icon_path: Option<PathBuf>,
    pub abs_path: Arc<Path>,
    pub external_path: Option<PathBuf>,
    pub archived: bool,
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
    #[allow(dead_code)]
    broadcaster: ActivityBroadcaster<R::EventLoop>,
    on_did_delete_collection_emitter: EventEmitter<OnDidDeleteCollection>,
    on_did_add_collection_emitter: EventEmitter<OnDidAddCollection>,
}

impl<R: AppRuntime> CollectionService<R> {
    pub(crate) async fn new(
        ctx: &R::AsyncContext,
        abs_path: &Path,
        fs: Arc<dyn FileSystem>,
        storage: Arc<StorageService<R>>,
        environment_sources: &mut FxHashMap<Arc<String>, PathBuf>,
        broadcaster: ActivityBroadcaster<R::EventLoop>,
        active_profile: &Arc<ActiveProfile>,
        on_collection_did_delete_emitter: EventEmitter<OnDidDeleteCollection>,
        on_collection_did_add_emitter: EventEmitter<OnDidAddCollection>,
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
            broadcaster.clone(),
            active_profile,
        )
        .await
        .join_err_with::<()>(|| format!("failed to restore collections, {}", abs_path.display()))?;

        for (id, collection) in collections.iter() {
            environment_sources.insert(id.clone().inner(), collection.environments_path());
        }

        Ok(Self {
            abs_path,
            fs,
            storage,
            state: Arc::new(RwLock::new(ServiceState {
                collections,
                expanded_items,
            })),
            broadcaster,
            on_did_delete_collection_emitter: on_collection_did_delete_emitter,
            on_did_add_collection_emitter: on_collection_did_add_emitter,
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
        account: Option<Account>,
        params: &CreateCollectionParams,
    ) -> joinerror::Result<CollectionItemDescription> {
        let id_str = id.to_string();
        let abs_path: Arc<Path> = self.abs_path.join(id_str).into();
        if abs_path.exists() {
            return Err(Error::new::<()>(format!(
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
                    git_provider_type: GitProviderKind::GitHub,
                    repository: GitUrl::parse(&git_params.repository)?,
                    branch: git_params.branch.clone(),
                })
            }
            Some(CreateCollectionGitParams::GitLab(git_params)) => {
                Some(CollectionCreateGitParams {
                    git_provider_type: GitProviderKind::GitLab,
                    repository: GitUrl::parse(&git_params.repository)?,
                    branch: git_params.branch.clone(),
                })
            }
        };

        let abs_path: Arc<Path> = abs_path.clone().into();
        let builder =
            CollectionBuilder::<R>::new(self.fs.clone(), self.broadcaster.clone()).await?;

        let collection_result = builder
            .create(
                ctx,
                CollectionCreateParams {
                    name: Some(params.name.to_owned()),
                    internal_abs_path: abs_path.clone(),
                    external_abs_path: params.external_path.as_deref().map(|p| p.to_owned().into()),
                    git_params: git_params.clone(),
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

        if let (Some(git_params), Some(account)) = (git_params, account) {
            let client = match git_params.git_provider_type {
                GitProviderKind::GitHub => GitClient::GitHub {
                    account: account,
                    api: GitHubApiClient::new(HttpClient::new()), // FIXME:
                },
                GitProviderKind::GitLab => GitClient::GitLab {
                    account: account,
                    api: GitLabApiClient::new(HttpClient::new()), // FIXME:
                },
            };

            collection
                .init_vcs(client, git_params.repository, git_params.branch)
                .await?;
        }

        let icon_path = collection.icon_path();

        {
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
        }

        {
            let state_lock = self.state.read().await;

            // TODO: Make database errors not fail the operation

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

        self.on_did_add_collection_emitter
            .fire(OnDidAddCollection {
                collection_id: id.clone(),
            })
            .await;

        Ok(CollectionItemDescription {
            id: id.to_owned(),
            name: params.name.clone(),
            order: Some(params.order),
            expanded: true,
            vcs: None,
            icon_path,
            abs_path: abs_path.into(),
            external_path: params.external_path.clone(),
            archived: false,
        })
    }

    // FIXME: Setting the cloned collection's name and icon is not yet implemented
    // Since they are currently committed to the repository
    // Updating them here would be a committable change
    pub(crate) async fn clone_collection(
        &self,
        ctx: &R::AsyncContext,
        app_handle: &AppHandle<R>,
        id: &CollectionId,
        account: Account,
        params: CollectionItemCloneParams,
    ) -> joinerror::Result<CollectionItemDescription> {
        let id_str = id.to_string();
        let abs_path: Arc<Path> = self.abs_path.join(id_str).into();
        if abs_path.exists() {
            return Err(Error::new::<()>(format!(
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

        let builder = CollectionBuilder::new(self.fs.clone(), self.broadcaster.clone()).await?;

        let git_client = match params.git_provider_type {
            GitProviderKind::GitHub => GitClient::GitHub {
                account: account,
                api: app_handle.global::<GitHubApiClient>().clone(),
            },
            GitProviderKind::GitLab => GitClient::GitLab {
                account: account,
                api: app_handle.global::<GitLabApiClient>().clone(),
            },
        };
        let collection_result = builder
            .clone(
                ctx,
                git_client,
                CollectionCloneParams {
                    internal_abs_path: abs_path.clone(),
                    account_id: params.account_id,
                    git_provider_type: params.git_provider_type.clone(),
                    repository: GitUrl::parse(&params.repository)?,
                    branch: params.branch.clone(),
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
        let vcs = collection
            .vcs()
            .unwrap() // SAFETY: Collection is built from the clone operation, so it must have a VCS
            .summary()
            .await?;

        // FIXME: Should we allow user to set local icon when cloning a collection?
        let icon_path = collection.icon_path();

        {
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
            // TODO: Make database errors not fail the operation
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
        }

        self.on_did_add_collection_emitter
            .fire(OnDidAddCollection {
                collection_id: id.clone(),
            })
            .await;

        // TODO: Add account info to the config

        Ok(CollectionItemDescription {
            id: id.clone(),
            name: desc.name,
            order: Some(params.order),
            expanded: true,
            // FIXME: Rethink Manifest file and repository storage
            vcs: Some(vcs),
            icon_path,
            abs_path,
            external_path: None,
            archived: false,
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
            // TODO: Make database errors not fail the operation
            let mut txn = self.storage.begin_write(ctx).await?;

            self.storage
                .remove_item_metadata_txn(ctx, &mut txn, SEGKEY_COLLECTION.join(&id.to_string()))
                .await?;
            self.storage
                .put_expanded_items_txn(ctx, &mut txn, &state_lock.expanded_items)
                .await?;

            txn.commit()?;
        }

        self.on_did_delete_collection_emitter
            .fire(OnDidDeleteCollection {
                collection_id: id.to_owned(),
            })
            .await;

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

        // TODO: Make database errors not fail the operation
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
                let details = continue_if_err!(item.details().await, |e: Error| {
                    session::error!(format!("failed to describe collection `{}`: {}", id.to_string(), e.to_string()));
                });

                let vcs = if let Some(vcs) = item.vcs() {
                    match vcs.summary().await {
                        Ok(summary) => Some(summary),
                        Err(e) => {
                            session::warn!(format!("failed to get VCS summary for collection `{}`: {}", id.to_string(), e.to_string()));
                            None
                        }
                    }
                } else { None };

                let expanded = state_lock.expanded_items.contains(id);
                let icon_path = item.icon_path();

                yield CollectionItemDescription {
                    id: item.id.clone(),
                    name: details.name,
                    order: item.order,
                    expanded,
                    vcs,
                    icon_path,
                    abs_path: item.handle.abs_path().clone(),
                    external_path: None, // TODO: implement
                    archived: item.is_archived(),
                };
            }
        })
    }

    pub(crate) async fn archive_collection(
        &self,
        _ctx: &R::AsyncContext,
        id: &CollectionId,
    ) -> joinerror::Result<()> {
        let mut state_lock = self.state.write().await;
        let item = state_lock
            .collections
            .get_mut(&id)
            .ok_or_join_err_with::<()>(|| {
                format!("failed to find collection with id `{}`", id.to_string())
            })?;

        item.archive().await
    }

    pub(crate) async fn unarchive_collection(
        &self,
        _ctx: &R::AsyncContext,
        id: &CollectionId,
    ) -> joinerror::Result<()> {
        let mut state_lock = self.state.write().await;
        let item = state_lock
            .collections
            .get_mut(&id)
            .ok_or_join_err_with::<()>(|| {
                format!("failed to find collection with id `{}`", id.to_string())
            })?;

        item.unarchive().await
    }
}
async fn restore_collections<R: AppRuntime>(
    ctx: &R::AsyncContext,
    abs_path: &Path,
    fs: &Arc<dyn FileSystem>,
    storage: &Arc<StorageService<R>>,
    broadcaster: ActivityBroadcaster<R::EventLoop>,
    active_profile: &Arc<ActiveProfile>,
) -> joinerror::Result<HashMap<CollectionId, CollectionItem<R>>> {
    if !abs_path.exists() {
        return Ok(HashMap::new());
    }

    let mut collections = Vec::new();
    let mut read_dir = fs
        .read_dir(&abs_path)
        .await
        .join_err_with::<()>(|| format!("failed to read directory `{}`", abs_path.display()))?;

    let activity_handle = broadcaster.emit_continual(ToLocation::Window {
        activity_id: "restore_collections",
        title: "Restoring collections".to_string(),
        detail: None,
    })?;

    while let Some(entry) = read_dir.next_entry().await? {
        if !entry.file_type().await?.is_dir() {
            continue;
        }

        activity_handle.emit_progress(Some(format!(
            "Restoring collection `{}`",
            entry.file_name().to_string_lossy()
        )))?;

        let id_str = entry.file_name().to_string_lossy().to_string();
        let id: CollectionId = id_str.clone().into();

        let collection = {
            let collection_abs_path: Arc<Path> = entry.path().to_owned().into();
            let builder = CollectionBuilder::<R>::new(fs.clone(), broadcaster.clone())
                .await
                .join_err_with::<()>(|| {
                    format!(
                        "failed to rebuild collection `{}`, {}",
                        id_str,
                        collection_abs_path.display()
                    )
                })?;

            let collection_result = builder
                .load(CollectionLoadParams {
                    internal_abs_path: collection_abs_path,
                })
                .await;
            match collection_result {
                Ok(collection) => collection,
                Err(e) => {
                    // TODO: Let the frontend know a collection is invalid
                    session::error!(format!(
                        "failed to rebuild collection `{}`: {}",
                        id_str,
                        e.to_string()
                    ));
                    continue;
                }
            }
        };

        // Only load the vcs if the collection is not archived
        if !collection.is_archived() {
            let vcs = collection.details().await?.vcs;
            let account_id = collection.config().await?.account_id;

            if let (Some(vcs), Some(account_id)) = (vcs, account_id) {
                let account = active_profile
                    .account(&account_id)
                    .await
                    .ok_or_join_err_with::<()>(|| {
                        format!(
                            "failed to find account with id `{}`",
                            account_id.to_string()
                        )
                    })?;

                let client = match vcs.kind {
                    GitProviderKind::GitHub => GitClient::GitHub {
                        account,
                        api: GitHubApiClient::new(HttpClient::new()),
                    },
                    GitProviderKind::GitLab => GitClient::GitLab {
                        account,
                        api: GitLabApiClient::new(HttpClient::new()),
                    },
                };

                collection.load_vcs(client).await?;
            }
        }

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

    activity_handle.emit_finish()?;

    Ok(result)
}
