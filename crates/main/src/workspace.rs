use async_trait::async_trait;
use joinerror::{OptionExt, ResultExt};
use moss_bindingutils::primitives::ChangeJsonValue;
use moss_common::continue_if_err;
use moss_environment::{
    AnyEnvironment, Environment,
    builder::{CreateEnvironmentParams, EnvironmentBuilder, EnvironmentLoadParams},
    storage::{key_variable, key_variable_local_value},
};
use moss_fs::FileSystem;
use moss_git::url::GitUrl;
use moss_project::{
    Project, ProjectBuilder,
    builder::{
        ProjectCloneParams, ProjectCreateParams, ProjectImportArchiveParams,
        ProjectImportExternalParams, ProjectLoadParams,
    },
    git::GitClient,
};
use moss_storage2::{KvStorage, models::primitives::StorageScope};
use moss_workspace::storage::{KEY_ACTIVE_ENVIRONMENTS, key_project};
use rustc_hash::{FxHashMap, FxHashSet};
use sapic_base::{
    environment::types::primitives::{EnvironmentId, VariableId},
    other::GitProviderKind,
    project::types::primitives::ProjectId,
    user::types::primitives::AccountId,
    workspace::types::primitives::WorkspaceId,
};
use sapic_core::context::AnyAsyncContext;
use sapic_ipc::contracts::main::{
    environment::{
        BatchUpdateEnvironmentInput, CreateEnvironmentInput, CreateEnvironmentOutput,
        DeleteEnvironmentInput, EnvironmentGroup, StreamEnvironmentsEvent,
        StreamEnvironmentsOutput, UpdateEnvironmentGroupParams, UpdateEnvironmentInput,
        UpdateEnvironmentOutput, UpdateEnvironmentParams,
    },
    project::{
        CreateProjectParams, ExportProjectParams, ImportArchiveParams, ImportDiskParams,
        UpdateProjectParams,
    },
};
use sapic_platform::{
    environment::environment_edit_backend::EnvironmentFsEditBackend,
    project::project_edit_backend::ProjectFsEditBackend,
};
use sapic_system::{
    environment::{
        EnvironmentEditParams, EnvironmentItemDescription,
        environment_edit_service::EnvironmentEditService,
        environment_service::{CreateEnvironmentItemParams, EnvironmentService},
    },
    ports::{github_api::GitHubApiClient, gitlab_api::GitLabApiClient},
    project::{
        CreateProjectGitParams, ProjectConfigEditParams, ProjectEditParams,
        project_edit_service::ProjectEditService, project_service::ProjectService,
    },
    user::User,
    workspace::{WorkspaceEditOp, WorkspaceEditParams},
};
use std::{collections::HashSet, path::PathBuf, sync::Arc};
use tokio::sync::{OnceCell, RwLock};

use crate::environment::RuntimeEnvironment;

pub(crate) const GLOBAL_ACTIVE_ENVIRONMENT_KEY: &'static str = "";

#[derive(Clone)]
pub struct RuntimeProject {
    pub id: ProjectId,
    pub handle: Arc<Project>,

    pub(crate) edit: ProjectEditService,

    pub order: Option<isize>,
}
#[async_trait]
pub trait Workspace: Send + Sync {
    fn id(&self) -> WorkspaceId;
    fn abs_path(&self) -> PathBuf;

    async fn dispose(&self) -> joinerror::Result<()>;
    async fn edit(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: WorkspaceEditParams,
    ) -> joinerror::Result<()>;

