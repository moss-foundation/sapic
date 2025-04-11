use super::request_store::TABLE_REQUESTS;
use super::{request_store::RequestStoreImpl, RequestStore, StateDbManager};
use anyhow::Result;
use arc_swap::ArcSwap;
use moss_db::ReDbClient;
use std::mem;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Notify;

const COLLECTION_STATE_DB_NAME: &str = "state.db";

struct DbManagerCell {
    request_store: Arc<dyn RequestStore>,
}

impl DbManagerCell {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        dbg!(path.as_ref());
        let db_client = ReDbClient::new(path.as_ref().join(COLLECTION_STATE_DB_NAME))?
            .with_table(&TABLE_REQUESTS)?;

        let request_store = Arc::new(RequestStoreImpl::new(db_client));
        Ok(Self { request_store })
    }
}

pub struct StateDbManagerImpl {
    state: ArcSwap<DatabaseState>,
}

pub enum DatabaseState {
    Loaded(DbManagerCell),
    Reloading { notify: Arc<Notify> },
}

impl StateDbManagerImpl {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let cell = DbManagerCell::new(path)?;

        Ok(Self {
            state: ArcSwap::new(Arc::new(DatabaseState::Loaded(cell))),
        })
    }

    pub async fn request_store_2(&self) -> Arc<dyn RequestStore> {
        loop {
            let current_state = self.state.load();
            match current_state.as_ref() {
                DatabaseState::Loaded(cell) => return cell.request_store.clone(),
                DatabaseState::Reloading { notify } => {
                    notify.notified().await;
                    continue;
                }
            }
        }
    }

    pub async fn reload_2(
        &self,
        new_path: PathBuf,
        after_drop: impl FnOnce() -> Result<()> + Send + 'static,
    ) -> Result<()> {
        let local_notify = Arc::new(Notify::new());
        let reloading_state = Arc::new(DatabaseState::Reloading {
            notify: local_notify.clone(),
        });
        let _old_state = self.state.swap(reloading_state);

        // Wait for current operations to complete
        tokio::task::yield_now().await;
        drop(_old_state);

        after_drop()?;

        let new_cell = DbManagerCell::new(new_path)?;
        let new_state = Arc::new(DatabaseState::Loaded(new_cell));
        self.state.store(new_state);

        // Notify waiting operations
        local_notify.notify_waiters();
        Ok(())
    }

    // async fn reload(
    //     &self,
    //     path: PathBuf,
    //     after_drop: Box<dyn FnOnce() -> Result<()>>,
    // ) -> Result<()> {
    //     let local_notify = Arc::new(Notify::new());
    //     let old_cell = self.state.swap(Arc::new(DbManagerCellPlaceholder {
    //         notify: local_notify.clone(),
    //     }));

    //     tokio::task::yield_now().await;

    //     Ok(())
    // }
}

impl StateDbManager for StateDbManagerImpl {
    fn reload(&self, path: PathBuf, after_drop: Box<dyn FnOnce() -> Result<()>>) -> Result<()> {
        Ok(())
    }

    fn request_store(&self) -> Arc<dyn RequestStore> {
        // self.state.load().request_store.clone()
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_reload() {
        let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("myFolder");
        let db_path = base_path.join("test.db");
        // dbg!(&db_path);

        let state_db_manager = StateDbManagerImpl::new(base_path.clone()).unwrap();
        // let request_store = state_db_manager.request_store();
        // let requests = request_store.scan().unwrap();

        // assert_eq!(requests.len(), 0);

        let new_base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("newFolder");

        state_db_manager
            .reload_2(new_base_path.clone(), || {
                std::fs::rename(base_path, new_base_path)?;
                Ok(())
            })
            .await
            .unwrap();

        // let request_store = state_db_manager.request_store();
        // let requests = request_store.scan().unwrap();
        // assert_eq!(requests.len(), 0);
    }
}
