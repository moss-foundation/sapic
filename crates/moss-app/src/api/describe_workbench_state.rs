use moss_common::api::OperationResult;
use tauri::Runtime as TauriRuntime;

use crate::{
    app::App,
    context::AnyAppContext,
    models::{operations::DescribeWorkbenchStateOutput, primitives::WorkspaceId},
};

impl<R: TauriRuntime> App<R> {
    pub async fn describe_workbench_state<C: AnyAppContext<R>>(
        &self,
        ctx: &C,
    ) -> OperationResult<DescribeWorkbenchStateOutput> {
        let workspace_id = ctx.value::<WorkspaceId>().map(|id| (*id).clone());

        Ok(DescribeWorkbenchStateOutput {
            active_workspace_id: workspace_id,
            prev_workspace_id: None, // TODO: implement
        })
    }
}
