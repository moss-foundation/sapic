use tauri::Runtime as TauriRuntime;

use crate::{
    models::{
        entities::{EditorPartStateEntity, PanelPartStateEntity, SidebarPartStateEntity},
        operations::SetLayoutPartsStateInput,
    },
    workspace::{OperationError, Workspace},
};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn set_layout_parts_state(
        &self,
        input: SetLayoutPartsStateInput,
    ) -> Result<(), OperationError> {
        let layout_parts_state_store = self.state_db_manager.layout_parts_state_store();

        if let Some(editor_state) = input.editor {
            layout_parts_state_store.put_editor_part_state(EditorPartStateEntity {
                grid: editor_state.grid,
                panels: editor_state.panels,
                active_group: editor_state.active_group,
            })?;
        }

        if let Some(sidebar_state) = input.sidebar {
            layout_parts_state_store.put_sidebar_part_state(SidebarPartStateEntity {
                preferred_size: sidebar_state.preferred_size,
                is_visible: sidebar_state.is_visible,
            })?;
        }

        if let Some(panel_state) = input.panel {
            layout_parts_state_store.put_panel_part_state(PanelPartStateEntity {
                preferred_size: panel_state.preferred_size,
                is_visible: panel_state.is_visible,
            })?;
        }

        Ok(())
    }
}
