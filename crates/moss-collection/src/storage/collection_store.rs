use std::future::Future;
use std::pin::Pin;

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
        Self {
            client,
            table: TABLE_COLLECTION,
        }
    }
}

impl CollectionStore for CollectionStoreImpl {
    fn begin_write(&self) -> Result<(Transaction, &CollectionTable)> {
        let write_txn = self.client.begin_write()?;

        Ok((moss_db::Transaction::Write(write_txn), &self.table))
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
