use moss_common::api::OperationResult;
use tauri::Runtime as TauriRuntime;

use crate::{
    app::App,
    context::{AnyAppContext, ctxkeys},
    models::operations::DescribeWorkbenchStateOutput,
};

impl<R: TauriRuntime> App<R> {
    pub async fn describe_workbench_state<C: AnyAppContext<R>>(
        &self,
        ctx: &C,
    ) -> OperationResult<DescribeWorkbenchStateOutput> {
        let workspace_id = ctx.value::<ctxkeys::WorkspaceId>().map(|id| **id);

        Ok(DescribeWorkbenchStateOutput {
            active_workspace_id: workspace_id,
            prev_workspace_id: None, // TODO: implement
        })
    }
}
