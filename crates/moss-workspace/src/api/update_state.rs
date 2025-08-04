use moss_applib::AppRuntime;
use moss_common::api::OperationResult;

use crate::{models::operations::UpdateStateInput, workspace::Workspace};

impl<R: AppRuntime> Workspace<R> {
    pub async fn update_state(
        &self,
        ctx: &R::AsyncContext,
        input: UpdateStateInput,
    ) -> OperationResult<()> {
        match input {
            UpdateStateInput::UpdateEditorPartState(editor_part_state) => {
                self.layout_service
                    .put_editor_layout_state(ctx, editor_part_state)
                    .await?
            }
            UpdateStateInput::UpdateSidebarPartState(sidebar_part_state) => {
                self.layout_service
                    .put_sidebar_layout_state(ctx, sidebar_part_state)
                    .await?
            }
            UpdateStateInput::UpdatePanelPartState(panel_part_state) => {
                self.layout_service
                    .put_panel_layout_state(ctx, panel_part_state)
                    .await?
            }
            UpdateStateInput::UpdateActivitybarPartState(activitybar_part_state) => {
                self.layout_service
                    .put_activitybar_layout_state(ctx, activitybar_part_state)
                    .await?
            }
        }

        Ok(())
    }
}
