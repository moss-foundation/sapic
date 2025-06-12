use std::collections::HashMap;

use moss_common::api::OperationResult;
use moss_db::primitives::AnyValue;
use moss_storage::{
    storage::operations::TransactionalPutItem,
    workspace_storage::entities::state_store_entities::{
        EditorGridStateEntity, EditorPanelStateEntity,
    },
};
use tauri::Runtime as TauriRuntime;

use crate::{
    models::{
        operations::UpdateStateInput,
        types::{
            ActivitybarPartStateInfo, EditorPartStateInfo, PanelPartStateInfo, SidebarPartStateInfo,
        },
    },
    storage::segments::{
        PART_ACTIVITYBAR_SEGKEY, PART_EDITOR_SEGKEY, PART_PANEL_SEGKEY, PART_SIDEBAR_SEGKEY,
    },
    workspace::Workspace,
};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn update_state(&self, input: UpdateStateInput) -> OperationResult<()> {
        match input {
            UpdateStateInput::UpdateEditorPartState(editor_part_state) => {
                self.update_editor_part_state(editor_part_state)?
            }
            UpdateStateInput::UpdateSidebarPartState(sidebar_part_state) => {
                self.update_sidebar_part_state(sidebar_part_state)?
            }
            UpdateStateInput::UpdatePanelPartState(panel_part_state) => {
                self.update_panel_part_state(panel_part_state)?
            }
            UpdateStateInput::UpdateActivitybarPartState(activitybar_part_state) => {
                self.update_activitybar_part_state(activitybar_part_state)?
            }
        }

        Ok(())
    }

    fn update_editor_part_state(&self, part_state: EditorPartStateInfo) -> OperationResult<()> {
        let item_store = self.storage.item_store();
        let mut txn = self.storage.begin_write()?;

        let value = AnyValue::serialize(&EditorGridStateEntity::from(part_state.grid))?;

        TransactionalPutItem::put(
            item_store.as_ref(),
            &mut txn,
            PART_EDITOR_SEGKEY.join("grid"),
            value,
        )?;

        let value = AnyValue::serialize::<HashMap<String, EditorPanelStateEntity>>(
            &part_state
                .panels
                .into_iter()
                .map(|(key, panel)| (key, panel.into()))
                .collect(),
        )?;
        TransactionalPutItem::put(
            item_store.as_ref(),
            &mut txn,
            PART_EDITOR_SEGKEY.join("panels"),
            value,
        )?;

        let value = AnyValue::serialize(&part_state.active_group)?;
        TransactionalPutItem::put(
            item_store.as_ref(),
            &mut txn,
            PART_EDITOR_SEGKEY.join("activeGroup"),
            value,
        )?;

        Ok(txn.commit()?)
    }

    fn update_sidebar_part_state(&self, part_state: SidebarPartStateInfo) -> OperationResult<()> {
        let item_store = self.storage.item_store();
        let mut txn = self.storage.begin_write()?;

        let value = AnyValue::serialize(&part_state.position)?;
        TransactionalPutItem::put(
            item_store.as_ref(),
            &mut txn,
            PART_SIDEBAR_SEGKEY.join("position"),
            value,
        )?;

        let value = AnyValue::serialize(&part_state.size)?;
        TransactionalPutItem::put(
            item_store.as_ref(),
            &mut txn,
            PART_SIDEBAR_SEGKEY.join("size"),
            value,
        )?;

        let value = AnyValue::serialize(&part_state.visible)?;
        TransactionalPutItem::put(
            item_store.as_ref(),
            &mut txn,
            PART_SIDEBAR_SEGKEY.join("visible"),
            value,
        )?;

        Ok(txn.commit()?)
    }

    fn update_panel_part_state(&self, part_state: PanelPartStateInfo) -> OperationResult<()> {
        let item_store = self.storage.item_store();
        let mut txn = self.storage.begin_write()?;

        let value = AnyValue::serialize(&part_state.visible)?;
        TransactionalPutItem::put(
            item_store.as_ref(),
            &mut txn,
            PART_PANEL_SEGKEY.join("visible"),
            value,
        )?;

        let value = AnyValue::serialize(&part_state.size)?;
        TransactionalPutItem::put(
            item_store.as_ref(),
            &mut txn,
            PART_PANEL_SEGKEY.join("size"),
            value,
        )?;

        Ok(txn.commit()?)
    }

    fn update_activitybar_part_state(
        &self,
        part_state: ActivitybarPartStateInfo,
    ) -> OperationResult<()> {
        let item_store = self.storage.item_store();
        let mut txn = self.storage.begin_write()?;

        let value = AnyValue::serialize(&part_state.last_active_container_id)?;
        TransactionalPutItem::put(
            item_store.as_ref(),
            &mut txn,
            PART_ACTIVITYBAR_SEGKEY.join("lastActiveContainerId"),
            value,
        )?;

        let value = AnyValue::serialize(&part_state.position)?;
        TransactionalPutItem::put(
            item_store.as_ref(),
            &mut txn,
            PART_ACTIVITYBAR_SEGKEY.join("position"),
            value,
        )?;

        // TODO: Handle items update

        Ok(txn.commit()?)
    }
}
