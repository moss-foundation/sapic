use moss_common::api::OperationResult;

use crate::{
    collection::Collection,
    collection_registry::RequestNode,
    models::{operations::ListRequestsOutput, types::RequestNodeInfo},
};

impl Collection {
    pub async fn list_requests(&self) -> OperationResult<ListRequestsOutput> {
        let requests = self.registry().await?.requests_nodes();
        let requests_lock = requests.read().await;

        Ok(ListRequestsOutput(
            requests_lock
                .iter()
                .filter(|(_, iter_slot)| !iter_slot.is_leased())
                .map(|(key, iter_slot)| {
                    let request_data = iter_slot.value();
                    match request_data {
                        RequestNode::Request(request_data) => RequestNodeInfo::Request {
                            key,
                            name: request_data.name.to_string(),
                            path: request_data.path.clone(),
                            order: request_data.order,
                            protocol: request_data.protocol(),
                        },
                        RequestNode::Group(request_data) => RequestNodeInfo::Group {
                            key,
                            name: request_data.name.to_string(),
                            path: request_data.path.clone(),
                            order: request_data.order,
                        },
                    }
                })
                .collect(),
        ))
    }
}
