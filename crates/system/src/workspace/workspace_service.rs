use joinerror::ResultExt;
use moss_storage2::{Storage, models::primitives::StorageScope};
use rustc_hash::FxHashMap;
use sapic_base::workspace::types::primitives::WorkspaceId;

use serde_json::Value as JsonValue;
use std::sync::Arc;

use crate::workspace::{DynWorkspaceDiscoverer, types::KnownWorkspace};

// static KEY_LAST_ACTIVE_WORKSPACE: &'static str = "lastActiveWorkspace";
static KEY_WORKSPACE_PREFIX: &'static str = "workspace";

pub fn key_workspace_last_opened_at(id: &WorkspaceId) -> String {
    format!("{KEY_WORKSPACE_PREFIX}.{}.lastOpenedAt", id.to_string())
}

pub fn key_workspace(id: &WorkspaceId) -> String {
    format!("{KEY_WORKSPACE_PREFIX}.{}", id.to_string())
}

pub struct WorkspaceService {
    discoverer: DynWorkspaceDiscoverer,
    storage: Arc<dyn Storage>,
}

impl WorkspaceService {
    pub async fn new(discoverer: DynWorkspaceDiscoverer, storage: Arc<dyn Storage>) -> Self {
        Self {
            discoverer,
            storage,
        }
    }

    pub async fn known_workspaces(&self) -> joinerror::Result<Vec<KnownWorkspace>> {
        let restored_items: FxHashMap<String, JsonValue> = if let Ok(items) = self
            .storage
            .get_batch_by_prefix(StorageScope::Application, KEY_WORKSPACE_PREFIX)
            .await
        {
            items.into_iter().collect()
        } else {
            FxHashMap::default()
        };

        let discovered_workspaces = self
            .discoverer
            .discover_workspaces()
            .await
            .join_err::<()>("failed to discover workspaces")?;

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

                KnownWorkspace {
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
