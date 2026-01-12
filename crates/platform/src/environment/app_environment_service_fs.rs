// This is only used by the APP to create predefined environments after creating a workspace

use async_trait::async_trait;
use hcl::ser::{Block, LabeledBlock};
use joinerror::{ResultExt, bail};
use moss_environment::{
    configuration::{MetadataDecl, SourceFile},
    errors::{ErrorFailedToEncode, ErrorIo},
};
use moss_fs::{CreateOptions, FileSystem};
use moss_hcl::HclResultExt;
use sapic_base::{
    environment::types::primitives::EnvironmentId, workspace::types::primitives::WorkspaceId,
};
use sapic_core::context::AnyAsyncContext;
use sapic_system::environment::{
    AppEnvironmentServiceFs as AppEnvironmentServiceFsPort, CreateEnvironmentFsParams,
};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::project::format_env_file_name;

pub struct AppEnvironmentServiceFs {
    workspaces_path: PathBuf,
    fs: Arc<dyn FileSystem>,
}

impl AppEnvironmentServiceFs {
    pub fn new(workspaces_path: &Path, fs: Arc<dyn FileSystem>) -> Arc<Self> {
        Arc::new(Self {
            workspaces_path: workspaces_path.to_path_buf(),
            fs,
        })
    }
}

#[async_trait]
impl AppEnvironmentServiceFsPort for AppEnvironmentServiceFs {
    async fn create_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        workspace_id: &WorkspaceId,
        id: &EnvironmentId,
        params: &CreateEnvironmentFsParams,
    ) -> joinerror::Result<PathBuf> {
        let file_name = format_env_file_name(id);

        let abs_path = self
            .workspaces_path
            .join(workspace_id.to_string())
            .join("environments")
            .join(file_name);
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
}
