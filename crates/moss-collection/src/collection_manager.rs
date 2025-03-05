use anyhow::{anyhow, Result};
use dashmap::DashMap;
use moss_app::service::AppService;
use moss_fs::ports::{FileSystem, RemoveOptions, RenameOptions};
use std::collections::HashMap;
use std::{path::PathBuf, sync::Arc};
use thiserror::Error;
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

// TODO: Testing the performance impact of RwLock in this case
// Earlier we had used DashMap, but it doesn't work well in an async context
type CollectionMap = tokio::sync::RwLock<HashMap<PathBuf, CollectionHandle>>;

#[derive(Clone, Debug, Error)]
pub enum CollectionOperationError {
    #[error("The name of a collection cannot be empty.")]
    EmptyName,
    #[error("`{name}` is an invalid name for a collection.")]
    InvalidName { name: String }, // TODO: validate name
    #[error("A collection named {name} already exists in {path}.")]
    DuplicateName { name: String, path: PathBuf },
    #[error("The collection named `{name}` does not exist in {path}")]
    NonexistentCollection { name: String, path: PathBuf },
}

pub struct CollectionManager {
    fs: Arc<dyn FileSystem>,
    collection_store: Arc<dyn CollectionMetadataStore>,
    // TODO: extract request store
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
                let mut collections = HashMap::new();

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

                Ok::<Arc<CollectionMap>, anyhow::Error>(Arc::new(tokio::sync::RwLock::new(
                    collections,
                )))
            })
            .await?;

        Ok(Arc::clone(collections))
    }
}

impl CollectionManager {
    pub async fn overview_collections(&self) -> Result<Vec<OverviewCollectionOutput>> {
        let collections = self.collections().await?;
        let read_lock = collections.read().await;
        Ok(read_lock
            .iter()
            .map(|item| {
                let item_state = item.1.state();

                OverviewCollectionOutput {
                    name: item_state.name.clone(),
                    path: item.0.clone(),
                    order: item_state.order,
                }
            })
            .collect())
    }

    // TODO: At the moment, there is no clear understanding of the format in which
    // collection descriptions should be sent to the frontend. This largely depends
    // on the library we choose to use for displaying hierarchical structures.

    pub async fn index_collection(&self, path: PathBuf) -> Result<Arc<CollectionState>> {
        let collections = self.collections().await?;
        let read_lock = collections.read().await;

        let collection_handle = (*read_lock)
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
        if input.name.trim().is_empty() {
            return Err(CollectionOperationError::EmptyName.into());
        }
        let full_path = input.path.join(&input.name);
        let collections = self.collections().await?;
        {
            let read_lock = collections.read().await;
            if read_lock.contains_key(&full_path) {
                return Err(CollectionOperationError::DuplicateName {
                    name: input.name,
                    path: full_path,
                }
                .into());
            }
        }
        self.fs.create_dir(&full_path).await?;
        // TODO: init repo

        self.collection_store.put_collection_item(
            full_path.clone(),
            CollectionMetadataEntity {
                order: None,
                requests: Default::default(),
            },
        )?;

        {
            let mut write_lock = collections.write().await;
            (*write_lock).insert(
                full_path.clone(),
                CollectionHandle::new(
                    Arc::clone(&self.fs),
                    Arc::clone(&self.collection_request_substore),
                    input.name,
                    None,
                ),
            );
        }

        Ok(())
    }

