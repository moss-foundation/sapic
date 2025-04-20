use anyhow::Result;
use moss_db::{
    bincode_table::BincodeTable,
    common::{DatabaseError, Transaction},
    DatabaseClient, ReDbClient,
};
use std::{collections::HashMap, path::PathBuf};

use super::{entities::request_store_entities::RequestNodeEntity, RequestStore, RequestStoreTable};

#[rustfmt::skip]
pub(in crate::collection_storage) const TABLE_REQUESTS: BincodeTable<String, RequestNodeEntity> = BincodeTable::new("requests");

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
    fn list_request_nodes(&self) -> Result<HashMap<PathBuf, RequestNodeEntity>, DatabaseError> {
        let read_txn = self.client.begin_read()?;
        let result = self.table.scan(&read_txn)?;

        Ok(result
            .map(|(path_str, request)| (PathBuf::from(path_str), request))
            .collect())
    }

    fn set_request_node(
        &self,
        txn: &mut Transaction,
        path: PathBuf,
        node: RequestNodeEntity,
    ) -> Result<(), DatabaseError> {
        self.table
            .insert(txn, path.to_string_lossy().to_string(), &node)?;

        Ok(())
    }

    fn get_request_node(&self, path: PathBuf) -> Result<RequestNodeEntity, DatabaseError> {
        let read_txn = self.client.begin_read()?;
        let result = self
            .table
            .read(&read_txn, path.to_string_lossy().to_string())?;

        Ok(result)
    }

    fn delete_request_node(
        &self,
        txn: &mut Transaction,
        path: PathBuf,
    ) -> Result<(), DatabaseError> {
        self.table.remove(txn, path.to_string_lossy().to_string())?;

        Ok(())
    }
}
