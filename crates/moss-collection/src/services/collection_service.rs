use anyhow::Result;
use dashmap::DashMap;
use moss_app::service::Service;
use moss_fs::FileSystem;
use std::{path::PathBuf, sync::Arc};

use crate::domain::{
    models::{CollectionDetails, CollectionSource},
    ports::{
        collection_ports::CollectionIndexer,
        db_ports::{CollectionRequestSubstore, CollectionStore},
    },
};

pub enum CollectionHandle {
    Local {
        fs: Arc<dyn FileSystem>,
        repo: Arc<dyn CollectionRequestSubstore>,
        order: usize,
    },

    Remote {},
}

pub struct CollectionService {
    fs: Arc<dyn FileSystem>,
    collection_store: Arc<dyn CollectionStore>,
    collection_request_substore: Arc<dyn CollectionRequestSubstore>,
    collections: DashMap<CollectionSource, CollectionHandle>,
    indexer: Arc<dyn CollectionIndexer>,
}

impl CollectionService {
    pub fn new(
        fs: Arc<dyn FileSystem>,
        collection_store: Arc<dyn CollectionStore>,
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
    pub async fn create_collection(&self, path: PathBuf) -> Result<()> {
        self.fs.create_dir(path.as_path()).await?;

        let source = CollectionSource::Local(path);
        let order = self.collections.len() + 1;

        self.collection_store
            .put_collection_item(CollectionDetails {
                order,
                source: source.clone(),
            })?;

        self.collections.insert(
            source,
            CollectionHandle::Local {
                fs: Arc::clone(&self.fs),
                repo: Arc::clone(&self.collection_request_substore),
                order,
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