    // TODO: In the future, we need to test the impact of this on the user experience
    // Since we use the full path as the PatriciaMap's key
    // Renaming a collection is potentially a very heavy operation
    // Which requires rebuilding the entire PatriciaMap
    pub async fn rename_collection(&self, path_buf: PathBuf, new_name: &str) -> Result<()> {
        if new_name.trim().is_empty() {
            return Err(CollectionOperationError::EmptyName.into());
        }
        let collections = self.collections().await?;
        // FIXME: Is this checking necessary?
        {
            let read_lock = collections.read().await;
            if !read_lock.contains_key(&path_buf) {
                let name = path_buf.file_name().unwrap();
                return Err(CollectionOperationError::NonexistentCollection {
                    name: name.to_string_lossy().to_string(),
                    path: path_buf,
                }
                .into());
            }
        }

        let new_path = path_buf.parent().unwrap().join(&new_name);
        self.fs
            .rename(&path_buf, &new_path, RenameOptions::default())
            .await?;

        let metadata = self
            .collection_store
            .remove_collection_item(path_buf.clone())?;
        self.collection_store
            .put_collection_item(new_path.clone(), metadata)?;

        // Updating the key for every request within the collection
        {
            let mut write_lock = collections.write().await;

            let handle = (*write_lock).remove(&path_buf).unwrap();
            let state = handle.state();
            let requests = state
                .requests
                .read()
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect::<Vec<_>>();

            for (old_key, req_handle) in requests {
                // TODO: I'm not sure if this logic is robust enough?
                let req_relative_path =
                    PathBuf::from(String::from_utf8_lossy(&old_key).to_string())
                        .strip_prefix(&path_buf)?
                        .to_path_buf();
                let new_prefix = &path_buf.parent().unwrap().join(&new_name);
                let new_path = new_prefix.join(req_relative_path);
                println!("{}", new_path.to_string_lossy());

                let mut write_lock = state.requests.write();
                (*write_lock).remove(old_key);
                (*write_lock).insert(new_path.to_string_lossy().to_string(), req_handle);
            }

            (*write_lock).insert(new_path.clone(), handle);
        }

        Ok(())
    }

