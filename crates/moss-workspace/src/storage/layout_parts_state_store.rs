use anyhow::Result;
use moss_db::{
    bincode_table::BincodeTable,
    common::{AnyEntity, DatabaseError},
    DatabaseClient, ReDbClient,
};
use serde::de::DeserializeOwned;

use crate::models::entities::{
    EditorPartStateEntity, PanelPartStateEntity, SidebarPartStateEntity,
};

use super::{LayoutPartsStateStore, LayoutPartsStateStoreTable};

const WORKBENCH_PARTS_EDITOR_GRID_STATE_KEY: &str = "workbench.parts.editor.grid";
const WORKBENCH_PARTS_EDITOR_PANELS_STATE_KEY: &str = "workbench.parts.editor.panels";
const WORKBENCH_PARTS_PANEL_STATE_KEY: &str = "workbench.parts.panel";
const WORKBENCH_PARTS_SIDEBAR_STATE_KEY: &str = "workbench.parts.sidebar";
const WORKBENCH_PARTS_EDITOR_ACTIVE_GROUP_STATE_KEY: &str = "workbench.parts.editor.activeGroup";

#[rustfmt::skip]
pub(super) const TABLE_PARTS_STATE: BincodeTable<String, AnyEntity> = BincodeTable::new("parts_state");

pub struct PartsStateStoreImpl {
    client: ReDbClient,
    table: LayoutPartsStateStoreTable<'static>,
}

impl PartsStateStoreImpl {
    pub fn new(client: ReDbClient) -> Self {
        Self {
            client,
            table: TABLE_PARTS_STATE,
        }
    }

    fn get_by_key<T: DeserializeOwned>(&self, key: String) -> Result<T, DatabaseError> {
        let txn = self.client.begin_read()?;
        let data = self.table.read(&txn, key)?;

        Ok(bincode::deserialize(&data)?)
    }
}

impl LayoutPartsStateStore for PartsStateStoreImpl {
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
                bincode::deserialize(&data)?
            },
            panels: {
                let data = self
                    .table
                    .read(&txn, WORKBENCH_PARTS_EDITOR_PANELS_STATE_KEY.to_string())?;
                bincode::deserialize(&data)?
            },
            active_group: {
                let data = self.table.read(
                    &txn,
                    WORKBENCH_PARTS_EDITOR_ACTIVE_GROUP_STATE_KEY.to_string(),
                )?;
                bincode::deserialize(&data)?
            },
        })
    }
}
