use moss_common::api::OperationResult;
use moss_db::primitives::AnyValue;
use moss_storage::{
    storage::operations::PutItem,
    workspace_storage::entities::state_store_entities::{
        ActivitybarPartStateEntity, EditorPartStateEntity, PanelPartStateEntity,
        SidebarPartStateEntity,
    },
};
use tauri::Runtime as TauriRuntime;

use crate::{
    models::operations::UpdateStateInput,
    storage::segments::{
        PART_ACTIVITYBAR_SEGKEY, PART_EDITOR_SEGKEY, PART_PANEL_SEGKEY, PART_SIDEBAR_SEGKEY,
    },
    workspace::Workspace,
};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn update_state(&self, input: UpdateStateInput) -> OperationResult<()> {
        let item_store = self.workspace_storage.item_store();

        match input {
            UpdateStateInput::UpdateEditorPartState(editor_part_state) => {
                let value = AnyValue::serialize(&EditorPartStateEntity::from(editor_part_state))?;
                PutItem::put(
                    item_store.as_ref(),
                    PART_EDITOR_SEGKEY.to_segkey_buf(),
                    value,
                )?;
            }
            UpdateStateInput::UpdateSidebarPartState(sidebar_part_state) => {
                let value = AnyValue::serialize(&SidebarPartStateEntity::from(sidebar_part_state))?;
                PutItem::put(
                    item_store.as_ref(),
                    PART_SIDEBAR_SEGKEY.to_segkey_buf(),
                    value,
                )?;
            }
            UpdateStateInput::UpdatePanelPartState(panel_part_state) => {
                let value = AnyValue::serialize(&PanelPartStateEntity::from(panel_part_state))?;
                PutItem::put(
                    item_store.as_ref(),
                    PART_PANEL_SEGKEY.to_segkey_buf(),
                    value,
                )?;
            }
            UpdateStateInput::UpdateActivitybarPartState(activitybar_part_state) => {
                let value =
                    AnyValue::serialize(&ActivitybarPartStateEntity::from(activitybar_part_state))?;
                PutItem::put(
                    item_store.as_ref(),
                    PART_ACTIVITYBAR_SEGKEY.to_segkey_buf(),
                    value,
                )?;
            }
        }

        Ok(())
    }
}
