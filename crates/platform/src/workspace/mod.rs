pub mod workspace_edit_backend;
pub mod workspace_service_fs;

pub(super) const MANIFEST_FILE_NAME: &str = "Sapic.json";

// FIXME: We now require a mock storage for workspace fs/edit tests, since they need to store the environment
// We should try to strip the storage logic from them
#[cfg(test)]
pub(crate) mod tests {
    use async_trait::async_trait;
    use moss_storage2::{
        FlushMode, KvStorage, KvStorageCapabilities, SubstoreManager,
        models::primitives::StorageScope,
    };
    use serde_json::Value as JsonValue;
    use std::{sync::Arc, time::Instant};

    pub(crate) struct MockStorage {}

    impl MockStorage {
        pub(crate) fn new() -> Arc<Self> {
            Arc::new(Self {})
        }
    }

    pub(crate) struct MockCapabilities {}

    impl MockCapabilities {
        fn new() -> Arc<Self> {
            Arc::new(Self {})
        }
    }

    #[async_trait]
    impl KvStorageCapabilities for MockCapabilities {
        async fn last_checkpoint(&self) -> Option<Instant> {
            None
        }

        async fn flush(&self, _mode: FlushMode) -> joinerror::Result<()> {
            Ok(())
        }

        async fn optimize(&self) -> joinerror::Result<()> {
            Ok(())
        }
    }

    #[async_trait]
    impl SubstoreManager for MockStorage {
        async fn add_workspace(&self, _workspace_id: Arc<String>) -> joinerror::Result<()> {
            Ok(())
        }

        async fn remove_workspace(&self, _workspace_id: Arc<String>) -> joinerror::Result<()> {
            Ok(())
        }

        async fn add_project(
            &self,
            _workspace_id: Arc<String>,
            _project_id: Arc<String>,
        ) -> joinerror::Result<()> {
            Ok(())
        }

        async fn remove_project(
            &self,
            _workspace_id: Arc<String>,
            _project_id: Arc<String>,
        ) -> joinerror::Result<()> {
            Ok(())
        }
    }

    #[async_trait]
    impl KvStorage for MockStorage {
        async fn put(
            &self,
            _scope: StorageScope,
            _key: &str,
            _value: JsonValue,
        ) -> joinerror::Result<()> {
            Ok(())
        }

        async fn get(
            &self,
            _scope: StorageScope,
            _key: &str,
        ) -> joinerror::Result<Option<JsonValue>> {
            Ok(None)
        }

        async fn remove(
            &self,
            _scope: StorageScope,
            _key: &str,
        ) -> joinerror::Result<Option<JsonValue>> {
            Ok(None)
        }

        async fn put_batch(
            &self,
            _scope: StorageScope,
            _items: &[(&str, JsonValue)],
        ) -> joinerror::Result<()> {
            Ok(())
        }

        async fn get_batch(
            &self,
            _scope: StorageScope,
            _keys: &[&str],
        ) -> joinerror::Result<Vec<(String, Option<JsonValue>)>> {
            Ok(vec![])
        }

        async fn remove_batch(
            &self,
            _scope: StorageScope,
            _keys: &[&str],
        ) -> joinerror::Result<Vec<(String, Option<JsonValue>)>> {
            Ok(vec![])
        }

        async fn get_batch_by_prefix(
            &self,
            _scope: StorageScope,
            _prefix: &str,
        ) -> joinerror::Result<Vec<(String, JsonValue)>> {
            Ok(vec![])
        }

        async fn remove_batch_by_prefix(
            &self,
            _scope: StorageScope,
            _prefix: &str,
        ) -> joinerror::Result<Vec<(String, JsonValue)>> {
            Ok(vec![])
        }

        async fn capabilities(self: Arc<Self>) -> Arc<dyn KvStorageCapabilities> {
            MockCapabilities::new()
        }
    }
}
