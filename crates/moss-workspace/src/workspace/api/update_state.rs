use moss_common::api::OperationResult;
use moss_storage::workspace_storage::entities::state_store_entities::{
    EditorPartStateEntity, PanelPartStateEntity, SidebarPartStateEntity,
};
use tauri::Runtime as TauriRuntime;

use crate::{models::operations::UpdateStateInput, workspace::Workspace};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn update_state(&self, input: UpdateStateInput) -> OperationResult<()> {
        let state_store = self.workspace_storage.state_store();

        match input {
            UpdateStateInput::UpdateEditorPartState(editor_part_state) => {
                state_store
                    .put_editor_part_state(EditorPartStateEntity::from(editor_part_state))?;

                Ok(())
            }
            UpdateStateInput::UpdateSidebarPartState(sidebar_part_state) => {
                state_store
                    .put_sidebar_part_state(SidebarPartStateEntity::from(sidebar_part_state))?;

                Ok(())
            }
            UpdateStateInput::UpdatePanelPartState(panel_part_state) => {
                state_store.put_panel_part_state(PanelPartStateEntity::from(panel_part_state))?;

                Ok(())
            }
        }
    }
}
