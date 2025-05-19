use arc_swap::ArcSwap;
use async_trait::async_trait;
use moss_db::{ClientState, DatabaseResult, Transaction};
use serde_json::Value as JsonValue;
use std::any::TypeId;
use std::collections::HashMap;
use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Notify;

use crate::new_storage::{
    Dump, NewReset, NewResettableStorage, NewStorage, new_storage::NewStorageHandle,
};
use crate::storage::{Transactional, table::Table};

pub struct NewResettableStorageHandle {
    state: ArcSwap<ClientState<NewStorageHandle>>,
    tables: Vec<Arc<dyn Table>>,
}

impl NewResettableStorageHandle {
    pub fn new(path: impl AsRef<Path>, tables: Vec<Arc<dyn Table>>) -> DatabaseResult<Self> {
        let storage = NewStorageHandle::new(path, tables.clone())?;

        Ok(Self {
            state: ArcSwap::new(Arc::new(ClientState::Loaded(storage))),
            tables,
        })
    }
}

#[async_trait]
impl Transactional for NewResettableStorageHandle {
    async fn begin_write(&self) -> DatabaseResult<Transaction> {
        loop {
            match self.state.load().as_ref() {
                ClientState::Loaded(storage) => return storage.begin_write().await,
                ClientState::Reloading { notify } => notify.notified().await,
            }
        }
    }

    async fn begin_read(&self) -> DatabaseResult<Transaction> {
        loop {
            match self.state.load().as_ref() {
                ClientState::Loaded(storage) => return storage.begin_read().await,
                ClientState::Reloading { notify } => notify.notified().await,
            }
        }
    }
}

#[async_trait]
impl Dump for NewResettableStorageHandle {
    async fn dump(&self) -> DatabaseResult<HashMap<String, JsonValue>> {
        loop {
            match self.state.load().as_ref() {
                ClientState::Loaded(storage) => return storage.dump().await,
                ClientState::Reloading { notify } => notify.notified().await,
            }
        }
    }
}

#[async_trait]
impl NewStorage for NewResettableStorageHandle {
    async fn table(&self, id: &TypeId) -> DatabaseResult<Arc<dyn Table>> {
        loop {
            match self.state.load().as_ref() {
                ClientState::Loaded(storage) => return storage.table(id).await,
                ClientState::Reloading { notify } => notify.notified().await,
            }
        }
    }
}

#[async_trait]
impl NewReset for NewResettableStorageHandle {
    async fn reset(
        &self,
        path: &Path,
        after_drop: Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>>,
    ) -> anyhow::Result<()> {
        let local_notify = Arc::new(Notify::new());
        let reloading_state = Arc::new(ClientState::Reloading {
            notify: local_notify.clone(),
        });
        let old_state = self.state.swap(reloading_state);

        // Wait for current operations to complete
        tokio::task::yield_now().await;
        drop(old_state);

        after_drop.await?;

        let new_storage = NewStorageHandle::new(path, self.tables.clone())?;
        let new_state = Arc::new(ClientState::Loaded(new_storage));
        self.state.store(new_state);

        // Notify waiting operations
        local_notify.notify_waiters();
        Ok(())
    }
}

impl NewResettableStorage for NewResettableStorageHandle {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::segkey::{SegKey, SegKeyBuf};
    use moss_db::bincode_table::BincodeTable;
    use moss_db::primitives::AnyValue;
    use moss_testutils::random_name::random_string;
    use std::time::Duration;

    struct Table1 {
        table: BincodeTable<'static, SegKeyBuf, AnyValue>,
    }
    impl Table1 {
        pub const fn new() -> Self {
            Self {
                table: BincodeTable::new("table1"),
            }
        }
    }
    impl Table for Table1 {
        fn definition(&self) -> &BincodeTable<SegKeyBuf, AnyValue> {
            &self.table
        }
    }

    const TABLE1: Table1 = Table1::new();

    struct TestResettableStore {
        handle: Arc<dyn NewResettableStorage>,
    }

    impl TestResettableStore {
        pub fn new(path: &Path) -> Self {
            Self {
                handle: Arc::new(
                    NewResettableStorageHandle::new(path, vec![Arc::new(TABLE1)]).unwrap(),
                ),
            }
        }
        pub async fn table1(&self) -> DatabaseResult<Arc<dyn Table>> {
            self.handle.table(&TypeId::of::<Table1>()).await
        }
    }

    #[async_trait]
    impl Transactional for TestResettableStore {
        async fn begin_write(&self) -> DatabaseResult<Transaction> {
            self.handle.begin_write().await
        }

        async fn begin_read(&self) -> DatabaseResult<Transaction> {
            self.handle.begin_read().await
        }
    }

    #[async_trait]
    impl Dump for TestResettableStore {
        async fn dump(&self) -> DatabaseResult<HashMap<String, JsonValue>> {
            self.handle.dump().await
        }
    }

    #[async_trait]
    impl NewReset for TestResettableStore {
        async fn reset(
            &self,
            path: &Path,
            after_drop: Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>>,
        ) -> anyhow::Result<()> {
            self.handle.reset(path, after_drop).await
        }
    }

    #[tokio::test]
    async fn test_resettable_storage() {
        let filename = format!("{}.db", random_string(10));
        let new_filename = format!("{}.renamed", filename);

        let path = Path::new("tests").join(&filename);
        let new_path = Path::new("tests").join(&new_filename);

        let store = Arc::new(TestResettableStore::new(&path));
        let table1 = store.table1().await.unwrap();

        let mut write_txn = store.begin_write().await.unwrap();

        table1
            .definition()
            .insert(
                &mut write_txn,
                SegKey::new("table1").join("key1"),
                &AnyValue::new(serde_json::to_vec("value1").unwrap()),
            )
            .unwrap();

        write_txn.commit().unwrap();

        let path_clone = path.clone();
        let new_path_clone = new_path.clone();
        let after_drop = Box::pin(async {
            tokio::fs::rename(path_clone, new_path_clone).await.unwrap();
            tokio::time::sleep(Duration::from_secs(2)).await;
            println!("Finished Resetting");
            Ok(())
        });

        let new_path_clone = new_path.clone();
        let store_clone = store.clone();
        tokio::task::spawn(async move {
            store_clone
                .reset(&new_path_clone, after_drop)
                .await
                .unwrap();
        });

        tokio::time::sleep(Duration::from_secs(1)).await;

        dbg!(store.dump().await.unwrap());
    }
}
