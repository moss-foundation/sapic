use anyhow::Result;

use crate::{
    collection::Collection,
    models::operations::collection_operations::{ListRequestsOutput, RequestInfo},
};

impl Collection {
    pub async fn list_requests(&self) -> Result<ListRequestsOutput> {
        let requests = self.requests().await?;
        let requests_lock = requests.read().await;

        Ok(ListRequestsOutput(
            requests_lock
                .iter()
                .filter(|(_, iter_slot)| !iter_slot.is_leased())
                .map(|(key, iter_slot)| {
                    let request_data = iter_slot.value();
                    RequestInfo {
                        key,
                        name: request_data.name.to_string(),
                        request_dir_relative_path: request_data.request_dir_relative_path.clone(),
                        order: request_data.order,
                        typ: request_data.typ.clone(),
                    }
                })
                .collect(),
        ))
    }
}
