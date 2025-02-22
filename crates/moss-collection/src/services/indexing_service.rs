use anyhow::{anyhow, Result};
use moss_fs::ports::FileSystem;
use patricia_tree::PatriciaMap;
use std::{ffi::OsStr, path::PathBuf, sync::Arc};

use crate::{
    models::{
        collection::RequestType,
        indexing::{IndexedCollection, RequestEntry, RequestVariantEntry},
    },
    ports::collection_ports::CollectionIndexer,
};

pub struct IndexingService {
    fs: Arc<dyn FileSystem>,
}

#[async_trait::async_trait]
impl CollectionIndexer for IndexingService {
    async fn index(&self, path: &PathBuf) -> Result<IndexedCollection> {
        Ok(IndexedCollection {
            requests: self.index_requests(path.join("requests")).await?,
        })
    }
}

impl IndexingService {
    pub fn new(fs: Arc<dyn FileSystem>) -> Self {
        Self { fs }
    }

    async fn index_requests(&self, root: PathBuf) -> Result<PatriciaMap<RequestEntry>> {
        let mut result = PatriciaMap::new();
        let mut stack: Vec<PathBuf> = vec![root];

        while let Some(current_dir) = stack.pop() {
            let mut dir = self.fs.read_dir(&current_dir).await?;

            while let Some(entry) = dir.next_entry().await? {
                let file_type = entry.file_type().await?;
                if !file_type.is_dir() {
                    continue;
                }

                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "request") {
                    result.insert(
                        path.to_string_lossy().to_string(),
                        self.index_request_dir(path).await?,
                    );
                } else {
                    stack.push(path);
                }
            }
        }

        Ok(result)
    }

    // async fn index_requests(&self, root: PathBuf) -> Result<Node<String, RequestEntry>> {
    //     let mut children = Vec::new();
    //     let mut dir = self.fs.read_dir(&root).await?;
    //     let mut tasks = FuturesUnordered::new();

    //     while let Some(entry) = dir.next_entry().await? {
    //         let path = entry.path();
    //         let file_type = entry.file_type().await?;
    //         if !file_type.is_dir() {
    //             continue;
    //         }

    //         let sem_clone = self.concurrency_limit.clone();
    //         tasks.push(async move {
    //             let _permit = sem_clone.acquire_owned().await;
    //             self.index_dir(path).await
    //         });
    //     }

    //     while let Some(child_result) = tasks.next().await {
    //         children.push(child_result?);
    //     }

    //     Ok(children)
    // }

    // async fn index_dir(&self, path: PathBuf) -> Result<RequestIndexEntry> {
    //     if path.extension().map_or(false, |ext| ext == "request") {
    //         let req = self.index_request_dir(path).await?;
    //         return Ok(req);
    //     }

    //     let mut children = Vec::new();
    //     let mut dir = self.fs.read_dir(&path).await?;
    //     let mut tasks = FuturesUnordered::new();

    //     while let Some(entry) = dir.next_entry().await? {
    //         let child_path = entry.path();
    //         let file_type = entry.file_type().await?;
    //         if !file_type.is_dir() {
    //             continue;
    //         }
    //         let sem_clone = self.concurrency_limit.clone();
    //         let task = async move {
    //             let _permit = sem_clone.acquire_owned().await;
    //             self.index_dir(child_path).await
    //         };
    //         tasks.push(task);
    //     }

    //     while let Some(child) = tasks.next().await {
    //         children.push(child?);
    //     }

    //     let folder = DirEntry {
    //         name: path
    //             .file_name()
    //             .map(|s| s.to_string_lossy().to_string())
    //             .unwrap_or_default(),
    //         path,
    //         children,
    //     };
    //     Ok(RequestIndexEntry::Dir(folder))
    // }

    async fn index_request_dir(&self, path: PathBuf) -> Result<RequestEntry> {
        let folder_name = path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow!("Failed to read the request folder name"))?;

        let mut request_entry = RequestEntry {
            name: get_request_name(folder_name)?,
            ext: None,
            path: None,
            variants: Vec::new(),
        };

        let mut inner_dir = self.fs.read_dir(&path).await?;

        while let Some(inner_entry) = inner_dir.next_entry().await? {
            let file_path = inner_entry.path();
            let file_metadata = inner_entry.metadata().await?;

            if !file_metadata.is_file() || !is_sapic_file(&file_path) {
                continue;
            }

            let file_name = if let Some(name) = file_path.file_name() {
                name
            } else {
                // TODO: logging?
                println!("Failed to read the request file name");
                continue;
            };

            let request_typ = match get_request_type(file_name) {
                Ok(typ) => typ,
                Err(err) => {
                    // TODO: logging?
                    println!("Failed to get the request type: {}", err);
                    continue;
                }
            };

            if !request_typ.is_variant() {
                request_entry.path = Some(file_path);
                request_entry.ext = Some(request_typ);
            } else {
                request_entry.variants.push(RequestVariantEntry {
                    name: file_name.to_string_lossy().to_string(),
                    path: file_path,
                });
            }
        }

        Ok(request_entry)
    }
}

fn is_sapic_file(file_path: &PathBuf) -> bool {
    file_path
        .extension()
        .map(|ext| ext == "sapic")
        .unwrap_or(false)
}

fn get_request_type(file_name: &OsStr) -> Result<RequestType> {
    let file_name_str = file_name
        .to_str()
        .ok_or_else(|| anyhow!("failed to retrieve the request filename"))?;

    if let Some(typ) = file_name_str.split('.').nth(1) {
        RequestType::try_from(typ)
    } else {
        Err(anyhow!("failed to retrieve the request type"))
    }
}

fn get_request_name(folder_name: &str) -> Result<String> {
    if let Some(prefix) = folder_name.strip_suffix(".request") {
        Ok(prefix.to_string())
    } else {
        Err(anyhow!(
            "failed to extract the request name from the directory name"
        ))
    }
}

#[cfg(test)]
mod tests {
    use moss_fs::adapters::disk::DiskFileSystem;

    use super::*;

    #[test]
    fn test() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let r = IndexingService::new(Arc::new(DiskFileSystem::new()));
                let r = r
                    .index(&PathBuf::from("./tests/TestCollection"))
                    .await
                    .unwrap();

                dbg!(r);
            });
    }

    #[test]
    fn test_2() {
        let mut map = PatriciaMap::new();
        map.insert("foo/bar/collection", 1);
        map.insert("foo/some/test", 2);
        map.insert("baz/collection", 3);

        for (k, v) in map.iter_prefix(b"foo/") {
            dbg!(v);
        }
    }
}
