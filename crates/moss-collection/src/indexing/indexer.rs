use anyhow::{anyhow, Result};
use moss_fs::ports::FileSystem;
use patricia_tree::PatriciaMap;
use std::{collections::HashMap, ffi::OsString, path::PathBuf, sync::Arc};

use crate::{
    indexing::Indexer,
    models::{
        collection::RequestType,
        indexing::{IndexedCollection, RequestEntry, RequestVariantEntry},
    },
};

const REQUESTS_DIR: &'static str = "requests";
const REQUEST_DIR_EXT: &'static str = "request";
const REQUEST_FILE_EXT: &'static str = "sapic";

pub struct IndexerImpl {
    fs: Arc<dyn FileSystem>,
}

#[async_trait::async_trait]
impl Indexer for IndexerImpl {
    async fn index(&self, path: &PathBuf) -> Result<IndexedCollection> {
        Ok(IndexedCollection {
            requests: self.index_requests(path.join(REQUESTS_DIR)).await?,
        })
    }
}

impl IndexerImpl {
    pub fn new(fs: Arc<dyn FileSystem>) -> Self {
        Self { fs }
    }

    async fn index_requests(&self, root: PathBuf) -> Result<PatriciaMap<RequestEntry>> {
        let mut result = PatriciaMap::new();
        let mut stack: Vec<PathBuf> = vec![root.clone()];

        while let Some(current_dir) = stack.pop() {
            let mut dir = self.fs.read_dir(&current_dir).await?;

            while let Some(entry) = dir.next_entry().await? {
                let file_type = entry.file_type().await?;
                if !file_type.is_dir() {
                    continue;
                }

                let path = entry.path();

                if path
                    .extension()
                    .map(|ext| ext == REQUEST_DIR_EXT)
                    .unwrap_or(false)
                {
                    if let Ok(relative_path) = path.strip_prefix(&root) {
                        let key = relative_path.to_string_lossy().into_owned();
                        let request_entry = self.index_request_dir(path).await?;
                        result.insert(key, request_entry);
                    } else {
                        // TODO: log error
                        continue;
                    }
                } else {
                    stack.push(path);
                }
            }
        }

        Ok(result)
    }

    async fn index_request_dir(&self, path: PathBuf) -> Result<RequestEntry> {
        let folder_name = path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow!("Failed to read the request folder name"))?;

        let mut request_entry = RequestEntry {
            name: get_request_name(folder_name)?,
            typ: None,
            path: None,
            variants: HashMap::new(),
        };

        let mut inner_dir = self.fs.read_dir(&path).await?;

        while let Some(inner_entry) = inner_dir.next_entry().await? {
            let file_path = inner_entry.path();
            let file_metadata = inner_entry.metadata().await?;

            if !file_metadata.is_file() || !is_sapic_file(&file_path) {
                continue;
            }

            let file_name = if let Some(name) = file_path.file_name() {
                name.to_owned()
            } else {
                // TODO: logging?
                println!("Failed to read the request file name");
                continue;
            };

            // let request_typ = match get_request_type(&file_name) {
            //     Ok(typ) => typ,
            //     Err(err) => {
            //         // TODO: logging?
            //         println!("Failed to get the request type: {}", err);
            //         continue;
            //     }
            // };

            let parse_output = parse_request_folder_name(file_name)?;

            if !parse_output.file_type.is_variant() {
                request_entry.path = Some(file_path);
                request_entry.typ = Some(parse_output.file_type);
            } else {
                request_entry.variants.insert(
                    file_path,
                    RequestVariantEntry {
                        name: parse_output.name,
                    },
                );
            }
        }

        Ok(request_entry)
    }
}

fn is_sapic_file(file_path: &PathBuf) -> bool {
    file_path
        .extension()
        .map(|ext| ext == REQUEST_FILE_EXT)
        .unwrap_or(false)
}

struct RequestFolderParseOutput {
    name: String,
    file_type: RequestType,
}

fn parse_request_folder_name(file_name: OsString) -> Result<RequestFolderParseOutput> {
    let file_name_str = file_name
        .to_str()
        .ok_or_else(|| anyhow!("failed to retrieve the request filename"))?;

    let mut segments = file_name_str.split('.');

    let name = segments
        .next()
        .ok_or_else(|| anyhow!("failed to retrieve the request name"))?
        .to_string();

    let file_type_str = segments
        .next()
        .ok_or_else(|| anyhow!("failed to retrieve the request type"))?;

    Ok(RequestFolderParseOutput {
        name,
        file_type: RequestType::try_from(file_type_str)?,
    })
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
                let r = IndexerImpl::new(Arc::new(DiskFileSystem::new()));
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

        for (_k, v) in map.iter_prefix(b"foo/") {
            dbg!(v);
        }
    }
}
