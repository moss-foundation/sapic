use moss_applib::AppRuntime;

use crate::{Workspace, models::operations::ListChangesOutput};

impl<R: AppRuntime> Workspace<R> {
    pub async fn list_changes(
        &self,
        _ctx: &R::AsyncContext,
    ) -> joinerror::Result<ListChangesOutput> {
        let changes = self
            .collection_service
            .list_changes()
            .await?
            .into_iter()
            .collect();
        Ok(ListChangesOutput { changes })
    }
}
