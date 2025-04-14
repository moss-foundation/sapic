use moss_collection::collection::OperationError;
use moss_db::common::DatabaseError;
use tauri::Runtime as TauriRuntime;

use crate::models::entities::{
    EditorPartStateEntity, PanelPartStateEntity, SidebarPartStateEntity,
};
use crate::models::types::{EditorPartState, PanelPartState, SidebarPartState};
use crate::{models::operations::DescribeLayoutPartsStateOutput, workspace::Workspace};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn describe_layout_parts_state(
        &self,
    ) -> Result<DescribeLayoutPartsStateOutput, OperationError> {
        let layout_parts_state_store = self.state_db_manager.layout_parts_state_store();

        fn to_option<T, U>(
            result: Result<T, DatabaseError>,
            convert_fn: impl FnOnce(T) -> U,
        ) -> Result<Option<U>, OperationError> {
            match result {
                Ok(entity) => Ok(Some(convert_fn(entity))),
                Err(DatabaseError::NotFound { .. }) => Ok(None),
                Err(err) => Err(err.into()),
            }
        }

        let editor = to_option(
            layout_parts_state_store.get_editor_part_state(),
            |entity: EditorPartStateEntity| EditorPartState {
                grid: entity.grid,
                panels: entity.panels,
                active_group: entity.active_group,
            },
        )?;

        let sidebar = to_option(
            layout_parts_state_store.get_sidebar_part_state(),
            |entity: SidebarPartStateEntity| SidebarPartState {
                preferred_size: entity.preferred_size,
                is_visible: entity.is_visible,
            },
        )?;

        let panel = to_option(
            layout_parts_state_store.get_panel_part_state(),
            |entity: PanelPartStateEntity| PanelPartState {
                preferred_size: entity.preferred_size,
                is_visible: entity.is_visible,
            },
        )?;

        Ok(DescribeLayoutPartsStateOutput {
            editor,
            sidebar,
            panel,
        })
    }
}