    // Project
    async fn create_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: CreateProjectParams,
    ) -> joinerror::Result<RuntimeProject>;

    async fn clone_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        account_id: &AccountId,
        git_provider_kind: GitProviderKind,
        repository: &str,
        branch: Option<String>,
    ) -> joinerror::Result<RuntimeProject>;

    async fn import_archived_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: &ImportArchiveParams,
    ) -> joinerror::Result<RuntimeProject>;

    async fn import_external_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: &ImportDiskParams,
    ) -> joinerror::Result<RuntimeProject>;

    async fn delete_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
    ) -> joinerror::Result<Option<PathBuf>>;

    async fn update_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: UpdateProjectParams,
    ) -> joinerror::Result<()>;

    async fn archive_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
    ) -> joinerror::Result<()>;

    async fn unarchive_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
    ) -> joinerror::Result<()>;

    async fn export_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: ExportProjectParams,
    ) -> joinerror::Result<PathBuf>;

    async fn project(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
    ) -> joinerror::Result<RuntimeProject>;

    async fn projects(&self, ctx: &dyn AnyAsyncContext) -> joinerror::Result<Vec<RuntimeProject>>;

    // Environment
    async fn activate_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &EnvironmentId,
    ) -> joinerror::Result<()>;

    async fn create_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        input: CreateEnvironmentInput,
    ) -> joinerror::Result<EnvironmentItemDescription>;

    async fn delete_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &EnvironmentId,
    ) -> joinerror::Result<()>;

    async fn update_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: UpdateEnvironmentParams,
    ) -> joinerror::Result<()>;

    async fn batch_update_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        updates: Vec<UpdateEnvironmentParams>,
    ) -> joinerror::Result<()>;

    // I didn't migrate them since they seem to be only about updating the expand flag and order of env groups

    // async fn update_environment_group(
    //     &self,
    //     ctx: &dyn AnyAsyncContext,
    //     params: UpdateEnvironmentGroupParams,
    // ) -> joinerror::Result<()>;
    //
    // async fn batch_update_environment_groups(
    //     &self,
    //     ctx: &dyn AnyAsyncContext,
    //     updates: Vec<UpdateEnvironmentGroupParams>
    // ) -> joinerror::Result<()>;

    async fn environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &EnvironmentId,
    ) -> joinerror::Result<RuntimeEnvironment>;

    // FIXME: For now I have kept the old behavior of streaming both workspace and project environments for consistency
    // We will revisit this later
    async fn environments(
        &self,
        ctx: &dyn AnyAsyncContext,
    ) -> joinerror::Result<Vec<RuntimeEnvironment>>;

    async fn active_environments(
        &self,
        ctx: &dyn AnyAsyncContext,
    ) -> joinerror::Result<FxHashMap<Arc<String>, EnvironmentId>>;

    async fn environment_groups(
        &self,
        ctx: &dyn AnyAsyncContext,
    ) -> joinerror::Result<FxHashSet<ProjectId>>;
}

pub struct RuntimeWorkspace {
    id: WorkspaceId,
    abs_path: PathBuf,
    fs: Arc<dyn FileSystem>,
    storage: Arc<dyn KvStorage>,
    edit: Arc<dyn WorkspaceEditOp>,

    user: Arc<dyn User>,

    global_github_api: Arc<dyn GitHubApiClient>,
    global_gitlab_api: Arc<dyn GitLabApiClient>,

    project_service: ProjectService,
    projects: OnceCell<RwLock<FxHashMap<ProjectId, RuntimeProject>>>,

    // FIXME: Is it correctly to have environment service created at app level and pass it to the workspace?
    // The AppService needs the environment service to initialize predefined environments before a workspace is fully created
    environment_service: Arc<EnvironmentService>,
    environments: OnceCell<RwLock<FxHashMap<EnvironmentId, RuntimeEnvironment>>>,
    environment_groups: RwLock<FxHashSet<ProjectId>>,

    // The key is the project ID or GLOBAL_ACTIVE_ENVIRONMENT_KEY
    active_environments: RwLock<FxHashMap<Arc<String>, EnvironmentId>>,
}

impl RuntimeWorkspace {
    pub fn new(
        id: WorkspaceId,
        abs_path: PathBuf,
        fs: Arc<dyn FileSystem>,
        storage: Arc<dyn KvStorage>,
        edit: Arc<dyn WorkspaceEditOp>,
        user: Arc<dyn User>,
        global_github_api: Arc<dyn GitHubApiClient>,
        global_gitlab_api: Arc<dyn GitLabApiClient>,
        project_service: ProjectService,
        environment_service: Arc<EnvironmentService>,
    ) -> Self {
        Self {
            id,
            abs_path,
            fs,
            storage,
            edit: edit.clone(),
            user,
            global_github_api,
            global_gitlab_api,
            project_service,
            projects: OnceCell::new(),
            environment_service,
            environments: OnceCell::new(),
            environment_groups: RwLock::new(FxHashSet::default()),
            active_environments: Default::default(),
        }
    }

