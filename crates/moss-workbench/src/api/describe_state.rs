use std::sync::Arc;

use moss_common::api::OperationResult;
use tauri::Runtime as TauriRuntime;

use crate::{models::operations::DescribeWorkbenchStateOutput, workbench::Workbench};

impl<R: TauriRuntime> Workbench<R> {
    pub async fn describe_state(&self) -> OperationResult<DescribeWorkbenchStateOutput> {
        let active_workspace_id = self
            .active_workspace
            .load()
            .as_ref()
            .map(|workspace| workspace.id);

        Ok(DescribeWorkbenchStateOutput {
            active_workspace_id,
            prev_workspace_id: None, // TODO: implement
            abs_path: Arc::clone(&self.options.workspaces_abs_path),
        })
    }
}
