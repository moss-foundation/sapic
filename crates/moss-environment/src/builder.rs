use joinerror::{Error, ResultExt};
use moss_applib::AppRuntime;
use moss_contentmodel::{ContentModel, json::JsonModel};
use moss_fs::{CreateOptions, FileSystem, FsResultExt, model_registry::GlobalModelRegistry};
use moss_hcl::{Block, HclResultExt};
use moss_text::sanitized::sanitize;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::RwLock;

use crate::{
    configuration::{MetadataDecl, SourceFile},
    constants,
    environment::{Environment, EnvironmentPath, EnvironmentState},
    errors::{
        ErrorEnvironmentAlreadyExists, ErrorEnvironmentNotFound, ErrorFailedToDecode,
        ErrorFailedToEncode, ErrorIo,
    },
    models::primitives::EnvironmentId,
    services::{
        metadata_service::MetadataService, sync_service::SyncService,
        variable_service::VariableService,
    },
    utils,
};

pub struct CreateEnvironmentParams<'a> {
    pub id: EnvironmentId,
    pub name: String,
    pub abs_path: &'a Path,
    pub color: Option<String>,
    pub order: isize,
}

pub struct EnvironmentLoadParams {
    pub abs_path: PathBuf,
}

pub struct EnvironmentBuilder {
    fs: Arc<dyn FileSystem>,
}

impl EnvironmentBuilder {
    pub fn new(fs: Arc<dyn FileSystem>) -> Self {
        Self { fs }
    }

    pub async fn initialize<'a>(
        self,
        params: CreateEnvironmentParams<'a>,
    ) -> joinerror::Result<()> {
        debug_assert!(params.abs_path.is_absolute());

        let file_name = utils::format_file_name(&params.name);
        let abs_path = params.abs_path.join(&file_name);
        if abs_path.exists() {
            return Err(Error::new::<ErrorEnvironmentAlreadyExists>(
                abs_path.display().to_string(),
            ));
        }

        let file = SourceFile {
            metadata: Block::new(MetadataDecl {
                id: params.id,
                color: params.color,
            }),
            variables: None,
        };
        let content = hcl::to_string(&file).join_err_with::<ErrorFailedToEncode>(|| {
            format!("failed to encode environment file {}", abs_path.display())
        })?;

        self.fs
            .create_file_with(
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

        Ok(())
    }

    pub async fn create<'a, R: AppRuntime>(
        self,
        model_registry: Arc<GlobalModelRegistry>,
        params: CreateEnvironmentParams<'a>,
    ) -> joinerror::Result<Environment<R>> {
        debug_assert!(params.abs_path.is_absolute());

        let file_name = format!(
            "{}.{}",
            sanitize(&params.name),
            constants::ENVIRONMENT_FILE_EXTENSION
        );
        let abs_path = params.abs_path.join(&file_name);
        if abs_path.exists() {
            return Err(Error::new::<ErrorEnvironmentAlreadyExists>(
                abs_path.display().to_string(),
            ));
        }

        let file = SourceFile {
            metadata: Block::new(MetadataDecl {
                id: params.id,
                color: params.color,
            }),
            variables: None,
        };
        let content = hcl::to_string(&file).join_err_with::<ErrorFailedToEncode>(|| {
            format!("failed to encode environment file {}", abs_path.display())
        })?;

        self.fs
            .create_file_with(
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

        let hcl_value = hcl::to_value(file).unwrap(); // TODO: handle errors
        let json_value = serde_json::to_value(hcl_value).unwrap(); // TODO: handle errors
        let abs_path: Arc<Path> = abs_path.into();
        model_registry
            .insert(
                abs_path.clone(),
                ContentModel::Json(JsonModel::new(json_value)),
            )
            .await;

        let metadata_service = MetadataService::new(model_registry.clone());
        let sync_service = Arc::new(SyncService::new(model_registry.clone(), self.fs.clone()));
        let variable_service = VariableService::new(None, sync_service.clone())?;

        Ok(Environment {
            fs: self.fs.clone(),
            model_registry,
            metadata_service,
            sync_service,
            variable_service,
            state: RwLock::new(EnvironmentState {
                abs_path: EnvironmentPath::new(abs_path)?,
            }),
        })
    }

    pub async fn load<R: AppRuntime>(
        self,
        model_registry: Arc<GlobalModelRegistry>,
        params: EnvironmentLoadParams,
    ) -> joinerror::Result<Environment<R>> {
        let abs_path: Arc<Path> = params.abs_path.into();
        debug_assert!(abs_path.is_absolute());

        if !abs_path.exists() {
            return Err(Error::new::<ErrorEnvironmentNotFound>(
                abs_path.display().to_string(),
            ));
        }

        let file: SourceFile = {
            let mut reader = self
                .fs
                .open_file(&abs_path)
                .await
                .join_err_with::<ErrorIo>(|| {
                    format!("failed to open environment file {}", abs_path.display())
                })?;

            let mut buf = String::new();
            reader
                .read_to_string(&mut buf)
                .join_err_with::<ErrorIo>(|| {
                    format!("failed to read environment file {}", abs_path.display())
                })?;
            hcl::from_str(&buf).join_err_with::<ErrorFailedToDecode>(|| {
                format!("failed to decode environment file {}", abs_path.display())
            })?
        };

        let hcl_value = hcl::to_value(file).unwrap(); // TODO: handle errors
        let json_value = serde_json::to_value(hcl_value).unwrap(); // TODO: handle errors

        model_registry
            .insert(
                abs_path.clone(),
                ContentModel::Json(JsonModel::new(json_value)),
            )
            .await;

        let metadata_service = MetadataService::new(model_registry.clone());
        let sync_service = Arc::new(SyncService::new(model_registry.clone(), self.fs.clone()));
        let variable_service = VariableService::new(None, sync_service.clone())?;

        Ok(Environment {
            fs: self.fs.clone(),
            model_registry,
            metadata_service,
            sync_service,
            variable_service,
            state: RwLock::new(EnvironmentState {
                abs_path: EnvironmentPath::new(abs_path)?,
            }),
        })
    }
}
