use std::collections::HashMap;
use std::path::PathBuf;

use moss_db::{bincode_table::BincodeTable, ReDbClient};
use moss_db::{DatabaseClient, Transaction};

use crate::models::storage::RequestEntity;

use super::{RequestStore, RequestStoreTable};

const TABLE_REQUESTS: BincodeTable<String, RequestEntity> = BincodeTable::new("requests");

pub struct RequestStoreImpl {
    client: ReDbClient,
    table: RequestStoreTable<'static>,
}

impl RequestStoreImpl {
    pub fn new(client: ReDbClient) -> Self {
        // Initialize by creating the table in the database
        let table = TABLE_REQUESTS;
        let inner_txn = match client.begin_write().unwrap() {
            Transaction::Read(_) => {
                unreachable!()
            }
            Transaction::Write(txn) => txn,
        };
        inner_txn.open_table(table.table).unwrap();
        inner_txn.commit().unwrap();

        Self {
            client,
            table: TABLE_REQUESTS,
        }
    }
}

impl RequestStore for RequestStoreImpl {
    fn scan(&self) -> anyhow::Result<HashMap<String, RequestEntity>> {
        let read_txn = self.client.begin_read()?;
        let result = self.table.scan(&read_txn)?;

        Ok(result
            .map(|(path_str, request)| (path_str, request))
            .collect())
    }
}