    async fn projects_internal(
        &self,
        ctx: &dyn AnyAsyncContext,
    ) -> joinerror::Result<&RwLock<FxHashMap<ProjectId, RuntimeProject>>> {
        self.projects
            .get_or_try_init(|| async {
                let projects = self.project_service.projects(ctx).await?;

                let mut result = FxHashMap::default();
                for project in projects {
                    let builder = ProjectBuilder::new(
                        self.fs.clone(),
                        self.storage.clone(),
                        project.id.clone(),
                    )
                    .await;
                    let handle = continue_if_err!(
                        builder
                            .load(
                                ctx,
                                ProjectLoadParams {
                                    internal_abs_path: project.internal_abs_path.clone().into(),
                                },
                            )
                            .await,
                        |e| {
                            tracing::warn!(
                                "failed to load project '{}': {}",
                                project.id.to_string(),
                                e
                            );
                        }
                    );

                    self.environment_service
                        .add_source(ctx, &project.id, &handle.abs_path().join("environments"))
                        .await?;

                    result.insert(
                        project.id.clone(),
                        RuntimeProject {
                            id: project.id.clone(),
                            handle: handle.into(),
                            edit: ProjectEditService::new(ProjectFsEditBackend::new(
                                self.fs.clone(),
                                self.abs_path.join("projects"),
                            )),
                            order: project.order,
                        },
                    );
                }

                Ok::<_, joinerror::Error>(RwLock::new(result))
            })
            .await
            .join_err::<()>("failed to get projects")
    }

    async fn environments_internal(
        &self,
        ctx: &dyn AnyAsyncContext,
    ) -> joinerror::Result<&RwLock<FxHashMap<EnvironmentId, RuntimeEnvironment>>> {
        self.environments
            .get_or_try_init(|| async {
                // HACK: There's a weird race condition
                // Since project environment sources are added during project load
                // When stream_projects and stream_environments are called simultaneously
                // The project sources are not added correctly, making their environments non-discoverable
                // The hack is to ensure that projects_internal finishes here
                // Later we will split out the streaming of workspace vs project environments

                let _ = self.projects_internal(ctx).await?;

                let active_environments_result = self
                    .storage
                    .get(
                        ctx,
                        StorageScope::Workspace(self.id.inner()),
                        KEY_ACTIVE_ENVIRONMENTS,
                    )
                    .await;

                let active_environments: FxHashMap<Arc<String>, EnvironmentId> =
                    match active_environments_result {
                        Ok(Some(active_environments)) => {
                            serde_json::from_value(active_environments).unwrap_or_default()
                        }
                        Ok(None) => FxHashMap::default(),
                        Err(e) => {
                            tracing::warn!(
                                "failed to get activated environments from the db: {}",
                                e
                            );
                            FxHashMap::default()
                        }
                    };

                let mut active_environments_lock = self.active_environments.write().await;
                (*active_environments_lock).extend(active_environments);

                let environments = self.environment_service.environments(ctx).await?;

                let mut result = FxHashMap::default();

                for environment in environments {
                    let env_id = environment.id;
                    let builder = EnvironmentBuilder::new(
                        self.id.inner(),
                        self.fs.clone(),
                        self.storage.clone(),
                        env_id.clone(),
                    );

                    let handle = builder
                        .load(EnvironmentLoadParams {
                            abs_path: environment.internal_abs_path.clone(),
                        })
                        .await?;

                    result.insert(
                        env_id.clone(),
                        RuntimeEnvironment {
                            id: env_id.clone(),
                            project_id: environment.project_id.clone(),
                            handle: handle.into(),
                            edit: EnvironmentEditService::new(EnvironmentFsEditBackend::new(
                                &environment.internal_abs_path,
                                self.fs.clone(),
                            )),
                            order: None,
                        },
                    );

                    if let Some(group_id) = environment.project_id {
                        self.environment_groups.write().await.insert(group_id);
                    }
                }

                Ok::<_, joinerror::Error>(RwLock::new(result))
            })
            .await
            .join_err::<()>("failed to get environments")
    }
}

