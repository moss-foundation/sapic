use anyhow::{anyhow, Result};
use dashmap::DashMap;
use moss_app::service::Service;
use moss_fs::ports::FileSystem;
use parking_lot::RwLock;
use patricia_tree::PatriciaMap;
use serde::Serialize;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::OnceCell;

use crate::{
    models::{
        collection::{CollectionRequestVariantEntry, RequestType},
        storage::CollectionMetadataEntity,
    },
    ports::{
        collection_ports::CollectionIndexer,
        storage_ports::{CollectionMetadataStore, CollectionRequestSubstore},
    },
};

pub struct RequestState {
    pub name: String,
    pub order: Option<usize>,
    pub typ: Option<RequestType>,
    pub variants: RwLock<HashMap<PathBuf, CollectionRequestVariantEntry>>,
}

pub struct RequestHandle {
    fs: Arc<dyn FileSystem>,
    state: RequestState,
}

pub struct CollectionState {
    name: String,
    order: Option<usize>,
    requests: RwLock<PatriciaMap<Arc<RequestHandle>>>,
}

impl CollectionState {
    fn new(name: String, order: Option<usize>) -> Self {
        Self {
            name,
            order,
            requests: RwLock::new(PatriciaMap::new()),
        }
    }

    fn get_request_handle_or_init(
        &self,
        key: &[u8],
        f: impl FnOnce() -> RequestHandle,
    ) -> Arc<RequestHandle> {
        {
            let read_guard = self.requests.read();
            if let Some(entry) = read_guard.get(key) {
                return Arc::clone(&entry);
            }
        }

        let mut write_guard = self.requests.write();
        if let Some(entry) = write_guard.get(key) {
            return Arc::clone(&entry);
        }

        let entry = Arc::new(f());
        write_guard.insert(key, Arc::clone(&entry));
        entry
    }
}

pub struct CollectionHandle {
    fs: Arc<dyn FileSystem>,
    store: Arc<dyn CollectionRequestSubstore>,
    state: Arc<CollectionState>,
}

pub struct CreateRequestInput {
    name: String,
}

impl CollectionHandle {
    pub fn create_request(
        &self,
        collection_path: &PathBuf,
        input: CreateRequestInput,
    ) -> Result<()> {
        unimplemented!()
    }
}

type CollectionMap = DashMap<PathBuf, CollectionHandle>;

pub struct CollectionService {
    fs: Arc<dyn FileSystem>,
    collection_store: Arc<dyn CollectionMetadataStore>,
    collection_request_substore: Arc<dyn CollectionRequestSubstore>,
    collections: OnceCell<Arc<CollectionMap>>,
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
            collections: OnceCell::new(),
            indexer,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct CollectionOverview {
    pub name: String,
    pub path: PathBuf,
    pub order: Option<usize>,
}

impl CollectionService {
    async fn collections(&self) -> Result<Arc<CollectionMap>> {
        let collections = self
            .collections
            .get_or_try_init(|| async move {
                let collections = DashMap::new();

                for (collection_path, collection_metadata) in
                    self.collection_store.get_all_items()?
                {
                    let collection_name = match collection_path.file_name() {
                        Some(name) => name,
                        None => {
                            // TODO: logging
                            println!("failed to get the collection {:?} name", collection_path);
                            continue;
                        }
                    };

                    let state = CollectionState::new(
                        collection_name.to_string_lossy().to_string(),
                        collection_metadata.order,
                    );

                    collections.insert(
                        collection_path,
                        CollectionHandle {
                            fs: Arc::clone(&self.fs),
                            store: Arc::clone(&self.collection_request_substore),
                            state: Arc::new(state),
                        },
                    );
                }

                Ok::<Arc<CollectionMap>, anyhow::Error>(Arc::new(collections))
            })
            .await?;

        Ok(Arc::clone(collections))
    }
}

pub struct CreateCollectionInput {
    path: PathBuf,
    name: String,
    repo: Option<String>, // Url ?
}

impl CollectionService {
    pub async fn overview_collections(&self) -> Result<Vec<CollectionOverview>> {
        let collections = self.collections().await?;

        Ok(collections
            .iter()
            .map(|item| CollectionOverview {
                name: item.state.name.clone(),
                path: item.key().clone(),
                order: item.state.order,
            })
            .collect())
    }

    // TODO: At the moment, there is no clear understanding of the format in which
    // collection descriptions should be sent to the frontend. This largely depends
    // on the library we choose to use for displaying hierarchical structures.

