pub mod metadata_service;
pub mod storage_service;
pub mod sync_service;
pub mod variable_service;

use std::path::Path;

use json_patch::PatchOperation;
use moss_applib::AppRuntime;
use serde_json::Value as JsonValue;

pub(super) trait AnyStorageService<R: AppRuntime> {}

pub(super) trait AnySyncService {
    async fn apply(&self, path: &Path, patches: &[PatchOperation]) -> joinerror::Result<JsonValue>;
    async fn save(&self, path: &Path) -> joinerror::Result<()>;
}

pub(super) trait AnyMetadataService<R: AppRuntime> {
    #[allow(dead_code)]
    async fn apply(&self) -> joinerror::Result<()>;
}

pub(super) trait AnyVariableService<R: AppRuntime> {}
