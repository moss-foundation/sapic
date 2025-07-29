pub mod metadata_service;
pub mod storage_service;
pub mod sync_service;
pub mod variable_service;

use json_patch::PatchOperation;
use moss_applib::{AppRuntime, AppService};

pub(super) trait AnyStorageService<R: AppRuntime>: AppService {}

pub(super) trait AnySyncService<R: AppRuntime>: AppService {
    async fn apply(&self, patches: &[PatchOperation]) -> joinerror::Result<()>;
}

pub(super) trait AnyMetadataService<R: AppRuntime>: AppService {
    async fn apply(&self) -> joinerror::Result<()>;
}
