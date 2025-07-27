pub mod storage_service;
pub mod sync_service;
pub mod variable_service;

use json_patch::PatchOperation;
use moss_applib::{AppRuntime, ServiceMarker};

pub(super) trait AnyStorageService<R: AppRuntime>:
    Send + Sync + ServiceMarker + 'static
{
}

pub(super) trait AnySyncService<R: AppRuntime>:
    Send + Sync + ServiceMarker + 'static
{
    async fn apply(&self, patches: &[PatchOperation]) -> joinerror::Result<()>;
}
