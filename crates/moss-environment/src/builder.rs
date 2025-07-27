use joinerror::{Error, ResultExt};
use moss_applib::{AppRuntime, ServiceMarker, providers::ServiceMap};
use moss_fs::{CreateOptions, FileSystem, FsResultExt, model_registry::GlobalModelRegistry};
use moss_hcl::{Block, HclResultExt};
use moss_patch::{Model, json::JsonModel};
use moss_text::sanitized::sanitize;
use std::{any::TypeId, path::PathBuf, sync::Arc};

use crate::{
    configuration::{EnvironmentFile, Metadata},
    constants,
    environment::Environment,
    errors::{
        ErrorEnvironmentAlreadyExists, ErrorEnvironmentNotFound, ErrorFailedToDecode,
        ErrorFailedToEncode, ErrorIo,
    },
    models::primitives::EnvironmentId,
};

pub struct EnvironmentCreateParams {
    pub name: String,
    pub abs_path: PathBuf,
    pub color: Option<String>,
}

pub struct EnvironmentLoadParams {
    pub abs_path: PathBuf,
}

pub struct EnvironmentBuilder {
    fs: Arc<dyn FileSystem>,
    models: GlobalModelRegistry,
    services: ServiceMap,
}

impl EnvironmentBuilder {
    pub fn new(fs: Arc<dyn FileSystem>, models: GlobalModelRegistry) -> Self {
        Self {
            fs,
            models,
            services: Default::default(),
        }
    }

    pub fn with_service<T: ServiceMarker + Send + Sync>(
        mut self,
        service: impl Into<Arc<T>>,
    ) -> Self {
        self.services.insert(TypeId::of::<T>(), service.into());
        self
    }

    pub async fn create<R: AppRuntime>(
        self,
        params: EnvironmentCreateParams,
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

        let file = EnvironmentFile {
            metadata: Block::new(Metadata {
                id: EnvironmentId::new(),
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

        let hcl_value = hcl::to_value(file).unwrap();
        let json_value = serde_json::to_value(hcl_value).unwrap();
        self.models
            .add(
                abs_path.to_string_lossy().to_string(),
                Model::Json(JsonModel::new(json_value)),
            )
            .await;

        Ok(Environment::new(abs_path.into(), self.services.into()))
    }

    pub async fn load<R: AppRuntime>(
        self,
        params: EnvironmentLoadParams,
    ) -> joinerror::Result<Environment<R>> {
        let abs_path = params.abs_path;
        debug_assert!(abs_path.is_absolute());

        if !abs_path.exists() {
            return Err(Error::new::<ErrorEnvironmentNotFound>(
                abs_path.display().to_string(),
            ));
        }

        let _file: EnvironmentFile = {
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

        Ok(Environment::new(abs_path.into(), self.services.into()))
    }
}
