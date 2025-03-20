use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;

use moss_db::{bincode_table::BincodeTable, ReDbClient};
use moss_db::{DatabaseClient, Transaction};

use crate::models::storage::RequestEntity;

use super::{RequestStore, RequestStoreTable};

#[rustfmt::skip]
pub(super) const TABLE_REQUESTS: BincodeTable<String, RequestEntity> = BincodeTable::new("requests");

pub struct RequestStoreImpl {
    client: ReDbClient,
    table: RequestStoreTable<'static>,
}

impl RequestStoreImpl {
    pub fn new(client: ReDbClient) -> Self {
        Self {
            client,
            table: TABLE_REQUESTS,
        }
    }
}

impl RequestStore for RequestStoreImpl {
    fn scan(&self) -> anyhow::Result<HashMap<PathBuf, RequestEntity>> {
        let read_txn = self.client.begin_read()?;
        let result = self.table.scan(&read_txn)?;

        Ok(result
            .map(|(path_str, request)| (PathBuf::from(path_str), request))
            .collect())
    }

    fn begin_write(&self) -> Result<(Transaction, &RequestStoreTable)> {
        let write_txn = self.client.begin_write()?;
        Ok((write_txn, &self.table))
    }

    fn begin_read(&self) -> Result<(Transaction, &RequestStoreTable)> {
        let read_txn = self.client.begin_read()?;
        Ok((read_txn, &self.table))
    }
}
