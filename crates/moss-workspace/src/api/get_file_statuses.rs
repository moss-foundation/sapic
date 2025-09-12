use crate::{Workspace, models::operations::GetFileStatusesOutput};
use moss_applib::AppRuntime;

impl<R: AppRuntime> Workspace<R> {
    pub async fn get_file_statuses(
        &self,
        _ctx: &R::AsyncContext,
    ) -> joinerror::Result<GetFileStatusesOutput> {
        let statuses = self
            .collection_service
            .get_file_statuses()
            .await?
            .into_iter()
            .collect();
        Ok(GetFileStatusesOutput { statuses })
    }
}
