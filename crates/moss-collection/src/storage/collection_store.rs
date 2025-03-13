use std::path::PathBuf;


use anyhow::Result;
use moss_db::{bincode_table::BincodeTable, ReDbClient};
use moss_db::{DatabaseClient, Transaction};

use crate::models::storage::CollectionMetadataEntity;

use super::{CollectionStore, CollectionTable};

const TABLE_COLLECTION: BincodeTable<String, CollectionMetadataEntity> =
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

    fn get_all_items(&self) -> Result<Vec<(PathBuf, CollectionMetadataEntity)>> {
        let read_txn = self.client.begin_read()?;

        let iter = self.table.scan(&read_txn)?;
        read_txn.commit()?;
        Ok(
            iter.map(|(path, metadata)| (PathBuf::from(path), metadata))
                .collect()
        )
    }

    // async fn create_collection(
    //     &self,
    //     f: Box<
    //         dyn FnOnce(
    //                 Transaction,
    //                 CollectionTable,
    //             ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>
    //             + Send,
    //     >,
    // ) -> Result<()> {
    //     let write_txn = self.client.begin_write()?;

    //     f(Transaction::Write(write_txn), self.table.clone()).await
    // }
}
