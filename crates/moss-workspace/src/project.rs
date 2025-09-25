use crate::{
    builder::{OnDidAddProject, OnDidDeleteProject},
    dirs,
    models::{
        primitives::ProjectId,
        types::{
            CreateProjectGitParams, CreateProjectParams, EntryChange, ExportProjectParams,
            UpdateProjectParams,
        },
    },
    storage::{StorageService, segments::SEGKEY_COLLECTION},
};
use derive_more::{Deref, DerefMut};
use futures::Stream;
use joinerror::{Error, OptionExt, ResultExt};
use moss_app_delegate::{AppDelegate, broadcast::ToLocation};
use moss_applib::{AppRuntime, subscription::EventEmitter};
use moss_common::continue_if_err;
use moss_fs::{FileSystem, RemoveOptions, error::FsResultExt};
use moss_git::url::GitUrl;
use moss_git_hosting_provider::{
    GitProviderKind, github::client::GitHubApiClient, gitlab::client::GitLabApiClient,
};
use moss_logging::session;
use moss_project::{
    Project as ProjectHandle, ProjectBuilder, ProjectModifyParams,
    builder::{
        ProjectCloneParams, ProjectCreateGitParams, ProjectCreateParams,
        ProjectImportArchiveParams, ProjectImportExternalParams, ProjectLoadParams,
    },
    git::GitClient,
    vcs::VcsSummary,
};
use moss_user::{account::Account, models::primitives::AccountId, profile::Profile};
use rustc_hash::FxHashMap;
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    pin::Pin,
    sync::Arc,
};
use tokio::sync::RwLock;

// TODO: Rename all collections to projects

pub(crate) struct ProjectItemCloneParams {
    pub order: isize,
    pub account_id: AccountId,
    pub repository: String,
    pub git_provider_type: GitProviderKind,
    pub branch: Option<String>,
}

pub(crate) struct ProjectItemImportArchiveParams {
    pub name: String,
    pub order: isize,
    pub archive_path: PathBuf,
}

pub(crate) struct ProjectItemImportExternalParams {
    pub order: isize,
    pub external_path: PathBuf,
}

#[derive(Deref, DerefMut)]
struct ProjectItem<R: AppRuntime> {
    pub id: ProjectId,
    pub order: Option<isize>,

    #[deref]
    #[deref_mut]
    pub handle: Arc<ProjectHandle<R>>,
}

pub(crate) struct ProjectItemDescription {
    pub id: ProjectId,
    pub name: String,
    pub order: Option<isize>,
    pub expanded: bool,
    pub vcs: Option<VcsSummary>,
    // pub repository: Option<String>,

    // FIXME: Do we need this field?
    pub icon_path: Option<PathBuf>,
    pub internal_abs_path: Arc<Path>,
    pub external_path: Option<PathBuf>,
    pub archived: bool,
}

#[derive(Default)]
struct ServiceState<R: AppRuntime> {
    projects: HashMap<ProjectId, ProjectItem<R>>,
    expanded_items: HashSet<ProjectId>,
}

pub struct ProjectService<R: AppRuntime> {
    abs_path: PathBuf,
    fs: Arc<dyn FileSystem>,
    storage: Arc<StorageService<R>>,
    state: Arc<RwLock<ServiceState<R>>>,
    app_delegate: AppDelegate<R>,
    on_did_delete_project_emitter: EventEmitter<OnDidDeleteProject>,
    on_did_add_project_emitter: EventEmitter<OnDidAddProject>,
}

