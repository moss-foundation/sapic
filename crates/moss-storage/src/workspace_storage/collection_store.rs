use anyhow::Result;
use moss_db::{
    bincode_table::BincodeTable, common::Transaction, DatabaseClient, DatabaseError, ReDbClient,
};
use std::path::PathBuf;

use super::{
    entities::collection_store_entities::CollectionEntity, CollectionStore, CollectionStoreTable,
};

#[rustfmt::skip]
pub(in crate::workspace_storage) const TABLE_COLLECTIONS: BincodeTable<String, CollectionEntity> = BincodeTable::new("collections");

pub struct CollectionStoreImpl {
    client: ReDbClient,
    table: CollectionStoreTable<'static>,
}

impl CollectionStoreImpl {
    pub fn new(client: ReDbClient) -> Self {
        Self {
            client,
            table: TABLE_COLLECTIONS,
        }
    }
}

impl CollectionStore for CollectionStoreImpl {
    fn create_collection_entity(
        &self,
        txn: &mut Transaction,
        path: PathBuf,
        entity: CollectionEntity,
    ) -> Result<(), DatabaseError> {
        let key = path.to_string_lossy().to_string();
        self.table.insert(txn, key, &entity)
    }

    fn rekey_collection_entity(
        &self,
        txn: &mut Transaction,
        old_path: PathBuf,
        new_path: PathBuf,
    ) -> Result<(), DatabaseError> {
        let old_key = old_path.to_string_lossy().to_string();
        let new_key = new_path.to_string_lossy().to_string();

        self.table.rekey(txn, old_key, new_key)
    }

    fn delete_collection_entity(
        &self,
        txn: &mut Transaction,
        path: PathBuf,
    ) -> Result<(), DatabaseError> {
        let key = path.to_string_lossy().to_string();
        self.table.remove(txn, key)?;

        Ok(())
    }

    // fn begin_read(&self) -> Result<(Transaction, &CollectionStoreTable)> {
    //     let read_txn = self.client.begin_read()?;
    //     Ok((read_txn, &self.table))
    // }

    fn list_collections(&self) -> Result<Vec<(PathBuf, CollectionEntity)>> {
        let read_txn = self.client.begin_read()?;
        Ok(self
            .table
            .scan(&read_txn)?
            .map(|(path, metadata)| (PathBuf::from(path), metadata))
            .collect())
    }
}
