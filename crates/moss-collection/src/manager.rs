use dashmap::DashMap;
use std::{any::Any, marker::PhantomData, path::PathBuf, sync::Arc};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::collection::CollectionKind;

pub struct SledClient {
    db: sled::Db,
}

impl SledClient {
    pub fn new(db: sled::Db) -> Self {
        Self { db }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CollectionEntity {
    kind: CollectionKind,
    order: usize,
    // data: Vec<u8>,
}

pub trait CollectionRepository {
    fn put_collection_item(&self, input: PutCollectionInput) -> Result<()>;
}

pub trait CollectionRequestRepository {}

pub struct PutCollectionInput {
    pub source: String,
    pub kind: CollectionKind,
    pub order: usize,
}

impl CollectionRepository for SledClient {
    fn put_collection_item(&self, input: PutCollectionInput) -> Result<()> {
        // let data = match item {
        //     Collection::Local(local_collection) => bincode::serialize(&LocalCollectionEntity {
        //         source: local_collection.path().clone(),
        //     })?,
        //     Collection::Remote(_remote_collection) => unimplemented!(),
        // };

        let collections_tree = self.db.open_tree("collections")?;

        let value = bincode::serialize(&CollectionEntity {
            kind: input.kind,
            order: input.order,
        })?;
        collections_tree.insert(input.source, value)?;

        Ok(())
    }
}

impl CollectionRequestRepository for SledClient {}

pub struct CollectionDescriptor {
    pub source: PathBuf,
    pub order: usize,
}

pub trait FileSystemCollectionRead {}

pub trait FileSystemCollectionWrite {}

//

pub struct LocalFileSystem {}

impl FileSystemCollectionRead for LocalFileSystem {}

impl FileSystemCollectionWrite for LocalFileSystem {}

pub enum Collection {
    Local { path: PathBuf },

    Remote { url: String },
}

pub struct LocalCollection {
    path: PathBuf,
}

pub struct LocalCollectionHandle<FS, R>
where
    FS: FileSystemCollectionRead + FileSystemCollectionWrite,
    R: CollectionRequestRepository,
{
    fs: FS,
    repo: Arc<R>,
    collection: LocalCollection,
}

pub trait AnyCollectionHandle {}

pub struct CollectionHandle {
    kind: CollectionKind,
    inner: Box<dyn Any>,
}

impl CollectionHandle {
    pub fn new(inner: impl Any, kind: CollectionKind) -> Self {
        Self {
            kind,
            inner: Box::new(inner),
        }
    }
}

pub trait CollectionManagerFileSystem:
    FileSystemCollectionRead + FileSystemCollectionWrite
{
    fn create_dir(&self, path: &PathBuf) -> Result<()>;
}

pub struct CollectionManager<R, FS>
where
    R: CollectionRepository + CollectionRequestRepository + 'static,
    FS: CollectionManagerFileSystem,
{
    repo: Arc<R>,
    fs: Arc<FS>,
    collections: DashMap<String, CollectionHandle>,
}

impl<R, FS> CollectionManager<R, FS>
where
    R: CollectionRepository + CollectionRequestRepository + 'static,
    FS: CollectionManagerFileSystem,
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
                    fs: LocalFileSystem {},
                    repo: Arc::clone(&self.repo),
                    collection: LocalCollection { path },
                },
                kind,
            ),
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use dashmap::DashMap;
    use std::sync::Arc;

    use super::{
        CollectionManager, CollectionManagerFileSystem, FileSystemCollectionRead,
        FileSystemCollectionWrite, SledClient,
    };

    pub struct MockLocalFileSystem {}

    impl CollectionManagerFileSystem for MockLocalFileSystem {
        fn create_dir(&self, path: &std::path::PathBuf) -> anyhow::Result<()> {
            todo!()
        }
    }

    impl FileSystemCollectionWrite for MockLocalFileSystem {}
    impl FileSystemCollectionRead for MockLocalFileSystem {}

    #[test]
    fn put_collection() {
        let db: sled::Db = sled::open("my_db").unwrap();
        let client = SledClient::new(db);

        let manager = CollectionManager {
            repo: Arc::new(client),
            fs: Arc::new(MockLocalFileSystem {}),
            collections: DashMap::new(),
        };

        // let collection = Collection::Local(LocalCollection::new(
        //     MockLocalFileSystem {},
        //     "foo/bar/collection1".into(),
        // ));

        // client
        //     .put_collection_item(PutCollectionInput {
        //         source: "foo/bar/collection1".to_string(),
        //         kind: CollectionKind::Local,
        //         order: 1,
        //     })
        //     .unwrap();

        // client.db.get(b"foo/bar/collection1").unwrap();

        // db.insert(b"yo!", b"v1").unwrap();
        // db.remove(b"yo!").unwrap();

        // assert_eq!(&db.get(b"yo!").unwrap().unwrap(), b"v1");
    }
}
