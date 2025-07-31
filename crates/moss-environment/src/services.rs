pub mod metadata_service;
pub mod storage_service;
pub mod sync_service;
pub mod variable_service;

use json_patch::PatchOperation;
use moss_applib::AppRuntime;
use serde_json::Value as JsonValue;

pub(super) trait AnyStorageService<R: AppRuntime> {}

pub(super) trait AnySyncService<R: AppRuntime> {
    async fn apply(&self, patches: &[PatchOperation]) -> joinerror::Result<JsonValue>;
    async fn save(&self) -> joinerror::Result<()>;
}

pub(super) trait AnyMetadataService<R: AppRuntime> {
    #[allow(dead_code)]
    async fn apply(&self) -> joinerror::Result<()>;
}

pub(super) trait AnyVariableService<R: AppRuntime> {}
