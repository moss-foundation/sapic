use moss_common::api::OperationResult;
use tauri::Runtime as TauriRuntime;

use crate::{
    models::operations::UpdateStateInput, services::layout_service::LayoutService,
    workspace::Workspace,
};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn update_state(&self, input: UpdateStateInput) -> OperationResult<()> {
        let layout = self.services.get::<LayoutService>();

        match input {
            UpdateStateInput::UpdateEditorPartState(editor_part_state) => {
                layout.put_editor_layout_state(editor_part_state)?
            }
            UpdateStateInput::UpdateSidebarPartState(sidebar_part_state) => {
                layout.put_sidebar_layout_state(sidebar_part_state)?
            }
            UpdateStateInput::UpdatePanelPartState(panel_part_state) => {
                layout.put_panel_layout_state(panel_part_state)?
            }
            UpdateStateInput::UpdateActivitybarPartState(activitybar_part_state) => {
                layout.put_activitybar_layout_state(activitybar_part_state)?
            }
        }

        Ok(())
    }
}
