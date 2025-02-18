use anyhow::Result;
use dashmap::DashMap;
use moss_app::service::Service;
use std::{any::Any, path::PathBuf, sync::Arc};

use crate::domain::{
    models::{CollectionDetails, CollectionSource},
    ports::db_ports::{CollectionRequestSubstore, CollectionStore},
};

pub trait FileSystem: Send + Sync + 'static {
    fn create_dir(&self, path: &PathBuf) -> Result<()>;
    fn remove_dir(&self, path: &PathBuf) -> Result<()>;
}

pub enum CollectionHandle {
    Local {
        fs: Arc<dyn FileSystem + 'static>,
        repo: Arc<dyn CollectionRequestSubstore + 'static>,
        order: usize,
    },

    Remote {},
}

pub struct CollectionService {
    fs: Arc<dyn FileSystem + 'static>,
    collection_store: Arc<dyn CollectionStore>,
    collection_request_substore: Arc<dyn CollectionRequestSubstore>,
    collections: DashMap<CollectionSource, CollectionHandle>,
}

impl CollectionService {
    pub fn new(
        fs: Arc<dyn FileSystem>,
        collection_store: Arc<dyn CollectionStore>,
        collection_request_substore: Arc<dyn CollectionRequestSubstore>,
    ) -> Result<Self> {
        Ok(Self {
            fs,
            collection_store,
            collection_request_substore,
            collections: DashMap::new(),
        })
    }
}

impl CollectionService {
    pub fn create_collection(&self, path: PathBuf) -> Result<()> {
        self.fs.create_dir(&path)?;

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
