use moss_applib::{AppRuntime, AppService, ServiceMarker};

use crate::services::AnyMetadataService;

pub struct MetadataService {}

impl AppService for MetadataService {}
impl ServiceMarker for MetadataService {}

impl<R: AppRuntime> AnyMetadataService<R> for MetadataService {
    async fn apply(&self) -> joinerror::Result<()> {
        todo!()
    }
}
