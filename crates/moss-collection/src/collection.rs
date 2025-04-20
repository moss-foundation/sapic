pub mod api;

use anyhow::{Context, Result};
use moss_common::leased_slotmap::{LeasedSlotMap, ResourceKey};
use moss_fs::{FileSystem, RenameOptions};
use moss_storage::collection_storage::entities::request_store_entities::RequestNodeEntity;
use moss_storage::collection_storage::CollectionStorageImpl;
use moss_storage::CollectionStorage;
use std::collections::HashMap;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::{mpsc, OnceCell};

use crate::collection_registry::{
    CollectionRegistry, CollectionRequestData, CollectionRequestGroupData, RequestNode,
};
use crate::constants::*;
use crate::indexer::{
    IndexJob, IndexMessage, IndexedNode, IndexedRequestGroupNode, IndexedRequestNode, IndexerHandle,
};

#[derive(Clone, Debug)]
pub struct CollectionCache {
    pub name: String,
    pub order: Option<usize>,
}

pub struct Collection {
    fs: Arc<dyn FileSystem>,
    abs_path: PathBuf,
    collection_storage: Arc<dyn CollectionStorage>,
    registry: OnceCell<CollectionRegistry>,
    indexer_handle: IndexerHandle,
}

impl Collection {
    pub fn new(
        path: PathBuf,
        fs: Arc<dyn FileSystem>,
        indexer_handle: IndexerHandle,
    ) -> Result<Self> {
        let state_db_manager_impl = CollectionStorageImpl::new(&path).context(format!(
            "Failed to open the collection {} state database",
            path.display()
        ))?;

        Ok(Self {
            fs: Arc::clone(&fs),
            abs_path: path,
            registry: OnceCell::new(),
            collection_storage: Arc::new(state_db_manager_impl),
            indexer_handle,
        })
    }

    fn handle_indexed_request_node(
        &self,
        indexed_request_node: IndexedRequestNode,
        restored_requests: &HashMap<PathBuf, RequestNodeEntity>,
    ) -> Result<RequestNode> {
        let node_relative_path = indexed_request_node
            .path
            .strip_prefix(&self.abs_path)
            .unwrap()
            .to_path_buf();

        let order = restored_requests
            .get(&node_relative_path)
            .and_then(|e| e.as_request().and_then(|r| r.order));

        let spec_file_name = indexed_request_node
            .spec_file_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(GET_ENTRY_SPEC_FILE)
            .to_string();

        Ok(RequestNode::Request(CollectionRequestData {
            name: moss_fs::utils::decode_name(&indexed_request_node.name)?,
            path: node_relative_path,
            order,
            spec_file_name,
        }))
    }

    fn handle_indexed_request_group_node(
        &self,
        indexed_request_group_node: IndexedRequestGroupNode,
        restored_request_nodes: &HashMap<PathBuf, RequestNodeEntity>,
    ) -> Result<RequestNode> {
        let node_relative_path = indexed_request_group_node
            .path
            .strip_prefix(&self.abs_path)
            .unwrap()
            .to_path_buf();

        let order = restored_request_nodes
            .get(&node_relative_path)
            .and_then(|e| e.as_group().and_then(|g| g.order));

        let spec_file_name = if let Some(spec_file_path) = indexed_request_group_node.spec_file_path
        {
            spec_file_path
                .file_name()
                .and_then(|name| name.to_str().map(|s| s.to_string()))
        } else {
            None
        };

        Ok(RequestNode::Group(CollectionRequestGroupData {
            name: moss_fs::utils::decode_name(&indexed_request_group_node.name)?,
            path: node_relative_path,
            order,
            spec_file_name,
        }))
    }

    async fn registry(&self) -> Result<&CollectionRegistry> {
        let registry = self
            .registry
            .get_or_try_init(|| async move {
                let mut requests_nodes = LeasedSlotMap::new();
                let mut endpoints_nodes = LeasedSlotMap::new();
                let mut schemas_nodes = LeasedSlotMap::new();
                let mut components_nodes = LeasedSlotMap::new();

                let (result_tx, mut result_rx) = mpsc::unbounded_channel();
                self.indexer_handle.emit_job(IndexJob {
                    collection_key: ResourceKey::from(457895),
                    collection_abs_path: self.abs_path.clone(),
                    result_tx,
                })?;

                let request_store = self.collection_storage.request_store().await;
                let restored_requests = request_store.list_request_nodes()?;

                while let Some(index_msg) = result_rx.recv().await {
                    match index_msg {
                        IndexMessage::Ok(indexed_node) => match indexed_node {
                            IndexedNode::Request(indexed_request_entry) => {
                                let request_node = self.handle_indexed_request_node(
                                    indexed_request_entry,
                                    &restored_requests,
                                )?;

                                requests_nodes.insert(request_node);
                            }
                            IndexedNode::RequestGroup(indexed_request_group_node) => {
                                let request_group_node = self.handle_indexed_request_group_node(
                                    indexed_request_group_node,
                                    &restored_requests,
                                )?;

                                requests_nodes.insert(request_group_node);
                            }
                            IndexedNode::Endpoint(_indexed_endpoint_node) => unimplemented!(),
                            IndexedNode::EndpointGroup(_indexed_endpoint_group_node) => {
                                unimplemented!()
                            }
                            IndexedNode::Schema(_indexed_schema_node) => unimplemented!(),
                            IndexedNode::SchemaGroup(_indexed_schema_group_node) => {
                                unimplemented!()
                            }
                            IndexedNode::Component(_indexed_component_node) => unimplemented!(),
                            IndexedNode::ComponentGroup(_indexed_component_group_node) => {
                                unimplemented!()
                            }
                        },
                        IndexMessage::Err(err) => {
                            // TODO: log error
                            dbg!(err);
                        }
                    }
                }

                Ok::<_, anyhow::Error>(CollectionRegistry::new(
                    requests_nodes,
                    endpoints_nodes,
                    schemas_nodes,
                    components_nodes,
                ))
            })
            .await?;

        Ok(registry)
    }

    pub fn path(&self) -> &PathBuf {
        &self.abs_path
    }

    pub async fn reset(&mut self, new_path: PathBuf) -> Result<()> {
        let old_path = std::mem::replace(&mut self.abs_path, new_path.clone());
        let fs_clone = self.fs.clone();
        let new_path_clone = new_path.clone();

        let after_drop = Box::pin(async move {
            fs_clone
                .rename(&old_path, &new_path_clone, RenameOptions::default())
                .await?;

            Ok(())
        });

        self.collection_storage.reset(new_path, after_drop).await?;

        Ok(())
    }
}
