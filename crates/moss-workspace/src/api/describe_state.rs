use moss_common::api::{OperationError, OperationResult};
use moss_db::common::DatabaseError;
use moss_storage::workspace_storage::entities::state_store_entities::{
    EditorPartStateEntity, PanelPartStateEntity, SidebarPartStateEntity,
};
use tauri::Runtime as TauriRuntime;

use crate::models::types::{EditorPartState, PanelPartState, SidebarPartState};
use crate::{models::operations::DescribeStateOutput, workspace::Workspace};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn describe_state(&self) -> OperationResult<DescribeStateOutput> {
        let state_store = self.workspace_storage.state_store();

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
            state_store.get_editor_part_state(),
            |entity: EditorPartStateEntity| EditorPartState::from(entity),
        )?;

        let sidebar = to_option(
            state_store.get_sidebar_part_state(),
            |entity: SidebarPartStateEntity| SidebarPartState::from(entity),
        )?;

        let panel = to_option(
            state_store.get_panel_part_state(),
            |entity: PanelPartStateEntity| PanelPartState::from(entity),
        )?;

        Ok(DescribeStateOutput {
            editor,
            sidebar,
            panel,
        })
    }
}
