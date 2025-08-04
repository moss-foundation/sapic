pub mod metadata_service;
pub mod storage_service;
pub mod sync_service;
pub mod variable_service;

use std::path::Path;

use json_patch::PatchOperation;
use moss_applib::AppRuntime;
use serde_json::Value as JsonValue;

use crate::configuration::MetadataDecl;

#[allow(unused)]
pub(super) trait AnyStorageService<R: AppRuntime> {}

pub(super) trait AnySyncService {
    async fn apply(&self, path: &Path, patches: &[PatchOperation]) -> joinerror::Result<JsonValue>;
    async fn save(&self, path: &Path) -> joinerror::Result<()>;
}

pub(super) trait AnyMetadataService {
    // INFO: maybe we should use a different type to separate metadata type from the configuration types
    async fn describe(&self, abs_path: &Path) -> joinerror::Result<MetadataDecl>;

    #[allow(dead_code)]
    async fn apply(&self) -> joinerror::Result<()>;
}

#[allow(unused)]
pub(super) trait AnyVariableService<R: AppRuntime> {}
