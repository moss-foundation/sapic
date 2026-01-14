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

    async fn read_environment_sourcefile(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &EnvironmentId,
    ) -> joinerror::Result<SourceFile> {
        let abs_path = self.environments_path.join(format_env_file_name(id));
        let rdr = self
            .fs
            .open_file(ctx, &abs_path)
            .await
            .join_err_with::<()>(|| format!("failed to open file: {}", abs_path.display()))?;

        let parsed: SourceFile = hcl::from_reader(rdr)
            .join_err_with::<()>(|| format!("failed to parse hcl: {}", abs_path.display()))?;

        Ok(parsed)
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
                    ignore_if_not_exists: true,
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

#[cfg(test)]
mod tests {
    use hcl::Expression;
    use indexmap::IndexMap;
    use moss_environment::{configuration::VariableDecl, models::types::VariableOptions};
    use moss_fs::RealFileSystem;
    use moss_testutils::random_name::random_string;
    use sapic_base::environment::types::primitives::VariableId;
    use sapic_core::context::ArcContext;

    use super::*;

    async fn setup_env_service_fs() -> (ArcContext, Arc<EnvironmentServiceFs>, PathBuf) {
        let ctx = ArcContext::background();
        let test_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("tests")
            .join("data")
            .join(random_string(10));

        let tmp_path = test_path.join("tmp");
        let environments_path = test_path.join("environments");

        tokio::fs::create_dir_all(&environments_path).await.unwrap();
        tokio::fs::create_dir_all(&tmp_path).await.unwrap();
        let fs = Arc::new(RealFileSystem::new(&tmp_path));
        let env_fs = Arc::new(EnvironmentServiceFs::new(environments_path, fs));

        (ctx, env_fs, test_path)
    }

    #[tokio::test]
    async fn test_create_environment_success() {
        let (ctx, env_fs, test_path) = setup_env_service_fs().await;

        let params = CreateEnvironmentFsParams {
            name: "Test".to_string(),
            color: Some(String::from("#ff0000")),
            variables: IndexMap::default(),
        };
        let id = EnvironmentId::new();
        let path = env_fs.create_environment(&ctx, &id, &params).await.unwrap();

        assert!(path.exists());

        let source_file = env_fs.read_environment_sourcefile(&ctx, &id).await.unwrap();

        assert_eq!(source_file.metadata.name, params.name);
        assert_eq!(source_file.metadata.color, params.color);

        tokio::fs::remove_dir_all(test_path).await.unwrap();
    }

    #[tokio::test]
    async fn test_create_environment_already_exists() {
        let (ctx, env_fs, test_path) = setup_env_service_fs().await;

        let params = CreateEnvironmentFsParams {
            name: "Test".to_string(),
            color: Some(String::from("#ff0000")),
            variables: IndexMap::default(),
        };
        let id = EnvironmentId::new();
        env_fs.create_environment(&ctx, &id, &params).await.unwrap();

        let result = env_fs.create_environment(&ctx, &id, &params).await;
        assert!(result.is_err());

        tokio::fs::remove_dir_all(test_path).await.unwrap();
    }

    #[tokio::test]
    async fn test_create_environment_with_variables() {
        let (ctx, env_fs, test_path) = setup_env_service_fs().await;

        let var_key1 = VariableId::new();
        let var_key2 = VariableId::new();

        let var_decl1 = VariableDecl {
            name: "Variable 1".to_string(),
            value: Expression::String("Variable 1".to_string()),
            description: Some("Description".to_string()),
            options: VariableOptions { disabled: false },
        };

        let var_decl2 = VariableDecl {
            name: "Variable 2".to_string(),
            value: Expression::Bool(false),
            description: Some("Disabled".to_string()),
            options: VariableOptions { disabled: true },
        };

        let mut variables = IndexMap::new();
        variables.insert(var_key1.clone(), var_decl1.clone());
        variables.insert(var_key2.clone(), var_decl2.clone());

        let params = CreateEnvironmentFsParams {
            name: "Test".to_string(),
            color: Some(String::from("#ff0000")),
            variables: variables.clone(),
        };

        let id = EnvironmentId::new();
        env_fs.create_environment(&ctx, &id, &params).await.unwrap();

        let source_file = env_fs.read_environment_sourcefile(&ctx, &id).await.unwrap();

        assert_eq!(source_file.metadata.name, params.name);
        assert_eq!(source_file.metadata.color, params.color);
        let stored_variables = source_file.variables.unwrap().into_inner();
        assert_eq!(stored_variables, variables);

        tokio::fs::remove_dir_all(test_path).await.unwrap();
    }

    #[tokio::test]
    async fn test_remove_environment_success() {
        let (ctx, env_fs, test_path) = setup_env_service_fs().await;

        let params = CreateEnvironmentFsParams {
            name: "Test".to_string(),
            color: Some(String::from("#ff0000")),
            variables: IndexMap::default(),
        };
        let id = EnvironmentId::new();
        let path = env_fs.create_environment(&ctx, &id, &params).await.unwrap();

        env_fs.remove_environment(&ctx, &id).await.unwrap();

        assert!(!path.exists());

        tokio::fs::remove_dir_all(test_path).await.unwrap();
    }

    // Deleting non-existent environment is handled gracefully
    #[tokio::test]
    async fn test_remove_environment_nonexistent() {
        let (ctx, env_fs, test_path) = setup_env_service_fs().await;
        let id = EnvironmentId::new();

        let result = env_fs.remove_environment(&ctx, &id).await;

        assert!(result.is_ok());

        tokio::fs::remove_dir_all(test_path).await.unwrap();
    }

    #[tokio::test]
    async fn test_lookup_environments_empty() {
        let (ctx, env_fs, test_path) = setup_env_service_fs().await;

        let environments = env_fs.lookup_environments(&ctx).await.unwrap();
        assert!(environments.is_empty());

        tokio::fs::remove_dir_all(test_path).await.unwrap();
    }

    #[tokio::test]
    async fn test_lookup_environments_after_creation() {
        let (ctx, env_fs, test_path) = setup_env_service_fs().await;

        let params = CreateEnvironmentFsParams {
            name: "Test".to_string(),
            color: Some(String::from("#ff0000")),
            variables: IndexMap::default(),
        };
        let id = EnvironmentId::new();
        let path = env_fs.create_environment(&ctx, &id, &params).await.unwrap();

        let environments = env_fs.lookup_environments(&ctx).await.unwrap();
        assert_eq!(environments.len(), 1);
        assert_eq!(environments[0].id, id);
        assert_eq!(environments[0].internal_abs_path, path);
        tokio::fs::remove_dir_all(test_path).await.unwrap();
    }

    #[tokio::test]
    async fn test_lookup_environments_after_deletion() {
        let (ctx, env_fs, test_path) = setup_env_service_fs().await;

        let params = CreateEnvironmentFsParams {
            name: "Test".to_string(),
            color: Some(String::from("#ff0000")),
            variables: IndexMap::default(),
        };
        let id = EnvironmentId::new();
        let path = env_fs.create_environment(&ctx, &id, &params).await.unwrap();

        env_fs.remove_environment(&ctx, &id).await.unwrap();

        let environments = env_fs.lookup_environments(&ctx).await.unwrap();
        assert!(environments.is_empty());

        tokio::fs::remove_dir_all(test_path).await.unwrap();
    }
}
