use std::sync::Arc;

use moss_common::api::OperationResult;
use tauri::Runtime as TauriRuntime;

use crate::{
    app::App, models::operations::DescribeWorkbenchStateOutput,
    services::workspace_service::WorkspaceService,
};

impl<R: TauriRuntime> App<R> {
    pub async fn describe_workbench_state(&self) -> OperationResult<DescribeWorkbenchStateOutput> {
        let workspace_service = self.service::<WorkspaceService<R>>();
        let active_workspace_id = workspace_service.active_workspace_id().await;

        Ok(DescribeWorkbenchStateOutput {
            active_workspace_id,
            prev_workspace_id: None, // TODO: implement
            abs_path: Arc::clone(&self.abs_path),
        })
    }
}
