use anyhow::{anyhow, Result};
use moss_fs::FileSystem;
use std::{ffi::OsStr, path::PathBuf, sync::Arc};

#[derive(Debug)]
pub enum HttpFileTypeExt {
    Post,
    Get,
    Put,
    Delete,
}

#[derive(Debug)]
pub enum RequestFileTypeExt {
    Http(HttpFileTypeExt),
    WebSocket,
    Graphql,
    Grpc,
    Variant,
}

impl TryFrom<&str> for RequestFileTypeExt {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "post" => Ok(Self::Http(HttpFileTypeExt::Post)),
            "get" => Ok(Self::Http(HttpFileTypeExt::Get)),
            "put" => Ok(Self::Http(HttpFileTypeExt::Put)),
            "delete" => Ok(Self::Http(HttpFileTypeExt::Delete)),

            "ws" => Ok(Self::WebSocket),
            "graphql" => Ok(Self::WebSocket),
            "grpc" => Ok(Self::WebSocket),

            "variant" => Ok(Self::Variant),

            _ => Err(anyhow!("unknown request file type extension: {}", value)),
        }
    }
}

impl RequestFileTypeExt {
    fn is_http(&self) -> bool {
        match self {
            RequestFileTypeExt::Http(_) => true,
            _ => false,
        }
    }

    fn is_variant(&self) -> bool {
        match self {
            RequestFileTypeExt::Variant => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct RequestVariantEntry {
    name: String,
    path: PathBuf,
}

#[derive(Debug)]
pub struct RequestEntry {
    name: String,
    ext: Option<RequestFileTypeExt>,
    path: Option<PathBuf>,
    variants: Vec<RequestVariantEntry>,
}

#[derive(Debug)]
pub enum RequestIndexEntry {
    Request(RequestEntry),

    Folder {
        name: String,
        path: PathBuf,
        children: Vec<RequestIndexEntry>,
    },
}

#[derive(Debug)]
pub struct IndexedCollection {
    name: Option<String>,
    requests: Vec<RequestIndexEntry>,
}

pub struct IndexingService {
    fs: Arc<dyn FileSystem>,
}

struct HandleRequestFileOutput {
    ext: RequestFileTypeExt,
    is_request_file: bool,
}

impl IndexingService {
    pub fn new(fs: Arc<dyn FileSystem>) -> Self {
        Self { fs }
    }

    pub async fn index(&self, path: &PathBuf) -> Result<IndexedCollection> {
        Ok(IndexedCollection {
            name: path
                .file_name()
                .and_then(|s| Some(s.to_string_lossy().to_string())),
            requests: self.index_requests(path.join("requests")).await?,
        })
    }

    async fn index_requests(&self, path: PathBuf) -> Result<Vec<RequestIndexEntry>> {
        let mut tasks = Vec::new();
        let mut stack = vec![path];

        while let Some(current_path) = stack.pop() {
            let mut dir = tokio::fs::read_dir(current_path).await?;

            while let Some(entry) = dir.next_entry().await? {
                let path = entry.path();
                let metadata = entry.metadata().await?;
                if !metadata.is_dir() {
                    continue;
                }

                if path.extension().is_some_and(|value| value == "request") {
                    tasks.push(self.index_request_dir(path.clone()));
                } else {
                    stack.push(path);
                }
            }
        }

        futures::future::join_all(tasks).await.into_iter().collect()
    }

    async fn index_request_dir(&self, path: PathBuf) -> Result<RequestIndexEntry> {
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

        let mut inner_dir = tokio::fs::read_dir(&path).await?;

        while let Some(inner_entry) = inner_dir.next_entry().await? {
            let file_path = inner_entry.path();
            let file_metadata = inner_entry.metadata().await?;

            if file_metadata.is_file() && !is_sapic_file(&file_path) {
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

        Ok(RequestIndexEntry::Request(request_entry))
    }
}

fn is_sapic_file(file_path: &PathBuf) -> bool {
    file_path
        .extension()
        .map(|ext| ext == "sapic")
        .unwrap_or(false)
}

fn get_request_type(file_name: &OsStr) -> Result<RequestFileTypeExt> {
    let file_name_str = file_name
        .to_str()
        .ok_or_else(|| anyhow!("failed to retrieve the request filename"))?;

    if let Some(typ) = file_name_str.split('.').nth(1) {
        RequestFileTypeExt::try_from(typ)
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
    use moss_fs::adapters::disk::DickFileSystem;

    use super::*;

    #[test]
    fn test() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let r = IndexingService::new(Arc::new(DickFileSystem::new()));
                let r = r
                    .index(&PathBuf::from("./tests/TestCollection"))
                    .await
                    .unwrap();

                dbg!(r);
            });
    }
}