    pub async fn index_collection(&self, path: PathBuf) -> Result<Arc<CollectionState>> {
        let collections = self.collections().await?;
        let collection_handle = collections
            .get(&path)
            .ok_or_else(|| anyhow!("Collection with path {:?} not found", path))?;
        let collection_state = Arc::clone(&collection_handle.state);

        let indexed_collection = self.indexer.index(&path).await?;

        for (raw_request_path, indexed_request_entry) in indexed_collection.requests {
            let request_handle =
                collection_state.get_request_handle_or_init(&raw_request_path, || RequestHandle {
                    fs: Arc::clone(&self.fs),
                    state: RequestState {
                        name: indexed_request_entry.name,
                        order: None,
                        typ: indexed_request_entry.typ,
                        variants: Default::default(),
                    },
                });

            let mut variants = Vec::new();

            {
                let variants_lock = request_handle.state.variants.read();
                for (variant_path, variant_entry) in indexed_request_entry.variants {
                    let variant_order = variants_lock
                        .get(&variant_path)
                        .and_then(|variant| variant.order);

                    variants.push((
                        variant_path,
                        CollectionRequestVariantEntry {
                            name: variant_entry.name,
                            order: variant_order,
                        },
                    ));
                }
            }

            request_handle
                .state
                .variants
                .write()
                .extend(variants.into_iter());
        }

        Ok(collection_state)
    }

    pub async fn create_collection(&self, input: CreateCollectionInput) -> Result<()> {
        if input.name.is_empty() {
            return Err(anyhow!("Collection name cannot be empty"));
        }

        let full_path = input.path.join(&input.name);
        self.fs.create_dir(&full_path).await?;

        // TODO: init repo

        let collections = self.collections().await?;

        self.collection_store.put_collection_item(
            input.path.clone(),
            CollectionMetadataEntity {
                order: None,
                requests: Default::default(),
            },
        )?;

        collections.insert(
            input.path,
            CollectionHandle {
                fs: Arc::clone(&self.fs),
                store: Arc::clone(&self.collection_request_substore),
                state: Arc::new(CollectionState::new(input.name, None)),
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

#[cfg(test)]
mod tests {
    use moss_fs::adapters::disk::DiskFileSystem;

    use crate::{
        models::storage::RequestMetadataEntity, services::indexing_service::IndexingService,
    };

    use super::*;

    struct MockCollectionMetadataStore {}

    const TEST_COLLECTION_PATH: &'static str =
        "/Users/g10z3r/Project/keenawa-co/sapic/crates/moss-collection/tests/TestCollection";

    const TEST_REQUEST_PATH: &'static str =
        "/Users/g10z3r/Project/keenawa-co/sapic/crates/moss-collection/tests/TestCollection/requests/Test1.request";

    impl CollectionMetadataStore for MockCollectionMetadataStore {
        fn get_all_items(&self) -> Result<Vec<(PathBuf, CollectionMetadataEntity)>> {
            Ok(vec![(
                PathBuf::from(TEST_COLLECTION_PATH),
                CollectionMetadataEntity {
                    order: None,
                    requests: {
                        let mut this = HashMap::new();
                        this.insert(
                            TEST_REQUEST_PATH.into(),
                            RequestMetadataEntity {
                                order: None,
                                variants: Default::default(),
                            },
                        );

                        this
                    },
                },
            )])
        }

        fn put_collection_item(&self, path: PathBuf, item: CollectionMetadataEntity) -> Result<()> {
            todo!()
        }

        fn remove_collection_item(&self, path: PathBuf) -> Result<()> {
            todo!()
        }
    }

    struct MockCollectionRequestSubstore {}

    impl CollectionRequestSubstore for MockCollectionRequestSubstore {}

    #[test]
    fn collections() {
        let fs = Arc::new(DiskFileSystem::new());
        let collection_store = Arc::new(MockCollectionMetadataStore {});
        let collection_request_substore = Arc::new(MockCollectionRequestSubstore {});
        let indexer = Arc::new(IndexingService::new(fs.clone()));

        let service =
            CollectionService::new(fs, collection_store, collection_request_substore, indexer)
                .unwrap();

        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let result: Arc<DashMap<PathBuf, CollectionHandle>> =
                    service.collections().await.unwrap();

                for item in result.iter() {
                    dbg!(item.key());
                }
            });
    }

    #[test]
    fn overview_collection() {
        let fs = Arc::new(DiskFileSystem::new());
        let collection_store = Arc::new(MockCollectionMetadataStore {});
        let collection_request_substore = Arc::new(MockCollectionRequestSubstore {});
        let indexer = Arc::new(IndexingService::new(fs.clone()));

        let service =
            CollectionService::new(fs, collection_store, collection_request_substore, indexer)
                .unwrap();

        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let result = service.overview_collections().await.unwrap();

                dbg!(&result);
            });
    }

    #[test]
    fn index_collection() {
        let fs = Arc::new(DiskFileSystem::new());
        let collection_store = Arc::new(MockCollectionMetadataStore {});
        let collection_request_substore = Arc::new(MockCollectionRequestSubstore {});
        let indexer = Arc::new(IndexingService::new(fs.clone()));

        let service =
            CollectionService::new(fs, collection_store, collection_request_substore, indexer)
                .unwrap();

        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let result = service
                    .index_collection(PathBuf::from(TEST_COLLECTION_PATH))
                    .await
                    .unwrap();

                for (path, handle) in result.requests.read().iter() {
                    dbg!(String::from_utf8_lossy(&path).to_string());
                    dbg!(handle.state.variants.read());
                }
            });
    }
}
