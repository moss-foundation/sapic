use moss_app_delegate::AppDelegate;
use moss_applib::{
    AppRuntime,
    context::{AnyAsyncContext, Reason},
};
use moss_logging::session;
use moss_storage2::{Storage, models::primitives::StorageScope};
use serde_json::Value as JsonValue;
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    sync::Arc,
};
use tauri::ipc::Channel as TauriChannel;
use tokio::sync::{mpsc, oneshot};

use crate::{
    Project,
    models::{
        events::StreamResourcesEvent,
        operations::{StreamResourcesInput, StreamResourcesOutput},
        primitives::{FrontendResourcePath, ResourceId},
    },
    project::OnDidChangeEvent,
    storage::{KEY_EXPANDED_ENTRIES, KEY_RESOURCE_PREFIX},
    worktree::entry::EntryDescription,
};

impl<R: AppRuntime> Project<R> {
    pub async fn stream_resources(
        &self,
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        channel: TauriChannel<StreamResourcesEvent>,
        input: StreamResourcesInput,
    ) -> joinerror::Result<StreamResourcesOutput> {
        let (tx, mut rx) = mpsc::unbounded_channel::<EntryDescription>();
        let (done_tx, mut done_rx) = oneshot::channel::<()>();

        let mut handles = Vec::new();
        let expansion_dirs = match input {
            StreamResourcesInput::LoadRoot => vec![PathBuf::from("")],
            StreamResourcesInput::ReloadPath(path) => vec![path],
        };

        let storage = <dyn Storage>::global(app_delegate);
        let storage_scope = StorageScope::Project(self.id.inner());
        let expanded_entries: Arc<HashSet<ResourceId>> = match storage
            .get(storage_scope.clone(), KEY_EXPANDED_ENTRIES)
            .await
        {
            Ok(Some(entries)) => serde_json::from_value::<HashSet<ResourceId>>(entries)
                .unwrap_or_else(|e| {
                    session::warn!(format!("failed to deserialize expanded entries: {}", e));
                    HashSet::new()
                })
                .into(),
            Ok(None) => HashSet::new().into(),
            Err(e) => {
                session::warn!(format!("failed to get expanded entries: {}", e));
                HashSet::new().into()
            }
        };

        let all_entry_keys: Arc<HashMap<String, JsonValue>> = match storage
            .get_batch_by_prefix(storage_scope.clone(), KEY_RESOURCE_PREFIX)
            .await
        {
            Ok(keys) => keys.into_iter().collect::<HashMap<_, _>>().into(),
            Err(e) => {
                session::warn!(format!("failed to get all entry keys: {}", e));
                HashMap::new().into()
            }
        };

        for dir in expansion_dirs {
            let entries_tx_clone = tx.clone();
            let worktree_service_clone = self.worktree().await.to_owned();
            // We need to fetch this data from the database here, otherwise we'll be requesting it every time the scan method is called.

            let handle = tokio::spawn({
                let expanded_entries_clone = expanded_entries.clone();
                let all_entry_keys_clone = all_entry_keys.clone();
                let ctx_clone = ctx.clone();
                let app_handle_clone = app_delegate.clone();

                async move {
                    let _ = worktree_service_clone
                        .scan(
                            &ctx_clone,
                            app_handle_clone,
                            &dir,
                            expanded_entries_clone,
                            all_entry_keys_clone,
                            entries_tx_clone,
                        )
                        .await;
                }
            });

            handles.push(handle);
        }

        drop(tx);

        let ctx_clone = ctx.clone();
        // TODO: check if the context is done, if so, break the loop (cancellation)
        let processing_task = tokio::spawn(async move {
            loop {
                // Handle only timeout error now
                // New error type
                // unimplemented for cancelled now
                tokio::select! {
                    entry_result = rx.recv() => {
                        if let Some(entry) = entry_result {
                            let _ = channel.send(StreamResourcesEvent{
                                id: entry.id,
                                name: entry.name,
                                path: FrontendResourcePath::new(entry.path.to_path_buf()),
                                class: entry.class,
                                kind: entry.kind,
                                protocol: entry.protocol,
                                order: entry.order,
                                expanded: entry.expanded,
                            });
                        }
                    }

                    _ = &mut done_rx => {
                        while let Ok(entry) = rx.try_recv() {
                            let _ = channel.send(StreamResourcesEvent{
                                id: entry.id,
                                name: entry.name,
                                path: FrontendResourcePath {
                                    raw: entry.path.to_path_buf(),
                                    segments: entry.path.to_path_buf().iter().map(|s| s.to_string_lossy().to_string()).collect(),
                                },
                                class: entry.class,
                                kind: entry.kind,
                                protocol: entry.protocol,
                                order: entry.order,
                                expanded: entry.expanded,
                            });
                        }
                        break;
                    }

                    else => {
                        match ctx_clone.done() {
                            Some(Reason::Timeout) => {
                                return Err(joinerror::Error::new::<()>("stream entries time out"));
                            },
                            Some(Reason::Canceled) => {
                                // FIXME: Implement cancellation
                                unimplemented!()
                            }
                            None => {},
                        }
                    }
                }
            }
            Ok(())
        });

        let completion_task = tokio::spawn(async move {
            futures::future::join_all(handles).await;
            let _ = done_tx.send(());
        });

        // FIXME: handle potential error from cancellation/timeout
        let _result = tokio::try_join!(processing_task, completion_task);

        self.on_did_change
            .fire(OnDidChangeEvent::Toggled(true))
            .await;

        Ok(StreamResourcesOutput {})
    }
}
