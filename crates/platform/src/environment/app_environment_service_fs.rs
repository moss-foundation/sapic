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

#[cfg(test)]
mod tests {
    use moss_fs::RealFileSystem;
    use moss_testutils::random_name::random_string;
    use sapic_core::context::ArcContext;

    use super::*;
    async fn setup_app_env_service_fs() -> (ArcContext, Arc<AppEnvironmentServiceFs>, PathBuf) {
        let ctx = ArcContext::background();
        let test_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("tests")
            .join("data")
            .join(random_string(10));

        let tmp_path = test_path.join("tmp");
        let workspaces_path = test_path.join("workspaces");

        tokio::fs::create_dir_all(&tmp_path).await.unwrap();
        tokio::fs::create_dir_all(&workspaces_path).await.unwrap();

        let fs = Arc::new(RealFileSystem::new(&tmp_path));
        let env_fs = AppEnvironmentServiceFs::new(&workspaces_path, fs.clone());

        (ctx, env_fs, test_path)
    }

    #[tokio::test]
    async fn test_create_environment_success() {
        let (ctx, env_fs, test_path) = setup_app_env_service_fs().await;

        let workspace_id = WorkspaceId::new();
        let workspace_path = test_path.join("workspaces").join(workspace_id.to_string());
        let environment_path = workspace_path.join("environments");
        tokio::fs::create_dir_all(&workspace_path).await.unwrap();
        tokio::fs::create_dir_all(&environment_path).await.unwrap();

        let environment_id = EnvironmentId::new();

        let params = CreateEnvironmentFsParams {
            name: "Globals".to_string(),
            color: Some(String::from("#ffffff")),
            variables: Default::default(),
        };
        let path = env_fs
            .create_environment(&ctx, &workspace_id, &environment_id, &params)
            .await
            .unwrap();

        let content = std::fs::read_to_string(&path).unwrap();
        let parsed: SourceFile = hcl::from_str(&content).unwrap();

        assert_eq!(parsed.metadata.name, params.name);
        assert_eq!(parsed.metadata.color, params.color);

        dbg!(&test_path);

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    async fn test_create_environment_nonexistent_workspace() {
        let (ctx, env_fs, test_path) = setup_app_env_service_fs().await;

        let workspace_id = WorkspaceId::new();

        let environment_id = EnvironmentId::new();

        let params = CreateEnvironmentFsParams {
            name: "Globals".to_string(),
            color: Some(String::from("#ffffff")),
            variables: Default::default(),
        };

        let result = env_fs
            .create_environment(&ctx, &workspace_id, &environment_id, &params)
            .await;
        assert!(result.is_err());

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }
}
