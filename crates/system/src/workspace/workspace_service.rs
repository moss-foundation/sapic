use crate::workspace::{
    CreatedWorkspace, WorkspaceCreateOp, WorkspaceServiceFs, types::WorkspaceItem,
};
use async_trait::async_trait;
use joinerror::ResultExt;
use moss_storage2::{KvStorage, models::primitives::StorageScope};
use rustc_hash::FxHashMap;
use sapic_base::workspace::types::primitives::WorkspaceId;
use sapic_core::context::AnyAsyncContext;
use serde_json::Value as JsonValue;
use std::{path::PathBuf, sync::Arc};

static KEY_WORKSPACE_PREFIX: &'static str = "workspace";

pub fn key_workspace_last_opened_at(id: &WorkspaceId) -> String {
    format!("{KEY_WORKSPACE_PREFIX}.{}.lastOpenedAt", id.to_string())
}

pub fn key_workspace(id: &WorkspaceId) -> String {
    format!("{KEY_WORKSPACE_PREFIX}.{}", id.to_string())
}

pub struct WorkspaceService {
    fs: Arc<dyn WorkspaceServiceFs>,
    storage: Arc<dyn KvStorage>,
}

impl WorkspaceService {
    pub fn new(fs: Arc<dyn WorkspaceServiceFs>, storage: Arc<dyn KvStorage>) -> Self {
        Self { fs, storage }
    }

    pub async fn delete_workspace(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &WorkspaceId,
    ) -> joinerror::Result<Option<PathBuf>> {
        // TODO: schedule deletion of the workspace directory on a background if we fail to delete it
        // Remove storage entry first since files might not have been properly deleted yet
        if let Err(e) = self
            .storage
            .remove_batch_by_prefix(ctx, StorageScope::Application, &key_workspace(id))
            .await
        {
            tracing::warn!(
                "failed to remove database entries for workspace `{}`: {}",
                id,
                e.to_string()
            );
        }

        let deleted_path = self.fs.delete_workspace(id).await?;

        Ok(deleted_path)
    }

    pub async fn workspaces(
        &self,
        ctx: &dyn AnyAsyncContext,
    ) -> joinerror::Result<Vec<WorkspaceItem>> {
        let restored_items: FxHashMap<String, JsonValue> = if let Ok(items) = self
            .storage
            .get_batch_by_prefix(ctx, StorageScope::Application, KEY_WORKSPACE_PREFIX)
            .await
        {
            items.into_iter().collect()
        } else {
            FxHashMap::default()
        };

        let discovered_workspaces = self
            .fs
            .lookup_workspaces()
            .await
            .join_err::<()>("failed to lookup workspaces")?;

        let workspaces = discovered_workspaces
            .into_iter()
            .map(|discovered| {
                let filtered_items = restored_items
                    .iter()
                    .filter(|(key, _)| key.starts_with(&key_workspace(&discovered.id)))
                    .collect::<FxHashMap<_, _>>();

                let last_opened_at = filtered_items
                    .get(&key_workspace_last_opened_at(&discovered.id))
                    .and_then(|value| value.as_i64());

                WorkspaceItem {
                    id: discovered.id,
                    name: discovered.name,
                    abs_path: Arc::from(discovered.abs_path),
                    last_opened_at,
                }
            })
            .collect();

        Ok(workspaces)
    }
}

#[async_trait]
impl WorkspaceCreateOp for WorkspaceService {
    async fn create(&self, name: String) -> joinerror::Result<CreatedWorkspace> {
        let id = WorkspaceId::new();
        let abs_path = self
            .fs
            .create_workspace(&id, &name, self.storage.clone())
            .await?;

        Ok(CreatedWorkspace { id, name, abs_path })
    }
}