    pub async fn delete_collection(&self, path_buf: PathBuf) -> Result<()> {
        match path_buf.try_exists() {
            Ok(true) => {
                self.fs
                    .remove_dir(
                        &path_buf,
                        RemoveOptions {
                            recursive: true,
                            ignore_if_not_exists: true,
                        },
                    )
                    .await?;
            }
            Ok(false) => {
                // TODO: Logging this anormaly, the collection has already been deleted in the filesystem
            }
            Err(e) => {
                // This is likely a permission issue
                return Err(e.into());
            }
        }

        let collections = self.collections().await?;
        {
            let read_lock = collections.read().await;
            if !read_lock.contains_key(&path_buf) {
                // TODO: Logging this anormaly, the collection is already deleted from the map
            }
        }

        let _ = self
            .collection_store
            .remove_collection_item(path_buf.clone())?;
        {
            let mut write_lock = collections.write().await;
            (*write_lock).remove(&path_buf);
        }

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
    use moss_fs::adapters::disk::DiskFileSystem;
    use std::collections::HashMap;

    use super::*;
    use crate::models::operations::collection_operations::CreateRequestInput;
    use crate::{
        indexing::indexer::IndexingService,
        models::storage::RequestMetadataEntity,
        storage::{MockCollectionMetadataStore, MockCollectionRequestSubstore},
    };

    const TEST_COLLECTION_PATH: &'static str = "TestCollection";

    const TEST_REQUEST_PATH: &'static str = "TestCollection/requests/Test1.request";
    // FIXME: I have not figured out how automock works and the best way to use it
    // For easier testing on my part I'll try to manually create test structures for them
    struct TestCollectionMetadataStore {
        collections: DashMap<PathBuf, CollectionMetadataEntity>,
    }

    impl TestCollectionMetadataStore {
        pub fn new() -> Self {
            Self {
                collections: DashMap::new(),
            }
        }
    }
    impl CollectionMetadataStore for TestCollectionMetadataStore {
        fn get_all_items(&self) -> Result<Vec<(PathBuf, CollectionMetadataEntity)>> {
            Ok(self
                .collections
                .iter()
                .map(|ref_multi| (ref_multi.key().clone(), ref_multi.value().clone()))
                .collect())
        }

        fn put_collection_item(&self, path: PathBuf, item: CollectionMetadataEntity) -> Result<()> {
            self.collections.insert(path.clone(), item);
            Ok(())
        }

        fn remove_collection_item(&self, path: PathBuf) -> Result<CollectionMetadataEntity> {
            if let Some((_k, v)) = self.collections.remove(&path) {
                Ok(v)
            } else {
                Err(anyhow!(
                    "{} not found in CollectionMetadataStore",
                    path.to_string_lossy()
                ))
            }
        }
    }

    struct TestCollectionRequestSubstore {}

    impl TestCollectionRequestSubstore {
        pub fn new() -> Self {
            Self {}
        }
    }
    impl CollectionRequestSubstore for TestCollectionRequestSubstore {}

    fn generate_test_service() -> CollectionManager {
        let fs = Arc::new(DiskFileSystem::new());
        let collection_store = TestCollectionMetadataStore::new();
        let collection_request_substore = TestCollectionRequestSubstore::new();
        let indexer = Arc::new(IndexingService::new(fs.clone()));
        CollectionManager::new(
            fs,
            Arc::new(collection_store),
            Arc::new(collection_request_substore),
            indexer,
        )
        .unwrap()
    }

    #[test]
    fn test_create_collection() {
        let service = generate_test_service();
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                service
                    .create_collection(CreateCollectionInput {
                        name: "TestCollection".to_string(),
                        path: "Collections".into(),
                        repo: None,
                    })
                    .await
                    .unwrap();
                let collections = service.collections().await.unwrap();
                let read_lock = collections.read().await;
                assert!(
                    (*read_lock).contains_key(&PathBuf::from("Collections").join("TestCollection"))
                );
            });
    }

    #[test]
    fn test_rename_collection() {
        let service = generate_test_service();
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                service
                    .create_collection(CreateCollectionInput {
                        name: "Pre-renaming".to_string(),
                        path: "Collections".into(),
                        repo: None,
                    })
                    .await
                    .unwrap();
                let old_collection_path = PathBuf::from("Collections").join("Pre-renaming");
                let collections = service.collections().await.unwrap();
                {
                    let mut write_lock = collections.read().await;

                    let handle = (*write_lock).get(&old_collection_path).unwrap();
                    handle
                        .create_request(
                            &old_collection_path,
                            None,
                            CreateRequestInput {
                                name: "Test".to_string(),
                                url: None,
                                payload: None,
                            },
                        )
                        .await
                        .unwrap();
                }
                let new_collection_path = PathBuf::from("Collections").join("Post-renaming");
                service
                    .rename_collection(old_collection_path.clone(), "Post-renaming")
                    .await
                    .unwrap();
                {
                    let read_lock = collections.read().await;
                    assert!(!(*read_lock).contains_key(&old_collection_path));
                    assert!((*read_lock).contains_key(&new_collection_path));
                    let collection = (*read_lock).get(&new_collection_path).unwrap();
                    let new_request_path =
                        new_collection_path.join("requests").join("Test.request");
                    assert!(collection
                        .state()
                        .requests
                        .read()
                        .contains_key(new_request_path.to_string_lossy().to_string()))
                }
            });
    }

    #[test]
    fn test_delete_collection() {
        let service = generate_test_service();
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                service
                    .create_collection(CreateCollectionInput {
                        name: "ToBeDeleted".to_string(),
                        path: "Collections".into(),
                        repo: None,
                    })
                    .await
                    .unwrap();
                let path = PathBuf::from("Collections").join("ToBeDeleted");
                service.delete_collection(path.clone()).await.unwrap();
                let collections = service.collections().await.unwrap();
                {
                    let read_lock = collections.read().await;
                    assert!(!(*read_lock).contains_key(&path));
                }
            });
    }

    // #[test]
    // fn collections() {
    //     let fs = Arc::new(DiskFileSystem::new());
    //     let mut collection_store = MockCollectionMetadataStore::new();
    //     collection_store.expect_get_all_items().returning(|| {
    //         Ok(vec![(
    //             PathBuf::from(TEST_COLLECTION_PATH),
    //             CollectionMetadataEntity {
    //                 order: None,
    //                 requests: {
    //                     let mut this = HashMap::new();
    //                     this.insert(
    //                         TEST_REQUEST_PATH.into(),
    //                         RequestMetadataEntity {
    //                             order: None,
    //                             variants: Default::default(),
    //                         },
    //                     );
    //
    //                     this
    //                 },
    //             },
    //         )])
    //     });
    //
    //     let collection_request_substore = MockCollectionRequestSubstore::new();
    //     let indexer = Arc::new(IndexingService::new(fs.clone()));
    //     let service = CollectionManager::new(
    //         fs,
    //         Arc::new(collection_store),
    //         Arc::new(collection_request_substore),
    //         indexer,
    //     )
    //     .unwrap();
    //
    //     tokio::runtime::Builder::new_multi_thread()
    //         .enable_all()
    //         .build()
    //         .unwrap()
    //         .block_on(async {
    //             let result: Arc<DashMap<PathBuf, CollectionHandle>> =
    //                 service.collections().await.unwrap();
    //
    //             for item in result.iter() {
    //                 dbg!(item.key());
    //             }
    //         });
    // }

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
