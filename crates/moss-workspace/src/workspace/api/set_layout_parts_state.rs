use moss_common::api::OperationResult;
use moss_storage::workspace_storage::entities::state_store_entities::{
    EditorPartStateEntity, PanelPartStateEntity, SidebarPartStateEntity,
};
use tauri::{Emitter, Runtime as TauriRuntime};

use crate::{
    models::operations::{SetLayoutPartsStateInput, SetLayoutPartsStateParams},
    workspace::Workspace,
};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn set_layout_parts_state(
        &self,
        input: SetLayoutPartsStateInput,
        params: SetLayoutPartsStateParams,
    ) -> OperationResult<()> {
        let layout_parts_state_store = self.workspace_storage.state_store();

        if let Some(editor_state) = input.editor {
            layout_parts_state_store
                .put_editor_part_state(EditorPartStateEntity::from(editor_state))?;
        }

        if let Some(sidebar_state) = input.sidebar {
            layout_parts_state_store
                .put_sidebar_part_state(SidebarPartStateEntity::from(sidebar_state))?;
        }

        if let Some(panel_state) = input.panel {
            layout_parts_state_store
                .put_panel_part_state(PanelPartStateEntity::from(panel_state))?;
        }

        Ok(())
    }
}
