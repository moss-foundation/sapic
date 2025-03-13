use std::path::PathBuf;


use anyhow::Result;
use moss_db::{bincode_table::BincodeTable, ReDbClient};
use moss_db::{DatabaseClient, Transaction};

use crate::models::storage::CollectionEntity;

use super::{CollectionStore, CollectionTable};

const TABLE_COLLECTION: BincodeTable<String, CollectionEntity> =
    BincodeTable::new("collection");

pub struct CollectionStoreImpl {
    client: ReDbClient,
    table: CollectionTable<'static>,
}

impl CollectionStoreImpl {
    pub fn new(client: ReDbClient) -> Self {
        // Initialize by creating the table in the database
        let table = TABLE_COLLECTION;
        let mut inner_txn= match client.begin_write().unwrap() {
            Transaction::Read(_) => {unreachable!()}
            Transaction::Write(txn) => {txn}
        };
        inner_txn.open_table(table.table).unwrap();
        inner_txn.commit().unwrap();

        Self {
            client,
            table: TABLE_COLLECTION,
        }
    }
}

impl CollectionStore for CollectionStoreImpl {

    fn begin_write(&self) -> Result<(Transaction, &CollectionTable)> {
        let write_txn = self.client.begin_write()?;
        Ok((write_txn, &self.table))
    }

    fn begin_read(&self) -> Result<(Transaction, &CollectionTable)> {
        let read_txn = self.client.begin_read()?;
        Ok((read_txn, &self.table))
    }

    fn get_all_items(&self) -> Result<Vec<(PathBuf, CollectionEntity)>> {
        let read_txn = self.client.begin_read()?;
        Ok(
            self.table
                .scan(&read_txn)?
                .map(|(path, metadata)| (PathBuf::from(path), metadata))
                .collect()
        )
    }

}
