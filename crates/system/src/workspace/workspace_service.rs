use moss_fs::FileSystem;
use moss_storage2::{Storage, models::primitives::StorageScope};
use moss_workspace::{models::primitives::WorkspaceId, workspace::WorkspaceSummary};
use rustc_hash::FxHashMap;

use serde_json::Value as JsonValue;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

// static KEY_LAST_ACTIVE_WORKSPACE: &'static str = "lastActiveWorkspace";
static KEY_WORKSPACE_PREFIX: &'static str = "workspace";

pub fn key_workspace_last_opened_at(id: &WorkspaceId) -> String {
    format!("{KEY_WORKSPACE_PREFIX}.{}.lastOpenedAt", id.to_string())
}

pub fn key_workspace(id: &WorkspaceId) -> String {
    format!("{KEY_WORKSPACE_PREFIX}.{}", id.to_string())
}

pub struct WorkspaceItem {
    pub id: WorkspaceId,
    pub name: String,
    pub abs_path: Arc<Path>,
    pub last_opened_at: Option<i64>,
}

pub struct WorkspaceService {
    workspaces_dir: PathBuf,
    fs: Arc<dyn FileSystem>,
    storage: Arc<dyn Storage>,
}

impl WorkspaceService {
    pub async fn new(
        fs: Arc<dyn FileSystem>,
        storage: Arc<dyn Storage>,
        workspaces_dir: PathBuf,
    ) -> Self {
        Self {
            fs,
            storage,
            workspaces_dir,
        }
    }

    pub async fn known_workspaces(&self) -> joinerror::Result<Vec<WorkspaceItem>> {
        let restored_items: FxHashMap<String, JsonValue> = if let Ok(items) = self
            .storage
            .get_batch_by_prefix(StorageScope::Application, KEY_WORKSPACE_PREFIX)
            .await
        {
            items.into_iter().collect()
        } else {
            FxHashMap::default()
        };

        let mut read_dir = self.fs.read_dir(&self.workspaces_dir).await?;

        let mut workspaces = vec![];
        while let Some(entry) = read_dir.next_entry().await? {
            if !entry.file_type().await?.is_dir() {
                continue;
            }

            let id_str = entry.file_name().to_string_lossy().to_string();
            let id: WorkspaceId = id_str.into();

            // Log the error and skip when encountering a workspace with invalid manifest
            let summary = match WorkspaceSummary::new(&self.fs, &entry.path()).await {
                Ok(summary) => summary,
                Err(e) => {
                    // FIXME: We can't use session logs outside of a session.
                    // session::error!(format!(
                    //     "failed to parse workspace `{}` manifest: {}",
                    //     id.as_str(),
                    //     e.to_string()
                    // ));

                    println!(
                        "ERROR: failed to parse workspace `{}` manifest: {}",
                        id.as_str(),
                        e.to_string()
                    );
                    continue;
                }
            };

            let filtered_items = restored_items
                .iter()
                .filter(|(key, _)| key.starts_with(&key_workspace(&id)))
                .collect::<FxHashMap<_, _>>();

            let last_opened_at = filtered_items
                .get(&key_workspace_last_opened_at(&id))
                .and_then(|value| value.as_i64());

            workspaces.push(WorkspaceItem {
                id,
                name: summary.name,
                abs_path: entry.path().into(),
                last_opened_at,
            });
        }

        Ok(workspaces)
    }
}
