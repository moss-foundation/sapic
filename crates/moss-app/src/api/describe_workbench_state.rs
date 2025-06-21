use std::sync::Arc;

use moss_common::api::OperationResult;
use tauri::Runtime as TauriRuntime;

use crate::{app::App, models::operations::DescribeWorkbenchStateOutput};

impl<R: TauriRuntime> App<R> {
    pub async fn describe_workbench_state(&self) -> OperationResult<DescribeWorkbenchStateOutput> {
        let active_workspace_id = self.active_workspace_id().await;

        Ok(DescribeWorkbenchStateOutput {
            active_workspace_id,
            prev_workspace_id: None, // TODO: implement
            abs_path: Arc::clone(&self.abs_path),
        })
    }
}
