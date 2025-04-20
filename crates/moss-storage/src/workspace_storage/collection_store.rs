use anyhow::Result;
use moss_db::{bincode_table::BincodeTable, common::Transaction, DatabaseClient, ReDbClient};
use std::path::PathBuf;

use super::{
    entities::collection_store_entities::CollectionEntity, CollectionStore, CollectionStoreTable,
};

#[rustfmt::skip]
pub(super) const TABLE_COLLECTIONS: BincodeTable<String, CollectionEntity> = BincodeTable::new("collections");

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
    fn begin_write(&self) -> Result<(Transaction, &CollectionStoreTable)> {
        let write_txn = self.client.begin_write()?;
        Ok((write_txn, &self.table))
    }

    fn begin_read(&self) -> Result<(Transaction, &CollectionStoreTable)> {
        let read_txn = self.client.begin_read()?;
        Ok((read_txn, &self.table))
    }

    fn scan(&self) -> Result<Vec<(PathBuf, CollectionEntity)>> {
        let read_txn = self.client.begin_read()?;
        Ok(self
            .table
            .scan(&read_txn)?
            .map(|(path, metadata)| (PathBuf::from(path), metadata))
            .collect())
    }
}
