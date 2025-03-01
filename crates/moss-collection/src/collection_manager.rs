use anyhow::{anyhow, Result};
use dashmap::DashMap;
use moss_app::service::AppService;
use moss_fs::ports::FileSystem;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::OnceCell;

use crate::{
    collection_handle::{CollectionHandle, CollectionState},
    indexing::CollectionIndexer,
    models::{
        collection::CollectionRequestVariantEntry,
        operations::collection_operations::{CreateCollectionInput, OverviewCollectionOutput},
        storage::CollectionMetadataEntity,
    },
    request_handle::{RequestHandle, RequestState},
    storage::{CollectionMetadataStore, CollectionRequestSubstore},
};

type CollectionMap = DashMap<PathBuf, CollectionHandle>;

pub struct CollectionManager {
    fs: Arc<dyn FileSystem>,
    collection_store: Arc<dyn CollectionMetadataStore>,
    collection_request_substore: Arc<dyn CollectionRequestSubstore>,
    collections: OnceCell<Arc<CollectionMap>>,
    indexer: Arc<dyn CollectionIndexer>,
}

impl CollectionManager {
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

impl CollectionManager {
    async fn collections(&self) -> Result<Arc<CollectionMap>> {
        let collections = self
            .collections
            .get_or_try_init(|| async move {
                let collections = DashMap::new();

                for (collection_path, collection_metadata) in
                    self.collection_store.get_all_items()?
                {
                    let collection_name = match collection_path.file_name() {
                        Some(name) => name.to_string_lossy().to_string(),
                        None => {
                            // TODO: logging
                            println!("failed to get the collection {:?} name", collection_path);
                            continue;
                        }
                    };

                    collections.insert(
                        collection_path,
                        CollectionHandle::new(
                            Arc::clone(&self.fs),
                            Arc::clone(&self.collection_request_substore),
                            collection_name,
                            collection_metadata.order,
                        ),
                    );
                }

                Ok::<Arc<CollectionMap>, anyhow::Error>(Arc::new(collections))
            })
            .await?;

        Ok(Arc::clone(collections))
    }
}

impl CollectionManager {
    pub async fn overview_collections(&self) -> Result<Vec<OverviewCollectionOutput>> {
        let collections = self.collections().await?;

        Ok(collections
            .iter()
            .map(|item| {
                let item_state = item.state();

                OverviewCollectionOutput {
                    name: item_state.name.clone(),
                    path: item.key().clone(),
                    order: item_state.order,
                }
            })
            .collect())
    }

    // TODO: At the moment, there is no clear understanding of the format in which
    // collection descriptions should be sent to the frontend. This largely depends
    // on the library we choose to use for displaying hierarchical structures.

    pub(crate) async fn index_collection(&self, path: PathBuf) -> Result<Arc<CollectionState>> {
        let collections = self.collections().await?;
        let collection_handle = collections
            .get(&path)
            .ok_or_else(|| anyhow!("Collection with path {:?} not found", path))?;
        let collection_state = collection_handle.state();

        let indexed_collection = self.indexer.index(&path).await?;

        for (raw_request_path, indexed_request_entry) in indexed_collection.requests {
            let request_handle =
                collection_state.get_request_handle_or_init(&raw_request_path, || {
                    RequestHandle::new(
                        Arc::clone(&self.fs),
                        RequestState {
                            name: indexed_request_entry.name,
                            order: None,
                            typ: indexed_request_entry.typ,
                            variants: Default::default(),
                        },
                    )
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
            CollectionHandle::new(
                Arc::clone(&self.fs),
                Arc::clone(&self.collection_request_substore),
                input.name,
                None,
            ),
        );

        Ok(())
    }
}

impl AppService for CollectionManager {
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
    use std::collections::HashMap;

    use moss_fs::adapters::disk::DiskFileSystem;

    use crate::{
        indexing::indexer::IndexingService,
        models::storage::RequestMetadataEntity,
        storage::{MockCollectionMetadataStore, MockCollectionRequestSubstore},
    };

    use super::*;

    const TEST_COLLECTION_PATH: &'static str =
        "/Users/g10z3r/Project/keenawa-co/sapic/crates/moss-collection/tests/TestCollection";

    const TEST_REQUEST_PATH: &'static str =
        "/Users/g10z3r/Project/keenawa-co/sapic/crates/moss-collection/tests/TestCollection/requests/Test1.request";

    #[test]
    fn collections() {
        let fs = Arc::new(DiskFileSystem::new());
        let mut collection_store = MockCollectionMetadataStore::new();
        collection_store.expect_get_all_items().returning(|| {
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
        });

        let collection_request_substore = MockCollectionRequestSubstore::new();
        let indexer = Arc::new(IndexingService::new(fs.clone()));
        let service = CollectionManager::new(
            fs,
            Arc::new(collection_store),
            Arc::new(collection_request_substore),
            indexer,
        )
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
        let mut collection_store = MockCollectionMetadataStore::new();
        collection_store.expect_get_all_items().returning(|| {
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
        });

        let collection_request_substore = MockCollectionRequestSubstore::new();
        let indexer = Arc::new(IndexingService::new(fs.clone()));
        let service = CollectionManager::new(
            fs,
            Arc::new(collection_store),
            Arc::new(collection_request_substore),
            indexer,
        )
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
        let mut collection_store = MockCollectionMetadataStore::new();
        collection_store.expect_get_all_items().returning(|| {
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
        });

        let collection_request_substore = MockCollectionRequestSubstore::new();
        let indexer = Arc::new(IndexingService::new(fs.clone()));
        let service = CollectionManager::new(
            fs,
            Arc::new(collection_store),
            Arc::new(collection_request_substore),
            indexer,
        )
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
