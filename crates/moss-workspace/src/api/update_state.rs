use moss_applib::AppRuntime;

use crate::{models::operations::UpdateStateInput, workspace::Workspace};

// DEPRECATED
impl<R: AppRuntime> Workspace<R> {
    pub async fn update_state(
        &self,
        ctx: &R::AsyncContext,
        input: UpdateStateInput,
    ) -> joinerror::Result<()> {
        match input {
            UpdateStateInput::UpdateEditorPartState(editor_part_state) => {
                self.layout_service
                    .update_editor_layout(ctx, editor_part_state)
                    .await?
            }
            UpdateStateInput::UpdateSidebarPartState(sidebar_part_state) => {
                self.layout_service
                    .update_sidebar_layout(ctx, sidebar_part_state)
                    .await?
            }
            UpdateStateInput::UpdatePanelPartState(panel_part_state) => {
                self.layout_service
                    .update_panel_layout(ctx, panel_part_state)
                    .await?
            }
            UpdateStateInput::UpdateActivitybarPartState(activitybar_part_state) => {
                self.layout_service
                    .update_activitybar_layout(ctx, activitybar_part_state)
                    .await?
            }
        }

        Ok(())
    }
}
