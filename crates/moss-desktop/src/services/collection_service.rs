use anyhow::Result;
use dashmap::DashMap;
use moss_collection::collection::{CollectionKind, LocalCollection};
use std::{any::Any, path::PathBuf, sync::Arc};

use crate::app::{
    models::collection::PutCollectionInput,
    repositories::collection_repository::{CollectionRepository, CollectionRequestRepository},
    service::Service,
};

pub trait FileSystem: Send + Sync + 'static {
    fn create_dir(&self, path: &PathBuf) -> Result<()>;
    fn remove_dir(&self, path: &PathBuf) -> Result<()>;
}

pub struct CollectionHandle {
    kind: CollectionKind,
    inner: Arc<dyn Any + Send + Sync>,
}

impl CollectionHandle {
    pub fn new(inner: impl Any + Send + Sync, kind: CollectionKind) -> Self {
        Self {
            kind,
            inner: Arc::new(inner),
        }
    }
}

pub struct LocalCollectionHandle<FS, R>
where
    FS: FileSystem,
    R: CollectionRequestRepository,
{
    fs: Arc<FS>,
    repo: Arc<R>,
    collection: LocalCollection,
}

pub struct CollectionService<R, FS>
where
    R: CollectionRepository + CollectionRequestRepository + 'static,
    FS: FileSystem + 'static,
{
    repo: Arc<R>,
    fs: Arc<FS>,
    collections: DashMap<String, CollectionHandle>,
}

impl<R, FS> CollectionService<R, FS>
where
    R: CollectionRepository + CollectionRequestRepository,
    FS: FileSystem,
{
    pub fn new(fs: Arc<FS>, repo: Arc<R>) -> Result<Self> {
        Ok(Self {
            repo,
            fs,
            collections: DashMap::new(),
        })
    }
}

impl<R, FS> CollectionService<R, FS>
where
    R: CollectionRepository + CollectionRequestRepository,
    FS: FileSystem,
{
    pub fn create_collection(&self, path: PathBuf) -> Result<()> {
        self.fs.create_dir(&path)?;

        let source = path.to_string_lossy().to_string();
        let kind = CollectionKind::Local;

        self.repo.put_collection_item(PutCollectionInput {
            source: source.clone(),
            kind,
            order: self.collections.len() + 1,
        })?;

        self.collections.insert(
            source,
            CollectionHandle::new(
                LocalCollectionHandle {
                    fs: Arc::clone(&self.fs),
                    repo: Arc::clone(&self.repo),
                    collection: LocalCollection { path },
                },
                kind,
            ),
        );

        Ok(())
    }
}

impl<R, FS> Service for CollectionService<R, FS>
where
    R: CollectionRepository + CollectionRequestRepository + 'static,
    FS: FileSystem + 'static,
{
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    fn dispose(&self) {}

    fn as_any(&self) -> &(dyn std::any::Any + Send) {
        self
    }
}
