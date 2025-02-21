use anyhow::Result;
use dashmap::DashMap;
use moss_app::service::Service;
use moss_fs::FileSystem;
use patricia_tree::PatriciaMap;
use std::{collections::HashMap, path::PathBuf, sync::Arc};

use crate::domain::{
    models::{collection::RequestType, storage::CollectionMetadataEntity},
    ports::{
        collection_ports::CollectionIndexer,
        db_ports::{CollectionMetadataStore, CollectionRequestSubstore},
    },
};

pub struct CollectionRequestVariantEntry {
    name: String,
    order: Option<usize>,
}

pub struct CollectionRequestEntry {
    name: String,
    order: Option<usize>,
    ext: Option<RequestType>,
    variants: HashMap<String, CollectionRequestVariantEntry>,
}

pub struct LocalCollectionState {
    name: String,
    order: usize,
    requests: PatriciaMap<CollectionRequestEntry>,
}

impl LocalCollectionState {
    fn new(name: String, order: usize) -> Self {
        Self {
            name,
            order,
            requests: PatriciaMap::new(),
        }
    }
}

pub enum CollectionHandle {
    Local {
        fs: Arc<dyn FileSystem>,
        repo: Arc<dyn CollectionRequestSubstore>,
        state: LocalCollectionState,
    },

    Remote {},
}

pub struct DescribeCollectionOutput {
    name: String,
    order: Option<usize>,
    source: String,
    // requests: Vec
}

pub struct CollectionService {
    fs: Arc<dyn FileSystem>,
    collection_store: Arc<dyn CollectionMetadataStore>,
    collection_request_substore: Arc<dyn CollectionRequestSubstore>,
    collections: DashMap<String, CollectionHandle>,
    indexer: Arc<dyn CollectionIndexer>,
}

impl CollectionService {
    pub fn new(
        fs: Arc<dyn FileSystem>,
        collection_store: Arc<dyn CollectionMetadataStore>,
        collection_request_substore: Arc<dyn CollectionRequestSubstore>,
        indexer: Arc<dyn CollectionIndexer>,
    ) -> Result<Self> {
        Ok(Self {
            fs,
            collection_store,
            collection_request_substore,
            collections: DashMap::new(),
            indexer,
        })
    }
}

impl CollectionService {
    // TODO: At the moment, there is no clear understanding of the format in which
    // collection descriptions should be sent to the frontend. This largely depends
    // on the library we choose to use for displaying hierarchical structures.

    pub async fn describe_collections(&self) -> Result<()> {
        if self.collections.is_empty() {
            self.restore_collections().await?;
        }

        unimplemented!()
    }

    async fn restore_collections(&self) -> Result<()> {
        for (source, collection_details) in self.collection_store.get_all_items()? {
            // TODO: Check whether the source is a URL or a filesystem path,
            // and create either a local or remote handle accordingly.
            // Currently, we only support the local handle.

            let path = PathBuf::from(source.clone());

            let collection_name = match path.file_name() {
                Some(name) => name,
                None => {
                    println!("failed to get the collection {:?} name", path);
                    continue;
                }
            };

            let indexed_collection = match self.indexer.index(&path).await {
                Ok(data) => data,
                Err(err) => {
                    println!("failed to index the collection {:?}: {}", path, err);
                    continue;
                }
            };

            let mut state = LocalCollectionState::new(
                collection_name.to_string_lossy().to_string(),
                collection_details.order,
            );

            for (key, request_entry) in indexed_collection.requests.iter_prefix(source.as_bytes()) {
                let metadata = collection_details.requests.get(&key);

                let mut variants = HashMap::new();
                for variant in &request_entry.variants {
                    let variant_path = variant.path.to_string_lossy().to_string();
                    let variant_order = metadata
                        .and_then(|request_metadata| Some(&request_metadata.variants))
                        .and_then(|variants_metadata| variants_metadata.get(&variant_path))
                        .and_then(|variant| Some(variant.order));

                    variants.insert(
                        variant_path,
                        CollectionRequestVariantEntry {
                            name: variant.name.clone(),
                            order: variant_order,
                        },
                    );
                }

                state.requests.insert(
                    key,
                    CollectionRequestEntry {
                        name: request_entry.name.clone(),
                        order: metadata.map(|m| m.order),
                        ext: request_entry.ext.clone(),
                        variants,
                    },
                );
            }

            self.collections.insert(
                source,
                CollectionHandle::Local {
                    fs: Arc::clone(&self.fs),
                    repo: Arc::clone(&self.collection_request_substore),
                    state,
                },
            );
        }

        todo!()
    }

    pub async fn create_collection(&self, path: PathBuf, name: String) -> Result<()> {
        self.fs.create_dir(path.join(&name).as_path()).await?;

        let source = path.to_string_lossy().to_string();
        let order = self.collections.len() + 1;

        self.collection_store.put_collection_item(
            source.clone(),
            CollectionMetadataEntity {
                order,
                requests: HashMap::new(),
            },
        )?;

        self.collections.insert(
            source,
            CollectionHandle::Local {
                fs: Arc::clone(&self.fs),
                repo: Arc::clone(&self.collection_request_substore),
                state: LocalCollectionState::new(name, order),
            },
        );

        Ok(())
    }
}

impl Service for CollectionService {
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    fn dispose(&self) {}

    fn as_any(&self) -> &(dyn std::any::Any + Send) {
        self
    }
}
