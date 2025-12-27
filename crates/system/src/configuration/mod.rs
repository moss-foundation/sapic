pub mod configuration_registry;

use async_trait::async_trait;
use sapic_core::context::AnyAsyncContext;
use serde_json::{Map, Value as JsonValue};

#[async_trait]
pub trait SettingsStore: Send + Sync + 'static {
    async fn values(&self) -> Map<String, JsonValue>;
    async fn get_value(&self, key: &str) -> Option<JsonValue>;
    async fn update_value(
        &self,
        ctx: &dyn AnyAsyncContext,
        key: &str,
        value: JsonValue,
    ) -> joinerror::Result<()>;
    async fn remove_value(&self, key: &str) -> joinerror::Result<Option<JsonValue>>;
}
