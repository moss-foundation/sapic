use serde::{Deserialize, Serialize};
// use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum CollectionKind {
    Local,
    Remote,
}

// pub trait AnyCollection {
//     fn source(&self) -> String;
//     fn kind(&self) -> CollectionKind;
// }

// pub struct Collection {
//     inner: Box<dyn AnyCollection>,
// }

// impl Collection {
//     pub fn new(collection: impl AnyCollection + 'static) -> Self {
//         Self {
//             inner: Box::new(collection),
//         }
//     }
// }

// impl AnyCollection for Collection {
//     fn source(&self) -> String {
//         self.inner.source()
//     }

//     fn kind(&self) -> CollectionKind {
//         self.inner.kind()
//     }
// }

// pub trait FileSystemCollectionRead {
//     fn read();
// }

// pub trait FileSystemCollectionWrite {}

// // pub enum Collection {
// //     Local(LocalCollection<LocalFileSystem>),
// //     Remote(RemoteCollection<RemoteFileSystem>),
// // }

// // impl Collection {
// //     pub fn source(&self) -> String {
// //         match self {
// //             Collection::Local(local_collection) => {
// //                 local_collection.path().to_string_lossy().to_string()
// //             }
// //             Collection::Remote(remote_collection) => remote_collection.url().to_string(),
// //         }
// //     }
// // }

// // Local Collection

// pub struct LocalFileSystem {}

// impl FileSystemCollectionRead for LocalFileSystem {
//     fn read() {}
// }

// impl FileSystemCollectionWrite for LocalFileSystem {}

// pub struct LocalCollection<F: FileSystemCollectionRead + FileSystemCollectionWrite> {
//     path: PathBuf,
//     fs: F,
// }

// impl<F: FileSystemCollectionRead + FileSystemCollectionWrite> AnyCollection for LocalCollection<F> {
//     fn source(&self) -> String {
//         self.path().to_string_lossy().to_string()
//     }

//     fn kind(&self) -> CollectionKind {
//         CollectionKind::Local
//     }
// }

// impl<F: FileSystemCollectionRead + FileSystemCollectionWrite> LocalCollection<F> {
//     pub fn new(fs: F, path: PathBuf) -> Self {
//         Self { path, fs }
//     }

//     pub fn path(&self) -> &PathBuf {
//         &self.path
//     }
// }

// // Remote Collection

// pub struct RemoteFileSystem {}

// impl FileSystemCollectionRead for RemoteFileSystem {
//     fn read() {}
// }

// pub struct RemoteCollection<F: FileSystemCollectionRead> {
//     url: String,
//     fs: F,
// }

// impl<F: FileSystemCollectionRead> AnyCollection for RemoteCollection<F> {
//     fn source(&self) -> String {
//         self.url().to_string()
//     }

//     fn kind(&self) -> CollectionKind {
//         CollectionKind::Remote
//     }
// }

// impl<F: FileSystemCollectionRead> RemoteCollection<F> {
//     pub fn url(&self) -> &str {
//         &self.url
//     }
// }
