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

// Now the environment service fs is only responsible for a single environments folder

pub struct EnvironmentServiceFs {
    environments_path: PathBuf,
    fs: Arc<dyn FileSystem>,
}

impl EnvironmentServiceFs {
    pub fn new(environments_path: PathBuf, fs: Arc<dyn FileSystem>) -> Self {
        Self {
            environments_path,
            fs,
        }
    }
}

#[async_trait]
impl EnvironmentServiceFsPort for EnvironmentServiceFs {
    // FIXME: For now I will scan both workspace and project level environments
    async fn lookup_environments(
        &self,
        ctx: &dyn AnyAsyncContext,
    ) -> joinerror::Result<Vec<LookedUpEnvironment>> {
        let mut environments = vec![];
        lookup_source(self, ctx, &self.environments_path, &mut environments).await?;

        Ok(environments)
    }

    async fn create_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &EnvironmentId,
        params: &CreateEnvironmentFsParams,
    ) -> joinerror::Result<PathBuf> {
        let file_name = format_env_file_name(id);

        let abs_path = self.environments_path.join(&file_name);
        if abs_path.exists() {
            bail!("environment {} already exists", id.to_string());
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
        id: &EnvironmentId,
    ) -> joinerror::Result<()> {
        let file_name = format_env_file_name(id);
        let path = self.environments_path.join(&file_name);
        self.fs
            .remove_file(
                ctx,
                &path,
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
