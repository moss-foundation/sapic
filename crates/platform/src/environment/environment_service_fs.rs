use async_trait::async_trait;
use joinerror::{OptionExt, ResultExt, bail};
use moss_environment::{
    configuration::{MetadataDecl, SourceFile},
    constants::ENVIRONMENT_FILE_EXTENSION,
    errors::{ErrorFailedToEncode, ErrorIo},
};
use moss_fs::{CreateOptions, FileSystem, RemoveOptions};
use moss_hcl::{Block, HclResultExt, LabeledBlock};
use moss_storage2::KvStorage;
use sapic_base::{
    environment::types::primitives::EnvironmentId, project::types::primitives::ProjectId,
    workspace::types::primitives::WorkspaceId,
};
use sapic_core::context::AnyAsyncContext;
use sapic_system::environment::{
    CreateEnvironmentFsParams, EnvironmentServiceFs as EnvironmentServiceFsPort,
    LookedUpEnvironment,
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::RwLock;

use crate::project::{format_env_file_name, parse_file_name};

struct ServiceState {
    pub(crate) sources: HashMap<Option<ProjectId>, PathBuf>,
}

pub struct EnvironmentServiceFs {
    workspaces_path: PathBuf,
    state: Arc<RwLock<ServiceState>>,
    fs: Arc<dyn FileSystem>,
}

impl EnvironmentServiceFs {
    pub fn new(workspaces_path: &Path, fs: Arc<dyn FileSystem>) -> Arc<Self> {
        // The service will be initiated at app build, at which point no workspace is open yet
        // Thus no sources are present until workspaces are loaded
        Arc::new(Self {
            workspaces_path: workspaces_path.to_path_buf(),
            state: Arc::new(RwLock::new(ServiceState {
                sources: HashMap::new(),
            })),
            fs,
        })
    }
}

#[async_trait]
impl EnvironmentServiceFsPort for EnvironmentServiceFs {
    async fn switch_workspace(
        &self,
        _ctx: &dyn AnyAsyncContext,
        workspace_id: &WorkspaceId,
    ) -> joinerror::Result<()> {
        let mut state = self.state.write().await;

        // Remove old workspace sources and initialize sources with path to global workspace environments
        // Project environment sources will be added once the projects are loaded
        state.sources.clear();

        state.sources.insert(
            None,
            self.workspaces_path
                .join(workspace_id.to_string())
                .join("environments"),
        );

        Ok(())
    }

    async fn add_source(
        &self,
        ctx: &dyn AnyAsyncContext,
        project_id: &ProjectId,
        source_path: &Path,
    ) -> joinerror::Result<()> {
        self.state
            .write()
            .await
            .sources
            .insert(Some(project_id.to_owned()), source_path.to_path_buf());

        Ok(())
    }

    async fn remove_source(
        &self,
        ctx: &dyn AnyAsyncContext,
        project_id: &ProjectId,
    ) -> joinerror::Result<()> {
        self.state
            .write()
            .await
            .sources
            .remove(&Some(project_id.clone()));
        Ok(())
    }

    // FIXME: For now I will scan both workspace and project level environments
    async fn lookup_environments(
        &self,
        ctx: &dyn AnyAsyncContext,
    ) -> joinerror::Result<Vec<LookedUpEnvironment>> {
        let sources = self.state.read().await.sources.clone();
        // For simplicity, I didn't migrate the scanner logic, just use the same one from lookup projects
        let mut environments = vec![];
        for (project_id, source) in sources {
            lookup_source(self, ctx, project_id, &source, &mut environments).await?;
        }

        Ok(environments)
    }

    async fn initialize_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        workspace_id: &WorkspaceId,
        id: &EnvironmentId,
        params: &CreateEnvironmentFsParams,
    ) -> joinerror::Result<PathBuf> {
        let state = self.state.read().await;

        let env_folder = if let Some(project_id) = &params.project_id {
            let source = state
                .sources
                .get(&Some(project_id.to_owned()))
                .ok_or_join_err_with::<()>(|| {
                    format!("source not found for project {}", project_id)
                })?;

            source.clone()
        } else {
            self.workspaces_path
                .join(workspace_id.to_string())
                .join("environments")
        };

        let file_name = format_env_file_name(id);

        let abs_path = env_folder.join(&file_name);
        if abs_path.exists() {
            bail!("environment {} already exists", params.name);
        }

        let content = hcl::to_string(&SourceFile {
            metadata: Block::new(MetadataDecl {
                name: params.name.clone(),
                color: params.color.clone(),
            }),
            variables: Some(LabeledBlock::new(params.variables.clone())),
        })
        .join_err_with::<ErrorFailedToEncode>(|| {
            format!("failed to encode environment file {}", abs_path.display())
        })?;

        self.fs
            .create_file_with(
                ctx,
                &abs_path,
                content.as_bytes(),
                CreateOptions {
                    overwrite: false,
                    ignore_if_exists: false,
                },
            )
            .await
            .join_err_with::<ErrorIo>(|| {
                format!("failed to create environment file {}", abs_path.display())
            })?;

        Ok(abs_path)
    }

    async fn remove_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        path: &Path,
    ) -> joinerror::Result<()> {
        self.fs
            .remove_file(
                ctx,
                path,
                RemoveOptions {
                    recursive: false,
                    ignore_if_not_exists: false,
                },
            )
            .await
            .join_err_with::<()>(|| format!("failed to remove environment {}", path.display()))?;

        Ok(())
    }
}

async fn lookup_source(
    env_fs: &EnvironmentServiceFs,
    ctx: &dyn AnyAsyncContext,
    project_id: Option<ProjectId>,
    source_path: &Path,
    environments: &mut Vec<LookedUpEnvironment>,
) -> joinerror::Result<()> {
    let mut read_dir = env_fs.fs.read_dir(ctx, source_path).await?;
    while let Some(entry) = read_dir.next_entry().await? {
        if !entry.file_type().await?.is_file() {
            continue;
        }

        // I assume at this step we don't try to parse the file, only return a list of possible environment files
        // The actual parsing of the contract is beyond the scope
        let file_name = entry.file_name().to_string_lossy().to_string();
        if !file_name.ends_with(ENVIRONMENT_FILE_EXTENSION) {
            continue;
        }

        match parse_file_name(&file_name) {
            Ok(env_id) => environments.push(LookedUpEnvironment {
                id: env_id.into(),
                project_id: project_id.clone(),
                internal_abs_path: entry.path(),
            }),
            Err(err) => {
                tracing::warn!(
                    "encountered environment file with invalid name '{}': {}",
                    file_name,
                    err.to_string()
                );
            }
        }
    }

    Ok(())
}