impl<R: AppRuntime> ProjectService<R> {
    pub(crate) async fn new(
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        abs_path: &Path,
        fs: Arc<dyn FileSystem>,
        storage: Arc<StorageService<R>>,
        environment_sources: &mut FxHashMap<Arc<String>, PathBuf>,
        active_profile: &Arc<Profile<R>>,
        on_project_did_delete_emitter: EventEmitter<OnDidDeleteProject>,
        on_project_did_add_emitter: EventEmitter<OnDidAddProject>,
    ) -> joinerror::Result<Self> {
        let abs_path = abs_path.join(dirs::PROJECTS_DIR);
        let expanded_items = if let Ok(expanded_items) = storage.get_expanded_items(ctx).await {
            expanded_items.into_iter().collect::<HashSet<_>>()
        } else {
            HashSet::new()
        };

        let collections =
            restore_collections(ctx, app_delegate, &abs_path, &fs, &storage, active_profile)
                .await
                .join_err_with::<()>(|| {
                    format!("failed to restore collections, {}", abs_path.display())
                })?;

        for (id, collection) in collections.iter() {
            environment_sources.insert(id.clone().inner(), collection.environments_path());
        }

        Ok(Self {
            abs_path,
            fs,
            storage,
            state: Arc::new(RwLock::new(ServiceState {
                projects: collections,
                expanded_items,
            })),
            app_delegate: app_delegate.clone(),
            on_did_delete_project_emitter: on_project_did_delete_emitter,
            on_did_add_project_emitter: on_project_did_add_emitter,
        })
    }

    pub async fn project(&self, id: &ProjectId) -> Option<Arc<ProjectHandle<R>>> {
        let state_lock = self.state.read().await;
        state_lock.projects.get(id).map(|item| item.handle.clone())
    }

