use anyhow::Result;
use dashmap::DashMap;
use moss_app::service::Service;
use moss_fs::ports::FileSystem;
use patricia_tree::PatriciaMap;
use std::{collections::HashMap, path::PathBuf, sync::Arc};

use crate::{
    models::{
        collection::{CollectionRequestEntry, CollectionRequestVariantEntry},
        storage::CollectionMetadataEntity,
    },
    ports::{
        collection_ports::CollectionIndexer,
        storage_ports::{CollectionMetadataStore, CollectionRequestSubstore},
    },
};

pub struct CollectionState {
    name: String,
    order: Option<usize>,
    requests: PatriciaMap<CollectionRequestEntry>,
}

impl CollectionState {
    fn new(name: String, order: Option<usize>) -> Self {
        Self {
            name,
            order,
            requests: PatriciaMap::new(),
        }
    }
}

pub struct CollectionHandle {
    fs: Arc<dyn FileSystem>,
    store: Arc<dyn CollectionRequestSubstore>,
    state: CollectionState,
}

pub struct CollectionService {
    fs: Arc<dyn FileSystem>,
    collection_store: Arc<dyn CollectionMetadataStore>,
    collection_request_substore: Arc<dyn CollectionRequestSubstore>,
    collections: DashMap<PathBuf, CollectionHandle>,
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
        for (collection_path, collection_metadata) in self.collection_store.get_all_items()? {
            let collection_name = match collection_path.file_name() {
                Some(name) => name,
                None => {
                    println!("failed to get the collection {:?} name", collection_path);
                    continue;
                }
            };

            let indexed_collection = match self.indexer.index(&collection_path).await {
                Ok(data) => data,
                Err(err) => {
                    println!(
                        "failed to index the collection {:?}: {}",
                        collection_path, err
                    );
                    continue;
                }
            };

            let mut state = CollectionState::new(
                collection_name.to_string_lossy().to_string(),
                collection_metadata.order,
            );

            for (key, request_entry) in indexed_collection
                .requests
                .iter_prefix(collection_path.to_string_lossy().as_bytes())
            {
                let request_metadata = collection_metadata.requests.get(&key);

                let mut variants = HashMap::new();
                for variant in &request_entry.variants {
                    let variant_path = variant.path.to_string_lossy().to_string();
                    let variant_order = request_metadata
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
                        order: request_metadata.map(|m| m.order),
                        typ: request_entry.ext.clone(),
                        variants,
                    },
                );
            }

            self.collections.insert(
                collection_path,
                CollectionHandle {
                    fs: Arc::clone(&self.fs),
                    store: Arc::clone(&self.collection_request_substore),
                    state,
                },
            );
        }

        todo!()
    }

    pub async fn create_collection(&self, path: PathBuf, name: String) -> Result<()> {
        self.fs.create_dir(path.join(&name).as_path()).await?;

        let order = self.collections.len() + 1;

        self.collection_store.put_collection_item(
            path.clone(),
            CollectionMetadataEntity {
                order: Some(order),
                requests: HashMap::new(),
            },
        )?;

        self.collections.insert(
            path,
            CollectionHandle {
                fs: Arc::clone(&self.fs),
                store: Arc::clone(&self.collection_request_substore),
                state: CollectionState::new(name, Some(order)),
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
