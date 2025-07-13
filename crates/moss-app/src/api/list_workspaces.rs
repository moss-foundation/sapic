use moss_applib::AppRuntime;
use moss_common::api::OperationResult;

use crate::{
    app::App,
    models::{operations::ListWorkspacesOutput, types::WorkspaceInfo},
    services::workspace_service::WorkspaceService,
};

impl<R: AppRuntime> App<R> {
    pub async fn list_workspaces(
        &self,
        _ctx: &R::AsyncContext,
    ) -> OperationResult<ListWorkspacesOutput> {
        let workspace_service = self.services.get::<WorkspaceService<R>>();
        let workspaces = workspace_service.list_workspaces().await?;
        let workspaces = workspaces
            .into_iter()
            .map(|item| WorkspaceInfo {
                id: item.id,
                name: item.name.clone(),
                last_opened_at: item.last_opened_at,
                active: item.active,
                abs_path: item.abs_path,
            })
            .collect();

        Ok(ListWorkspacesOutput(workspaces))
    }
}
