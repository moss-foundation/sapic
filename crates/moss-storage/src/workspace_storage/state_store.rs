use anyhow::Result;
use moss_db::{
    bincode_table::BincodeTable,
    common::{AnyEntity, DatabaseError},
    DatabaseClient, ReDbClient,
};
use serde::de::DeserializeOwned;

use super::{
    entities::state_store_entities::{
        EditorPartStateEntity, PanelPartStateEntity, SidebarPartStateEntity,
    },
    StateStore, StateStoreTable,
};

const WORKBENCH_PARTS_EDITOR_GRID_STATE_KEY: &str = "workbench.parts.editor.grid";
const WORKBENCH_PARTS_EDITOR_PANELS_STATE_KEY: &str = "workbench.parts.editor.panels";
const WORKBENCH_PARTS_PANEL_STATE_KEY: &str = "workbench.parts.panel";
const WORKBENCH_PARTS_SIDEBAR_STATE_KEY: &str = "workbench.parts.sidebar";
const WORKBENCH_PARTS_EDITOR_ACTIVE_GROUP_STATE_KEY: &str = "workbench.parts.editor.activeGroup";

#[rustfmt::skip]
pub(in crate::workspace_storage) const PARTS_STATE: BincodeTable<String, AnyEntity> = BincodeTable::new("parts_state");

pub struct StateStoreImpl {
    client: ReDbClient,
    table: StateStoreTable<'static>,
}

impl StateStoreImpl {
    pub fn new(client: ReDbClient) -> Self {
        Self {
            client,
            table: PARTS_STATE,
        }
    }

    fn get_by_key<T: DeserializeOwned>(&self, key: String) -> Result<T, DatabaseError> {
        let txn = self.client.begin_read()?;
        let data = self.table.read(&txn, key)?;

        Ok(serde_json::from_slice(&data)?)
    }
}

impl StateStore for StateStoreImpl {
    fn get_sidebar_part_state(&self) -> Result<SidebarPartStateEntity, DatabaseError> {
        self.get_by_key(WORKBENCH_PARTS_SIDEBAR_STATE_KEY.to_string())
    }

    fn get_panel_part_state(&self) -> Result<PanelPartStateEntity, DatabaseError> {
        self.get_by_key(WORKBENCH_PARTS_PANEL_STATE_KEY.to_string())
    }

    fn get_editor_part_state(&self) -> Result<EditorPartStateEntity, DatabaseError> {
        let txn = self.client.begin_read()?;

        Ok(EditorPartStateEntity {
            grid: {
                let data = self
                    .table
                    .read(&txn, WORKBENCH_PARTS_EDITOR_GRID_STATE_KEY.to_string())?;

                serde_json::from_slice(&data)?
            },
            panels: {
                let data = self
                    .table
                    .read(&txn, WORKBENCH_PARTS_EDITOR_PANELS_STATE_KEY.to_string())?;

                serde_json::from_slice(&data)?
            },
            active_group: {
                let data = self.table.read(
                    &txn,
                    WORKBENCH_PARTS_EDITOR_ACTIVE_GROUP_STATE_KEY.to_string(),
                )?;
                serde_json::from_slice(&data)?
            },
        })
    }

    fn put_sidebar_part_state(&self, state: SidebarPartStateEntity) -> Result<(), DatabaseError> {
        let mut txn = self.client.begin_write()?;

        let data = serde_json::to_vec(&state)?;

        self.table.insert(
            &mut txn,
            WORKBENCH_PARTS_SIDEBAR_STATE_KEY.to_string(),
            &data,
        )?;
        txn.commit()?;
        Ok(())
    }

    fn put_panel_part_state(&self, state: PanelPartStateEntity) -> Result<(), DatabaseError> {
        let mut txn = self.client.begin_write()?;

        let data = serde_json::to_vec(&state)?;
        self.table
            .insert(&mut txn, WORKBENCH_PARTS_PANEL_STATE_KEY.to_string(), &data)?;
        txn.commit()?;
        Ok(())
    }

    fn put_editor_part_state(&self, state: EditorPartStateEntity) -> Result<(), DatabaseError> {
        let mut txn = self.client.begin_write()?;

        let grid_data = serde_json::to_vec(&state.grid)?;
        self.table.insert(
            &mut txn,
            WORKBENCH_PARTS_EDITOR_GRID_STATE_KEY.to_string(),
            &grid_data,
        )?;

        let panels_data = serde_json::to_vec(&state.panels)?;
        self.table.insert(
            &mut txn,
            WORKBENCH_PARTS_EDITOR_PANELS_STATE_KEY.to_string(),
            &panels_data,
        )?;

        // Store active group state
        let active_group_data = serde_json::to_vec(&state.active_group)?;
        self.table.insert(
            &mut txn,
            WORKBENCH_PARTS_EDITOR_ACTIVE_GROUP_STATE_KEY.to_string(),
            &active_group_data,
        )?;

        txn.commit()?;
        Ok(())
    }

    fn delete_sidebar_part_state(&self) -> Result<(), DatabaseError> {
        let mut txn = self.client.begin_write()?;
        match self
            .table
            .remove(&mut txn, WORKBENCH_PARTS_SIDEBAR_STATE_KEY.to_string())
        {
            Ok(_) => {
                txn.commit()?;
                Ok(())
            }
            Err(DatabaseError::NotFound { .. }) => {
                txn.commit()?;
                Ok(()) // Not an error if the key doesn't exist
            }
            Err(err) => Err(err),
        }
    }

    fn delete_panel_part_state(&self) -> Result<(), DatabaseError> {
        let mut txn = self.client.begin_write()?;
        match self
            .table
            .remove(&mut txn, WORKBENCH_PARTS_PANEL_STATE_KEY.to_string())
        {
            Ok(_) => {
                txn.commit()?;
                Ok(())
            }
            Err(DatabaseError::NotFound { .. }) => {
                txn.commit()?;
                Ok(()) // Not an error if the key doesn't exist
            }
            Err(err) => Err(err),
        }
    }

    fn delete_editor_part_state(&self) -> Result<(), DatabaseError> {
        let mut txn = self.client.begin_write()?;

        // Remove grid state, ignoring NotFound errors
        match self
            .table
            .remove(&mut txn, WORKBENCH_PARTS_EDITOR_GRID_STATE_KEY.to_string())
        {
            Ok(_) => {}
            Err(DatabaseError::NotFound { .. }) => {} // Not an error if key doesn't exist
            Err(err) => return Err(err),
        }

        // Remove panels state, ignoring NotFound errors
        match self.table.remove(
            &mut txn,
            WORKBENCH_PARTS_EDITOR_PANELS_STATE_KEY.to_string(),
        ) {
            Ok(_) => {}
            Err(DatabaseError::NotFound { .. }) => {} // Not an error if key doesn't exist
            Err(err) => return Err(err),
        }

        // Remove active group state, ignoring NotFound errors
        match self.table.remove(
            &mut txn,
            WORKBENCH_PARTS_EDITOR_ACTIVE_GROUP_STATE_KEY.to_string(),
        ) {
            Ok(_) => {}
            Err(DatabaseError::NotFound { .. }) => {} // Not an error if key doesn't exist
            Err(err) => return Err(err),
        }

        txn.commit()?;
        Ok(())
    }
}
