use joinerror::Error;
use json_patch::PatchOperation;
use moss_applib::{AppRuntime, ServiceMarker};
use moss_fs::{CreateOptions, FileSystem, model_registry::GlobalModelRegistry};
use serde_json::Value as JsonValue;
use std::{path::Path, sync::Arc};
use tokio::sync::RwLock;

use crate::{configuration::EnvironmentFile, services::AnySyncService};

struct ServiceState {
    uri: String,
}

pub struct SyncService {
    models: GlobalModelRegistry,
    fs: Arc<dyn FileSystem>,
    state: Arc<RwLock<ServiceState>>,
}

impl ServiceMarker for SyncService {}

impl<R: AppRuntime> AnySyncService<R> for SyncService {
    async fn apply(&self, patches: &[PatchOperation]) -> joinerror::Result<()> {
        let state = self.state.read().await;
        let value = self
            .models
            .with_model_mut(&state.uri, |model| {
                let model = model.as_json_mut().expect("model is not a json model");
                model
                    .apply(patches)
                    .map_err(|err| Error::new::<()>(format!("failed to apply patches: {}", err)))?;

                Ok::<JsonValue, Error>(model.value().clone())
            })
            .await
            .ok_or_else(|| Error::new::<()>("model not found"))??;

        // TODO: we'll handle file system synchronization in the background a bit later,
        // so we can respond to the frontend faster.
        {
            // HACK: Right now, we have to convert HCL to JSON and then back to HCL,
            // because at the moment we can’t apply patches directly to HCL.
            // Once we implement that mechanism, we'll be able to patch HCL directly
            // and get rid of the intermediate conversions to JSON.

            let value_str = serde_json::to_string(&value).map_err(|err| {
                Error::new::<()>(format!("failed to convert json to string: {}", err))
            })?;

            dbg!(&value_str);

            let hcl_value = hcl::from_str::<EnvironmentFile>(&value_str).map_err(|err| {
                Error::new::<()>(format!("failed to convert json to hcl: {}", err))
            })?;

            let hcl_value_str = hcl::to_string(&hcl_value).map_err(|err| {
                Error::new::<()>(format!("failed to convert hcl to string: {}", err))
            })?;

            self.fs
                .create_file_with(
                    &Path::new(&state.uri),
                    hcl_value_str.as_bytes(),
                    CreateOptions {
                        overwrite: true,
                        ignore_if_exists: false,
                    },
                )
                .await
                .map_err(|err| Error::new::<()>(format!("failed to write file: {}", err)))?;
        }

        Ok(())
    }
}

impl SyncService {
    pub fn new(uri: String, models: GlobalModelRegistry, fs: Arc<dyn FileSystem>) -> Self {
        Self {
            models,
            fs,
            state: Arc::new(RwLock::new(ServiceState { uri })),
        }
    }
}
