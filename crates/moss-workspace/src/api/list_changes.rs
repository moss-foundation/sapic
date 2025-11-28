use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;

use crate::{Workspace, models::operations::ListChangesOutput};

impl Workspace {
    pub async fn list_changes<R: AppRuntime>(
        &self,
        _ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
    ) -> joinerror::Result<ListChangesOutput> {
        let changes = self
            .project_service
            .list_changes::<R>(app_delegate)
            .await?
            .into_iter()
            .collect();
        Ok(ListChangesOutput { changes })
    }
}