#[async_trait]
impl Workspace for RuntimeWorkspace {
    fn id(&self) -> WorkspaceId {
        self.id.clone()
    }
    fn abs_path(&self) -> PathBuf {
        self.abs_path.clone()
    }

    async fn edit(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: WorkspaceEditParams,
    ) -> joinerror::Result<()> {
        self.edit.edit(ctx, &self.id, params).await
    }

    async fn dispose(&self) -> joinerror::Result<()> {
        Ok(())
    }

    async fn project(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
    ) -> joinerror::Result<RuntimeProject> {
        let projects = self.projects_internal(ctx).await?;
        let project = projects
            .read()
            .await
            .get(id)
            .cloned()
            .ok_or_join_err_with::<()>(|| format!("project {} not found", id.to_string()))?;

        Ok(project)
    }

    async fn create_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: CreateProjectParams,
    ) -> joinerror::Result<RuntimeProject> {
        let git_params = if let Some(p) = params.git_params.clone() {
            let repository_url = GitUrl::parse(&p.repository_url_string())
                .join_err::<()>("failed to parse repository url")?;
            Some(CreateProjectGitParams {
                provider_kind: p.provider_kind(),
                repository_url,
                branch_name: p.branch_name(),
            })
        } else {
            None
        };

        let project_item = self
            .project_service
            .create_project(
                ctx,
                params.name.clone(),
                params.order,
                params.external_path,
                git_params.clone(),
                params.icon_path.clone(),
            )
            .await?;

        let builder = ProjectBuilder::new(
            self.fs.clone(),
            self.storage.clone(),
            project_item.id.clone(),
        )
        .await;

        let handle = builder
            .create(
                ctx,
                ProjectCreateParams {
                    name: Some(params.name),
                    abs_path: project_item.internal_abs_path.clone(),
                    config: project_item.config.clone(),
                    icon_path: params.icon_path,
                },
            )
            .await?;

        self.environment_service
            .add_source(ctx, handle.id(), &handle.abs_path().join("environments"))
            .await?;

        let project = RuntimeProject {
            id: project_item.id.clone(),
            handle: handle.into(),
            edit: ProjectEditService::new(ProjectFsEditBackend::new(
                self.fs.clone(),
                self.abs_path.join("projects"),
            )),
            order: Some(params.order),
        };

        let account = if let Some(git_params) = params.git_params.map(|p| p.account_id()) {
            self.user.account(&git_params).await
        } else {
            None
        };

        if let (Some(git_params), Some(account)) = (git_params, account) {
            let client = match git_params.provider_kind {
                GitProviderKind::GitHub => GitClient::GitHub {
                    account: account,
                    api: self.global_github_api.clone(),
                },
                GitProviderKind::GitLab => GitClient::GitLab {
                    account: account,
                    api: self.global_gitlab_api.clone(),
                },
            };

            if let Err(e) = project
                .handle
                .init_vcs(
                    ctx,
                    client,
                    git_params.repository_url,
                    git_params.branch_name,
                )
                .await
            {
                tracing::warn!("failed to init vcs: {}", e.to_string());
                // app_delegate.emit_oneshot(ToLocation::Toast {
                //     activity_id: "create_collection_init_vcs_failure",
                //     title: localize!(NO_TRANSLATE_KEY, "Failed to initialized collection vcs"),
                //     detail: Some(localize!(
                //         NO_TRANSLATE_KEY,
                //         "Failed to initialize collection vcs, creating a local only collection"
                //     )),
                // })?;
            }
        }

        let projects = self.projects_internal(ctx).await?;
        projects
            .write()
            .await
            .insert(project_item.id.clone(), project.clone());

        if let Err(e) = self
            .storage
            .add_project(self.id.inner(), project_item.id.inner())
            .await
        {
            return Err(joinerror::Error::new::<()>(format!(
                "failed to add project storage: {}",
                e
            )));
        }

        Ok(project)
    }

    async fn clone_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        account_id: &AccountId,
        git_provider_kind: GitProviderKind,
        repository: &str,
        branch: Option<String>,
    ) -> joinerror::Result<RuntimeProject> {
        let account = self
            .user
            .account(account_id)
            .await
            .ok_or_join_err_with::<()>(|| format!("account `{}` not found", account_id.inner()))?;

        let repo_url =
            GitUrl::parse(&repository).join_err::<()>("failed to parse repository url")?;
        // 1. Create directory
        // 2. Clone repo
        // 3. Setup config
        let (project_item, repository) = self
            .project_service
            .clone_project(
                ctx,
                &account,
                git_provider_kind.clone(),
                repo_url.clone(),
                branch,
            )
            .await?;

        // 4. Build project
        let builder = ProjectBuilder::new(
            self.fs.clone(),
            self.storage.clone(),
            project_item.id.clone(),
        )
        .await;

        let git_client = match git_provider_kind {
            GitProviderKind::GitHub => GitClient::GitHub {
                account,
                api: self.global_github_api.clone(),
            },
            GitProviderKind::GitLab => GitClient::GitLab {
                account,
                api: self.global_gitlab_api.clone(),
            },
        };

        let handle = builder
            .clone(
                ctx,
                repository,
                git_client,
                ProjectCloneParams {
                    internal_abs_path: project_item.internal_abs_path.clone().into(),
                    repository: repo_url,
                },
            )
            .await?;

        self.environment_service
            .add_source(ctx, handle.id(), &handle.abs_path().join("environments"))
            .await?;

        let project = RuntimeProject {
            id: project_item.id.clone(),
            handle: handle.into(),
            edit: ProjectEditService::new(ProjectFsEditBackend::new(
                self.fs.clone(),
                self.abs_path.join("projects"),
            )),
            order: None, // HACK: deprecated field
        };

        // 5. Add project to registry and storage
        let projects = self.projects_internal(ctx).await?;
        projects
            .write()
            .await
            .insert(project_item.id.clone(), project.clone());

        if let Err(e) = self
            .storage
            .add_project(self.id.inner(), project_item.id.inner())
            .await
        {
            return Err(joinerror::Error::new::<()>(format!(
                "failed to add project storage: {}",
                e
            )));
        }

        Ok(project)
    }

    async fn import_archived_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: &ImportArchiveParams,
    ) -> joinerror::Result<RuntimeProject> {
        let project_item = self
            .project_service
            .import_archived_project(ctx, &params.archive_path)
            .await?;

        let builder = ProjectBuilder::new(
            self.fs.clone(),
            self.storage.clone(),
            project_item.id.clone(),
        )
        .await;

        let handle = builder
            .import_archive(
                ctx,
                ProjectImportArchiveParams {
                    internal_abs_path: project_item.internal_abs_path.clone().into(),
                },
            )
            .await?;
        self.environment_service
            .add_source(ctx, handle.id(), &handle.abs_path().join("environments"))
            .await?;

        let project = RuntimeProject {
            id: project_item.id.clone(),
            handle: handle.into(),
            edit: ProjectEditService::new(ProjectFsEditBackend::new(
                self.fs.clone(),
                self.abs_path.join("projects"),
            )),
            order: None, // HACK: deprecated field
        };

        let projects = self.projects_internal(ctx).await?;
        projects
            .write()
            .await
            .insert(project_item.id.clone(), project.clone());

        if let Err(e) = self
            .storage
            .add_project(self.id.inner(), project_item.id.inner())
            .await
        {
            return Err(joinerror::Error::new::<()>(format!(
                "failed to add project storage: {}",
                e
            )));
        }

        Ok(project)
    }

    async fn import_external_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: &ImportDiskParams,
    ) -> joinerror::Result<RuntimeProject> {
        let project_item = self
            .project_service
            .import_external_project(ctx, &params.external_path)
            .await?;

        let builder = ProjectBuilder::new(
            self.fs.clone(),
            self.storage.clone(),
            project_item.id.clone(),
        )
        .await;

        let handle = builder
            .import_external(
                ctx,
                ProjectImportExternalParams {
                    internal_abs_path: project_item.internal_abs_path.clone().into(),
                    external_abs_path: params.external_path.clone().into(),
                },
            )
            .await?;

        self.environment_service
            .add_source(ctx, handle.id(), &handle.abs_path().join("environments"))
            .await?;

        let project = RuntimeProject {
            id: project_item.id.clone(),
            handle: handle.into(),
            edit: ProjectEditService::new(ProjectFsEditBackend::new(
                self.fs.clone(),
                self.abs_path.join("projects"),
            )),
            order: None, // HACK: deprecated field
        };

        let projects = self.projects_internal(ctx).await?;
        projects
            .write()
            .await
            .insert(project_item.id.clone(), project.clone());

        if let Err(e) = self
            .storage
            .add_project(self.id.inner(), project_item.id.inner())
            .await
        {
            return Err(joinerror::Error::new::<()>(format!(
                "failed to add project storage: {}",
                e
            )));
        }

        Ok(project)
    }

    async fn delete_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
    ) -> joinerror::Result<Option<PathBuf>> {
        let project = self.projects_internal(ctx).await?.write().await.remove(id);
        if project.is_none() {
            return Ok(None);
        }

        // Dropping repo and database handle to prevent lock when deleting the folder
        let project = project.unwrap();
        project.handle.dispose(ctx).await?;

        self.environment_service
            .remove_source(ctx, &project.id)
            .await?;

        self.storage
            .remove_project(self.id.inner(), project.id.inner())
            .await?;

        let path = self.project_service.delete_project(ctx, id).await?;

        if let Err(e) = self
            .storage
            .remove_batch_by_prefix(
                ctx,
                StorageScope::Workspace(self.id.inner()),
                &key_project(id),
            )
            .await
        {
            tracing::warn!("failed to remove project `{}` from storage: {}", id, e);
        }

        let mut environment_groups = self.environment_groups.write().await;
        environment_groups.remove(&id);

        Ok(path)
    }

    async fn update_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: UpdateProjectParams,
    ) -> joinerror::Result<()> {
        let project = self.project(ctx, &params.id).await?;

        // TODO: Implement relinking and unlinking remote repo when the user update it
        // Right now the repository will be updated in the config
        // But the repo will not be linked to a new repo
        project
            .edit
            .edit(ctx, &params.id, ProjectEditParams { name: params.name })
            .await?;

        // TODO: Migrate icon update logic or remove it?
        Ok(())
    }

    async fn archive_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
    ) -> joinerror::Result<()> {
        let project = self.project(ctx, &id).await?;
        project.handle.archive(ctx).await?;

        project
            .edit
            .edit_config(
                ctx,
                id,
                ProjectConfigEditParams {
                    archived: Some(true),
                },
            )
            .await?;

        Ok(())
    }

    async fn unarchive_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
    ) -> joinerror::Result<()> {
        let project = self.project(ctx, &id).await?;
        project.handle.unarchive(ctx).await?;

        project
            .edit
            .edit_config(
                ctx,
                id,
                ProjectConfigEditParams {
                    archived: Some(false),
                },
            )
            .await?;

        Ok(())
    }

    async fn export_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: ExportProjectParams,
    ) -> joinerror::Result<PathBuf> {
        let archive_path = self
            .project_service
            .export_archive(ctx, &params.id, &params.destination)
            .await?;

        Ok(archive_path)
    }

    async fn projects(&self, ctx: &dyn AnyAsyncContext) -> joinerror::Result<Vec<RuntimeProject>> {
        let projects = self.projects_internal(ctx).await?;

        Ok(projects.read().await.clone().into_values().collect())
    }

    async fn activate_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &EnvironmentId,
    ) -> joinerror::Result<()> {
        dbg!(1);
        let environments = self.environments_internal(ctx).await?.write().await;

        dbg!(2);
        let environment_item = environments
            .get(&id)
            .ok_or_join_err_with::<()>(|| format!("environment {} not found", id.to_string()))?;

        dbg!(3);
        let env_group_key = if let Some(project_id) = &environment_item.project_id {
            project_id.inner()
        } else {
            GLOBAL_ACTIVE_ENVIRONMENT_KEY.to_string().into()
        };

        let mut active_environments = self.active_environments.write().await;
        active_environments.insert(env_group_key, id.clone());

        dbg!(4);
        if let Err(e) = self
            .storage
            .put(
                ctx,
                StorageScope::Workspace(self.id.inner()),
                KEY_ACTIVE_ENVIRONMENTS,
                serde_json::to_value(active_environments.clone())?,
            )
            .await
        {
            tracing::warn!("failed to put active environments in the database: {}", e);
        }

        dbg!(5);
        Ok(())
    }

    async fn create_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        input: CreateEnvironmentInput,
    ) -> joinerror::Result<EnvironmentItemDescription> {
        let id = EnvironmentId::new();
        let environment_item = self
            .environment_service
            .create_environment(
                ctx,
                &self.id,
                CreateEnvironmentItemParams {
                    env_id: id.clone(),
                    project_id: input.project_id.clone(),
                    name: input.name.clone(),
                    order: input.order.clone(),
                    color: input.color.clone(),
                    variables: input.variables.clone(),
                },
            )
            .await?;

        let builder = EnvironmentBuilder::new(
            self.id.inner(),
            self.fs.clone(),
            self.storage.clone(),
            id.clone(),
        );

        let handle = builder
            .create(
                ctx,
                CreateEnvironmentParams {
                    name: input.name.clone(),
                    abs_path: &environment_item.internal_abs_path,
                    color: input.color.clone(),
                    variables: input.variables,
                },
            )
            .await?;

        let env_abs_path = handle.abs_path().await.to_owned();
        let environment = RuntimeEnvironment {
            id: environment_item.id.clone(),
            project_id: input.project_id.clone(),
            handle: handle.into(),
            edit: EnvironmentEditService::new(EnvironmentFsEditBackend::new(
                env_abs_path.as_ref(),
                self.fs.clone(),
            )),
            order: Some(input.order.clone()),
        };

        let environments = self.environments_internal(ctx).await?;
        environments
            .write()
            .await
            .insert(environment_item.id.clone(), environment.clone());

        let desc = environment.handle.describe(ctx).await?;

        if let Some(project_id) = &input.project_id {
            let mut groups_lock = self.environment_groups.write().await;
            groups_lock.insert(project_id.clone());
        }

        Ok(EnvironmentItemDescription {
            id: desc.id.clone(),
            project_id: input.project_id.clone(),
            is_active: false,
            display_name: input.name.clone(),
            order: Some(input.order.clone()),
            color: desc.color.clone(),
            abs_path: env_abs_path.into(),
            total_variables: desc.variables.len(),
        })
    }

    async fn delete_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &EnvironmentId,
    ) -> joinerror::Result<()> {
        let environments = self.environments_internal(ctx).await?;

        let environment = if let Some(environment) = environments.write().await.remove(id) {
            environment
        } else {
            return Ok(());
        };

        let env_group_key = if let Some(project_id) = environment.project_id {
            project_id.inner()
        } else {
            GLOBAL_ACTIVE_ENVIRONMENT_KEY.to_string().into()
        };

        // If the environment is currently active, removes it from active environments
        let active_environments_updated = {
            let mut active_environments = self.active_environments.write().await;
            if active_environments.get(&env_group_key) == Some(&environment.id) {
                active_environments.remove(&env_group_key);
                true
            } else {
                false
            }
        };

        let desc = environment.handle.describe(ctx).await?;

        self.environment_service
            .delete_environment(ctx, &self.id, desc)
            .await?;

        if active_environments_updated {
            let active_environments = self.active_environments.read().await;
            if let Err(e) = self
                .storage
                .put(
                    ctx,
                    StorageScope::Workspace(self.id.inner()),
                    KEY_ACTIVE_ENVIRONMENTS,
                    serde_json::to_value(active_environments.to_owned())?,
                )
                .await
            {
                tracing::warn!(
                    "failed to update active_environments in the database: {}",
                    e
                );
            }
        }

        Ok(())
    }

    async fn update_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: UpdateEnvironmentParams,
    ) -> joinerror::Result<()> {
        let environment = self.environment(ctx, &params.id).await?;

        // We need to assign VariableId to newly created variables
        let vars_to_add = params
            .vars_to_add
            .iter()
            .map(|params| (VariableId::new(), params.to_owned()))
            .collect::<Vec<_>>();
        environment
            .edit
            .edit(
                ctx,
                EnvironmentEditParams {
                    name: params.name,
                    color: params.color,
                    vars_to_add: vars_to_add.clone(),
                    vars_to_update: params.vars_to_update.clone(),
                    vars_to_delete: params.vars_to_delete.clone(),
                },
            )
            .await
            .join_err_with::<()>(|| format!("failed to update environment {}", params.id))?;

        let storage_scope = StorageScope::Workspace(self.id.inner());

        // Again issue with the signature of put_batch, will try to fix it later
        for (var_id, var_to_add) in vars_to_add {
            if let Err(e) = self
                .storage
                .put(
                    ctx,
                    storage_scope.clone(),
                    &key_variable_local_value(&params.id, &var_id),
                    var_to_add.local_value,
                )
                .await
            {
                tracing::error!("failed to add variable local value to the database: {}", e);
            }
        }

        for var_to_update in params.vars_to_update {
            match var_to_update.local_value {
                Some(ChangeJsonValue::Update(value)) => {
                    if let Err(e) = self
                        .storage
                        .put(
                            ctx,
                            storage_scope.clone(),
                            &key_variable_local_value(&params.id, &var_to_update.id),
                            value,
                        )
                        .await
                    {
                        tracing::error!(
                            "failed to update variable local value in the database: {}",
                            e
                        );
                    }
                }
                Some(ChangeJsonValue::Remove) => {
                    if let Err(e) = self
                        .storage
                        .remove(
                            ctx,
                            storage_scope.clone(),
                            &key_variable_local_value(&params.id, &var_to_update.id),
                        )
                        .await
                    {
                        tracing::error!(
                            "failed to remove variable local value in the database: {}",
                            e
                        );
                    }
                }
                None => {}
            }
        }

        for id in params.vars_to_delete {
            if let Err(e) = self
                .storage
                .remove_batch_by_prefix(ctx, storage_scope.clone(), &key_variable(&params.id, &id))
                .await
            {
                tracing::error!("failed to remove variable data from the database: {}", e);
            }
        }

        Ok(())
    }

    async fn batch_update_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        updates: Vec<UpdateEnvironmentParams>,
    ) -> joinerror::Result<()> {
        for update in updates {
            self.update_environment(ctx, update).await?
        }
        Ok(())
    }

    // async fn update_environment_group(&self, ctx: &dyn AnyAsyncContext, params: UpdateEnvironmentGroupParams) -> joinerror::Result<()> {
    //     todo!()
    // }
    //
    // async fn batch_update_environment_groups(&self, ctx: &dyn AnyAsyncContext, updates: Vec<UpdateEnvironmentGroupParams>) -> joinerror::Result<()> {
    //     todo!()
    // }

    async fn environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &EnvironmentId,
    ) -> joinerror::Result<RuntimeEnvironment> {
        let environments = self.environments_internal(ctx).await?;
        let environment = environments
            .read()
            .await
            .get(id)
            .cloned()
            .ok_or_join_err_with::<()>(|| format!("environment {} not found", id.to_string()))?;

        Ok(environment)
    }

    async fn environments(
        &self,
        ctx: &dyn AnyAsyncContext,
    ) -> joinerror::Result<Vec<RuntimeEnvironment>> {
        let environments = self.environments_internal(ctx).await?;

        Ok(environments.read().await.clone().into_values().collect())
    }

    async fn active_environments(
        &self,
        _ctx: &dyn AnyAsyncContext,
    ) -> joinerror::Result<FxHashMap<Arc<String>, EnvironmentId>> {
        let active_environments = self.active_environments.read().await;

        Ok(active_environments.clone())
    }

    async fn environment_groups(
        &self,
        ctx: &dyn AnyAsyncContext,
    ) -> joinerror::Result<FxHashSet<ProjectId>> {
        let environment_groups = self.environment_groups.read().await.clone();

        Ok(environment_groups)
    }
}
