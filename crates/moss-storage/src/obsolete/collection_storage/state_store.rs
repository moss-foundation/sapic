use moss_db::{
    AnyEntity, DatabaseClient, DatabaseResult, ReDbClient, Transaction, bincode_table::BincodeTable,
};
use std::path::Path;

use super::{StateStore, StateStoreTable, entities::state_store_entries::WorktreeEntryEntity};

#[rustfmt::skip]
pub(in crate::collection_storage) const TABLE_STATE: BincodeTable<String, AnyEntity> = BincodeTable::new("state");

const WORKTREE_ENTRIES_STATE_KEY: &str = "worktree.entries";

pub(in crate::collection_storage) struct StateStoreImpl {
    client: ReDbClient,
    table: StateStoreTable<'static>,
}

impl StateStoreImpl {
    pub fn new(client: ReDbClient) -> Self {
        Self {
            client,
            table: TABLE_STATE,
        }
    }
}

impl StateStore for StateStoreImpl {
    fn list_worktree_entries(&self) -> DatabaseResult<Vec<WorktreeEntryEntity>> {
        let txn = self.client.begin_read()?;
        let entries = self
            .table
            .scan_by_prefix(&txn, WORKTREE_ENTRIES_STATE_KEY)?
            .into_iter()
            .map(|(_, bytes)| serde_json::from_slice(&bytes))
            .collect::<Result<Vec<WorktreeEntryEntity>, _>>()?;

        Ok(entries)
    }

    fn upsert_worktree_entry(
        &self,
        txn: &mut Transaction,
        entry: WorktreeEntryEntity,
    ) -> DatabaseResult<()> {
        let key = worktree_entry_key(&entry.path);
        let value = serde_json::to_vec(&entry)?;

        self.table.insert(txn, key, &value)
    }
}

fn worktree_entry_key(path: impl AsRef<Path>) -> String {
    format!(
        "{}:file:{}",
        WORKTREE_ENTRIES_STATE_KEY,
        path.as_ref().to_string_lossy().to_string()
    )
}
