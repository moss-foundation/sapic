use std::{path::Path, sync::Arc};

use joinerror::Error;
use moss_applib::{AppService, ServiceMarker};
use moss_fs::model_registry::GlobalModelRegistry;

use crate::{configuration::MetadataDecl, services::AnyMetadataService};

pub struct MetadataService {
    model_registry: Arc<GlobalModelRegistry>,
}

impl MetadataService {
    pub fn new(model_registry: Arc<GlobalModelRegistry>) -> Self {
        Self { model_registry }
    }
}

impl AppService for MetadataService {}
impl ServiceMarker for MetadataService {}

impl AnyMetadataService for MetadataService {
    async fn describe(&self, abs_path: &Path) -> joinerror::Result<MetadataDecl> {
        let model = self
            .model_registry
            .get(abs_path)
            .await
            .ok_or_else(|| Error::new::<()>("model not found"))?;

        let json_value = model
            .as_json()
            .ok_or_else(|| Error::new::<()>("model is not a json model"))?
            .value()
            .clone();

        let metadata_value = json_value
            .get("metadata")
            .ok_or_else(|| Error::new::<()>("metadata field not found in json value"))?
            .clone();

        let metadata_decl =
            serde_json::from_value::<MetadataDecl>(metadata_value).map_err(|err| {
                Error::new::<()>(format!(
                    "failed to convert json value to metadata declaration: {}",
                    err
                ))
            })?;

        Ok(metadata_decl)
    }
}
