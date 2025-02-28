use anyhow::Result;
use moss_fs::ports::{CreateOptions, FileSystem};
use parking_lot::RwLock;
use patricia_tree::PatriciaMap;
use std::{path::PathBuf, sync::Arc};

use crate::{
    kdl::foundations::http::{Request, Url},
    models::operations::collection_operations::CreateRequestInput,
    request_handle::RequestHandle,
    storage::CollectionRequestSubstore,
};

pub(crate) struct CollectionState {
    pub name: String,
    pub order: Option<usize>,
    pub requests: RwLock<PatriciaMap<Arc<RequestHandle>>>,
}

impl CollectionState {
    pub fn new(name: String, order: Option<usize>) -> Self {
        Self {
            name,
            order,
            requests: RwLock::new(PatriciaMap::new()),
        }
    }

    pub fn get_request_handle_or_init(
        &self,
        key: &[u8],
        f: impl FnOnce() -> RequestHandle,
    ) -> Arc<RequestHandle> {
        {
            let read_guard = self.requests.read();
            if let Some(entry) = read_guard.get(key) {
                return Arc::clone(&entry);
            }
        }

        let mut write_guard = self.requests.write();
        if let Some(entry) = write_guard.get(key) {
            return Arc::clone(&entry);
        }

        let entry = Arc::new(f());
        write_guard.insert(key, Arc::clone(&entry));
        entry
    }
}

pub struct CollectionHandle {
    fs: Arc<dyn FileSystem>,
    store: Arc<dyn CollectionRequestSubstore>,
    state: Arc<CollectionState>,
}

impl CollectionHandle {
    pub fn new(
        fs: Arc<dyn FileSystem>,
        store: Arc<dyn CollectionRequestSubstore>,
        name: String,
        order: Option<usize>,
    ) -> Self {
        Self {
            fs,
            store,
            state: Arc::new(CollectionState::new(name, order)),
        }
    }

    pub(crate) fn state(&self) -> Arc<CollectionState> {
        Arc::clone(&self.state)
    }

    pub async fn create_request(
        &self,
        collection_path: &PathBuf,
        relative_path: Option<PathBuf>,
        input: CreateRequestInput,
    ) -> Result<()> {
        let requests_dir = collection_path.join("requests");
        let path = if let Some(path) = relative_path {
            requests_dir.join(path)
        } else {
            requests_dir
        };

        let request_dir = path.join(format!("{}.request", input.name));
        self.fs.create_dir(&request_dir).await?;

        let request_file_content = Request {
            url: Some(input.url.map(|raw| Url::new(raw)).unwrap_or(Url::default())),
            query_params: Default::default(),
            path_params: Default::default(),
            headers: Default::default(),
        };

        self.fs
            .create_file(
                &request_dir.join(format!("{}.{}.sapic", input.name, "get")),
                CreateOptions::default(),
            )
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use moss_fs::adapters::disk::DiskFileSystem;

    use crate::storage::MockCollectionRequestSubstore;

    use super::*;

    const TEST_COLLECTION_PATH: &'static str =
        "/Users/g10z3r/Project/keenawa-co/sapic/crates/moss-collection/tests/TestCollection";

    // #[test]
    // fn create_request() {
    //     let fs = Arc::new(DiskFileSystem::new());
    //     let collection_request_substore = MockCollectionRequestSubstore::new();

    //     let handle = CollectionHandle::new(
    //         fs,
    //         Arc::new(collection_request_substore),
    //         "TestCollection".to_string(),
    //         None,
    //     );

    //     tokio::runtime::Builder::new_multi_thread()
    //         .enable_all()
    //         .build()
    //         .unwrap()
    //         .block_on(async {
    //             handle
    //                 .create_request(
    //                     &PathBuf::from(TEST_COLLECTION_PATH),
    //                     None,
    //                     CreateRequestInput {
    //                         name: "Test42".to_string(),
    //                         query_params: Default::default(),
    //                     },
    //                 )
    //                 .await
    //                 .unwrap();
    //         });
    // }
}
