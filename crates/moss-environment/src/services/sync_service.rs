use joinerror::Error;
use json_patch::PatchOperation;
use moss_applib::{AppService, ServiceMarker};
use moss_fs::{CreateOptions, FileSystem, model_registry::GlobalModelRegistry};
use serde_json::Value as JsonValue;
use std::{path::Path, sync::Arc};

use crate::{configuration::SourceFile, services::AnySyncService};

pub struct SyncService {
    models: Arc<GlobalModelRegistry>,
    fs: Arc<dyn FileSystem>,
}

impl AppService for SyncService {}
impl ServiceMarker for SyncService {}

impl AnySyncService for SyncService {
    async fn save(&self, abs_path: &Path) -> joinerror::Result<()> {
        dbg!(&abs_path);

        let model = self
            .models
            .get(abs_path)
            .await
            .ok_or_else(|| Error::new::<()>("model not found"))?;

        let json_value = model
            .as_json()
            .ok_or_else(|| Error::new::<()>("model is not a json model"))?
            .value()
            .clone();

        let hcl_value =
            serde_json::from_value::<SourceFile>(json_value.clone()).map_err(|err| {
                Error::new::<()>(format!(
                    "failed to convert json value to structure: {}",
                    err
                ))
            })?;

        let content = hcl::to_string(&hcl_value).map_err(|err| {
            Error::new::<()>(format!(
                "failed to convert structure to hcl string: {}",
                err
            ))
        })?;

        self.fs
            .create_file_with(
                &abs_path,
                content.as_bytes(),
                CreateOptions {
                    overwrite: true,
                    ignore_if_exists: false,
                },
            )
            .await
            .map_err(|err| Error::new::<()>(format!("failed to write file: {}", err)))?;

        Ok(())
    }

    async fn apply(&self, path: &Path, patches: &[PatchOperation]) -> joinerror::Result<JsonValue> {
        dbg!(&path);

        let json_value = self
            .models
            .with_model_mut(path, |model| {
                let model = model.as_json_mut().expect("model is not a json model");
                model
                    .apply(patches)
                    .map_err(|err| Error::new::<()>(format!("failed to apply patches: {}", err)))?;

                Ok::<JsonValue, Error>(model.value().clone())
            })
            .await
            .ok_or_else(|| Error::new::<()>("model not found"))??;

        Ok(json_value)
    }
}

impl SyncService {
    pub fn new(model_registry: Arc<GlobalModelRegistry>, fs: Arc<dyn FileSystem>) -> Self {
        Self {
            models: model_registry,
            fs,
        }
    }
}