    pub(crate) async fn create_project(
        &self,
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        id: &ProjectId,
        account: Option<Account<R>>,
        params: &CreateProjectParams,
    ) -> joinerror::Result<ProjectItemDescription> {
        let mut rb = self.fs.start_rollback().await?;

        let id_str = id.to_string();
        let abs_path: Arc<Path> = self.abs_path.join(id_str).into();
        if abs_path.exists() {
            return Err(Error::new::<()>(format!(
                "collection directory `{}` already exists",
                abs_path.display()
            )));
        }

        self.fs
            .create_dir_with_rollback(&mut rb, &abs_path)
            .await
            .join_err_with::<()>(|| {
                format!("failed to create directory `{}`", abs_path.display())
            })?;

        let git_params = match params.git_params.as_ref() {
            None => None,
            Some(CreateProjectGitParams::GitHub(git_params)) => {
                let repository = match GitUrl::parse(&git_params.repository) {
                    Ok(repository) => Some(repository),
                    Err(e) => {
                        // Continue creating a collection without vcs
                        self.app_delegate.emit_oneshot(ToLocation::Toast {
                            activity_id: "create_collection_invalid_repository",
                            title: "Invalid Repository".to_string(),
                            detail: Some(
                                "The provided repository is invalid, skipping the vcs".to_string(),
                            ),
                        })?;
                        session::error!(format!(
                            "failed to parse repository url: {}",
                            e.to_string()
                        ));
                        None
                    }
                };
                repository.map(|repository| ProjectCreateGitParams {
                    git_provider_type: GitProviderKind::GitHub,
                    repository,
                    branch: git_params.branch.clone(),
                })
            }
            Some(CreateProjectGitParams::GitLab(git_params)) => {
                let repository = match GitUrl::parse(&git_params.repository) {
                    Ok(repository) => Some(repository),
                    Err(e) => {
                        // Continue creating a collection without vcs
                        self.app_delegate.emit_oneshot(ToLocation::Toast {
                            activity_id: "create_collection_invalid_repository",
                            title: "Invalid Repository".to_string(),
                            detail: Some(
                                "The provided repository is invalid, skipping the vcs".to_string(),
                            ),
                        })?;
                        session::error!(format!(
                            "failed to parse repository url: {}",
                            e.to_string()
                        ));
                        None
                    }
                };
                repository.map(|repository| ProjectCreateGitParams {
                    git_provider_type: GitProviderKind::GitLab,
                    repository,
                    branch: git_params.branch.clone(),
                })
            }
        };

        let abs_path: Arc<Path> = abs_path.clone().into();
        let builder = ProjectBuilder::new(self.fs.clone()).await;

        let project = match builder
            .create(
                ctx,
                ProjectCreateParams {
                    name: Some(params.name.to_owned()),
                    internal_abs_path: abs_path.clone(),
                    external_abs_path: params.external_path.as_deref().map(|p| p.to_owned().into()),
                    git_params: git_params.clone(),
                    icon_path: params.icon_path.to_owned(),
                },
            )
            .await
            .join_err::<()>("failed to build collection")
        {
            Ok(collection) => collection,
            Err(e) => {
                let _ = rb.rollback().await.map_err(|e| {
                    session::warn!(format!("failed to rollback fs changes: {}", e.to_string()))
                });
                return Err(e);
            }
        };

        if let (Some(git_params), Some(account)) = (git_params, account) {
            let client = match git_params.git_provider_type {
                GitProviderKind::GitHub => GitClient::GitHub {
                    account: account,
                    api: <dyn GitHubApiClient<R>>::global(app_delegate),
                },
                GitProviderKind::GitLab => GitClient::GitLab {
                    account: account,
                    api: <dyn GitLabApiClient<R>>::global(app_delegate),
                },
            };

            if let Err(e) = project
                .init_vcs(ctx, client, git_params.repository, git_params.branch)
                .await
            {
                session::warn!(format!("failed to init vcs: {}", e.to_string()));
                self.app_delegate.emit_oneshot(ToLocation::Toast {
                    activity_id: "create_collection_init_vcs_failure",
                    title: "Failed to initialized collection vcs".to_string(),
                    detail: Some(
                        "Failed to initialize collection vcs, creating a local only collection"
                            .to_string(),
                    ),
                })?;
            }
        }

        let icon_path = project.icon_path();

        {
            let mut state_lock = self.state.write().await;
            state_lock.expanded_items.insert(id.to_owned());
            state_lock.projects.insert(
                id.to_owned(),
                ProjectItem {
                    id: id.to_owned(),
                    order: Some(params.order),
                    handle: Arc::new(project),
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

        self.on_did_add_project_emitter
            .fire(OnDidAddProject {
                project_id: id.clone(),
            })
            .await;

        Ok(ProjectItemDescription {
            id: id.to_owned(),
            name: params.name.clone(),
            order: Some(params.order),
            expanded: true,
            vcs: None,
            icon_path,
            internal_abs_path: abs_path.into(),
            external_path: params.external_path.clone(),
            archived: false,
        })
    }

    // TODO: Setting the cloned collection's name and icon is not yet implemented
    // Since they are currently committed to the repository
    // Updating them here would be a committable change
    pub(crate) async fn clone_project(
        &self,
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        id: &ProjectId,
        account: Account<R>,
        params: ProjectItemCloneParams,
    ) -> joinerror::Result<ProjectItemDescription> {
        let mut rb = self.fs.start_rollback().await?;

        let id_str = id.to_string();
        let abs_path: Arc<Path> = self.abs_path.join(id_str).into();
        if abs_path.exists() {
            return Err(Error::new::<()>(format!(
                "collection directory `{}` already exists",
                abs_path.display()
            )));
        }

        self.fs
            .create_dir_with_rollback(&mut rb, &abs_path)
            .await
            .join_err_with::<()>(|| {
                format!("failed to create directory `{}`", abs_path.display())
            })?;

        let builder = ProjectBuilder::new(self.fs.clone()).await;

        let repository = match GitUrl::parse(&params.repository) {
            Ok(repository) => repository,
            Err(e) => {
                self.app_delegate.emit_oneshot(ToLocation::Toast {
                    activity_id: "clone_collection_invalid_repository",
                    title: "Invalid repository url".to_string(),
                    detail: Some(
                        "Cannot clone remote collection since the url is invalid".to_string(),
                    ),
                })?;

                let _ = rb
                    .rollback()
                    .await
                    .map_err(|e| session::warn!(format!("failed to rollback: {}", e.to_string())));
                return Err(e);
            }
        };

        let git_client = match params.git_provider_type {
            GitProviderKind::GitHub => GitClient::GitHub {
                account: account,
                api: <dyn GitHubApiClient<R>>::global(app_delegate),
            },
            GitProviderKind::GitLab => GitClient::GitLab {
                account: account,
                api: <dyn GitLabApiClient<R>>::global(app_delegate),
            },
        };
        let collection = match builder
            .clone(
                ctx,
                git_client,
                ProjectCloneParams {
                    internal_abs_path: abs_path.clone(),
                    account_id: params.account_id,
                    git_provider_type: params.git_provider_type.clone(),
                    repository,
                    branch: params.branch.clone(),
                },
            )
            .await
            .join_err::<()>("failed to clone collection")
        {
            Ok(collection) => collection,
            Err(e) => {
                let _ = rb.rollback().await.map_err(|e| {
                    session::warn!(format!("failed to rollback fs changes: {}", e.to_string()))
                });
                return Err(e);
            }
        };

        let desc = collection.details().await?;
        let vcs = collection
            .vcs()
            .unwrap() // SAFETY: Collection is built from the clone operation, so it must have a VCS
            .summary(ctx)
            .await?;

        // FIXME: Should we allow user to set local icon when cloning a collection?
        let icon_path = collection.icon_path();

        {
            let mut state_lock = self.state.write().await;
            state_lock.expanded_items.insert(id.clone());
            state_lock.projects.insert(
                id.clone(),
                ProjectItem {
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

        self.on_did_add_project_emitter
            .fire(OnDidAddProject {
                project_id: id.clone(),
            })
            .await;

        Ok(ProjectItemDescription {
            id: id.clone(),
            name: desc.name,
            order: Some(params.order),
            expanded: true,
            vcs: Some(vcs),
            icon_path,
            internal_abs_path: abs_path,
            external_path: None,
            archived: false,
        })
    }

    pub(crate) async fn delete_project(
        &self,
        ctx: &R::AsyncContext,
        id: &ProjectId,
    ) -> joinerror::Result<Option<PathBuf>> {
        let id_str = id.to_string();
        let abs_path = self.abs_path.join(id_str);

        let mut state_lock = self.state.write().await;

        let item = state_lock.projects.remove(&id);
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

        self.on_did_delete_project_emitter
            .fire(OnDidDeleteProject {
                project_id: id.to_owned(),
            })
            .await;

        if item_existed {
            Ok(Some(abs_path))
        } else {
            Ok(None)
        }
    }

    pub(crate) async fn update_project(
        &self,
        ctx: &R::AsyncContext,
        id: &ProjectId,
        params: UpdateProjectParams,
    ) -> joinerror::Result<()> {
        let mut state_lock = self.state.write().await;
        let item = state_lock
            .projects
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

        item.modify(ProjectModifyParams {
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

    pub(crate) async fn list_projects(
        &self,
        ctx: &R::AsyncContext,
    ) -> Pin<Box<dyn Stream<Item = ProjectItemDescription> + Send + '_>> {
        let state_clone = self.state.clone();
        let ctx_clone = ctx.clone();

        Box::pin(async_stream::stream! {
            let state_lock = state_clone.read().await;
            for (id, item) in state_lock.projects.iter() {
                let details = continue_if_err!(item.details().await, |e: Error| {
                    session::error!(format!("failed to describe collection `{}`: {}", id.to_string(), e.to_string()));
                });

                let vcs = if let Some(vcs) = item.vcs() {
                    match vcs.summary(&ctx_clone).await {
                        Ok(summary) => Some(summary),
                        Err(e) => {
                            session::warn!(format!("failed to get VCS summary for collection `{}`: {}", id.to_string(), e.to_string()));
                            None
                        }
                    }
                } else { None };

                let expanded = state_lock.expanded_items.contains(id);
                let icon_path = item.icon_path();

                yield ProjectItemDescription {
                    id: item.id.clone(),
                    name: details.name,
                    order: item.order,
                    expanded,
                    vcs,
                    icon_path,
                    internal_abs_path: item.handle.internal_abs_path().clone(),
                    external_path: item.handle.external_abs_path().map(|p| p.to_path_buf()),
                    archived: item.is_archived(),
                };
            }
        })
    }

    pub(crate) async fn archive_project(
        &self,
        _ctx: &R::AsyncContext,
        id: &ProjectId,
    ) -> joinerror::Result<()> {
        let mut state_lock = self.state.write().await;
        let item = state_lock
            .projects
            .get_mut(&id)
            .ok_or_join_err_with::<()>(|| {
                format!("failed to find collection with id `{}`", id.to_string())
            })?;

        item.archive().await
    }

    pub(crate) async fn unarchive_project(
        &self,
        _ctx: &R::AsyncContext,
        id: &ProjectId,
    ) -> joinerror::Result<()> {
        let mut state_lock = self.state.write().await;
        let item = state_lock
            .projects
            .get_mut(&id)
            .ok_or_join_err_with::<()>(|| {
                format!("failed to find collection with id `{}`", id.to_string())
            })?;

        item.unarchive().await
    }

    pub(crate) async fn import_archived_project(
        &self,
        ctx: &R::AsyncContext,
        id: &ProjectId,
        params: ProjectItemImportArchiveParams,
    ) -> joinerror::Result<ProjectItemDescription> {
        let mut rb = self.fs.start_rollback().await?;

        let id_str = id.to_string();
        let abs_path: Arc<Path> = self.abs_path.join(&id_str).into();
        if abs_path.exists() {
            return Err(Error::new::<()>(format!(
                "collection directory `{}` already exists",
                abs_path.display()
            )));
        }

        self.fs
            .create_dir_with_rollback(&mut rb, &abs_path)
            .await
            .join_err_with::<()>(|| {
                format!("failed to create directory `{}`", abs_path.display())
            })?;

        let builder = ProjectBuilder::new(self.fs.clone()).await;

        let collection = match builder
            .import_archive(
                ctx,
                ProjectImportArchiveParams {
                    internal_abs_path: abs_path.clone(),
                    archive_path: params.archive_path.into(),
                },
            )
            .await
            .join_err::<()>("failed to import collection from archive file")
        {
            Ok(collection) => collection,
            Err(e) => {
                let _ = rb.rollback().await.map_err(|e| {
                    session::warn!(format!("failed to rollback fs changes: {}", e.to_string()))
                });
                return Err(e);
            }
        };

        // Update the collection name based on user input
        if let Err(e) = collection
            .modify(ProjectModifyParams {
                name: Some(params.name),
                repository: None,
                icon_path: None,
            })
            .await
        {
            let _ = rb.rollback().await.map_err(|e| {
                session::warn!(format!("failed to rollback fs changes: {}", e.to_string()))
            });
            return Err(e);
        }

        let desc = collection.details().await?;

        let icon_path = collection.icon_path();
        {
            let mut state_lock = self.state.write().await;
            state_lock.expanded_items.insert(id.clone());
            state_lock.projects.insert(
                id.clone(),
                ProjectItem {
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

        self.on_did_add_project_emitter
            .fire(OnDidAddProject {
                project_id: id.clone(),
            })
            .await;

        Ok(ProjectItemDescription {
            id: id.clone(),
            name: desc.name,
            order: Some(params.order),
            expanded: true,
            vcs: None,
            icon_path,
            internal_abs_path: abs_path,
            external_path: None,
            archived: false,
        })
    }

    pub(crate) async fn import_external_project(
        &self,
        ctx: &R::AsyncContext,
        id: &ProjectId,
        params: ProjectItemImportExternalParams,
    ) -> joinerror::Result<ProjectItemDescription> {
        let mut rb = self.fs.start_rollback().await?;

        let id_str = id.to_string();
        let internal_abs_path: Arc<Path> = self.abs_path.join(&id_str).into();
        if internal_abs_path.exists() {
            return Err(Error::new::<()>(format!(
                "collection directory `{}` already exists",
                internal_abs_path.display()
            )));
        }

        self.fs
            .create_dir_with_rollback(&mut rb, &internal_abs_path)
            .await
            .join_err_with::<()>(|| {
                format!(
                    "failed to create directory `{}`",
                    internal_abs_path.display()
                )
            })?;

        let builder = ProjectBuilder::new(self.fs.clone()).await;
        let project = match builder
            .import_external(
                ctx,
                ProjectImportExternalParams {
                    internal_abs_path: internal_abs_path.clone(),
                    external_abs_path: params.external_path.clone().into(),
                },
            )
            .await
            .join_err::<()>("failed to import external project")
        {
            Ok(project) => project,
            Err(e) => {
                let _ = rb.rollback().await.map_err(|e| {
                    session::warn!(format!("failed to rollback fs changes: {}", e.to_string()))
                });
                return Err(e);
            }
        };

        let icon_path = project.icon_path();
        let name = project.details().await?.name;
        let vcs_summary = if let Some(vcs) = project.vcs() {
            match vcs.summary(ctx).await {
                Ok(summary) => Some(summary),
                Err(e) => {
                    session::error!(format!("failed to get vcs summary: {}", e));
                    None
                }
            }
        } else {
            None
        };

        {
            let mut state_lock = self.state.write().await;
            state_lock.expanded_items.insert(id.to_owned());
            state_lock.projects.insert(
                id.to_owned(),
                ProjectItem {
                    id: id.to_owned(),
                    order: Some(params.order),
                    handle: Arc::new(project),
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

        self.on_did_add_project_emitter
            .fire(OnDidAddProject {
                project_id: id.clone(),
            })
            .await;

        Ok(ProjectItemDescription {
            id: id.to_owned(),
            name,
            order: Some(params.order),
            expanded: true,
            vcs: vcs_summary,
            icon_path,
            internal_abs_path,
            external_path: Some(params.external_path),
            archived: false,
        })
    }

    pub(crate) async fn export_collection(
        &self,
        id: &ProjectId,
        params: &ExportProjectParams,
    ) -> joinerror::Result<PathBuf> {
        let state_lock = self.state.read().await;
        let item = state_lock.projects.get(&id).ok_or_join_err_with::<()>(|| {
            format!("failed to find collection with id `{}`", id.to_string())
        })?;

        item.export_archive(&params.destination).await
    }

    /// List file statuses for all collections that have a repository handle
    pub(crate) async fn list_changes(&self) -> joinerror::Result<Vec<EntryChange>> {
        let mut changes: Vec<EntryChange> = Vec::new();

        let state_lock = self.state.read().await;
        for (id, item) in state_lock.projects.iter() {
            let vcs = if let Some(vcs) = item.vcs() {
                vcs
            } else {
                continue;
            };

            let statuses_result = vcs.statuses().await;
            if let Err(e) = statuses_result {
                session::warn!(format!(
                    "failed to get file statuses for collection `{}`: {}",
                    id,
                    e.to_string()
                ));
                let _ = self.app_delegate.emit_oneshot(ToLocation::Toast {
                    activity_id: "get_file_statuses_error",
                    title: format!("Failed to get file statuses for collection `{}`", id),
                    detail: Some(e.to_string()),
                });
                continue;
            }

            for (path, status) in statuses_result? {
                changes.push(EntryChange {
                    project_id: id.clone(),
                    path,
                    status,
                })
            }
        }

        Ok(changes)
    }
}
async fn restore_collections<R: AppRuntime>(
    ctx: &R::AsyncContext,
    app_delegate: &AppDelegate<R>,
    abs_path: &Path,
    fs: &Arc<dyn FileSystem>,
    storage: &Arc<StorageService<R>>,
    active_profile: &Arc<Profile<R>>,
) -> joinerror::Result<HashMap<ProjectId, ProjectItem<R>>> {
    if !abs_path.exists() {
        return Ok(HashMap::new());
    }

    let mut collections = Vec::new();
    let mut read_dir = fs
        .read_dir(&abs_path)
        .await
        .join_err_with::<()>(|| format!("failed to read directory `{}`", abs_path.display()))?;

    let activity_handle = app_delegate.emit_continual(ToLocation::Window {
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
        let id: ProjectId = id_str.clone().into();

        let collection = {
            let collection_abs_path: Arc<Path> = entry.path().to_owned().into();
            let builder = ProjectBuilder::new(fs.clone()).await;

            let collection_result = builder
                .load(ProjectLoadParams {
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

        if collection.is_archived() {
            collections.push((id, collection));
            continue;
        }
        // Only load the vcs if the collection is not archived

        let details = match collection.details().await {
            Ok(details) => details,
            Err(e) => {
                app_delegate.emit_oneshot(ToLocation::Toast {
                    activity_id: "restore_collections_failed_to_get_details",
                    title: "Failed to get collection details".to_string(),
                    detail: Some(format!(
                        "Failed to get collection details: {}, it will be skipped.",
                        e.to_string()
                    )),
                })?;
                continue;
            }
        };

        if let (Some(vcs), Some(account_id)) = (details.vcs, details.account_id) {
            // FIXME: Skip initializing vcs instead of failing the restore process
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
                    api: <dyn GitHubApiClient<R>>::global(app_delegate),
                },
                GitProviderKind::GitLab => GitClient::GitLab {
                    account,
                    api: <dyn GitLabApiClient<R>>::global(app_delegate),
                },
            };

            collection.load_vcs(client).await?;
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
            ProjectItem {
                id,
                order,
                handle: Arc::new(collection),
            },
        );
    }

    activity_handle.emit_finish()?;

    Ok(result)
}
