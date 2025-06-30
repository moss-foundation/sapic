use std::path::Path;

use moss_common::api::OperationResult;
use tauri::ipc::Channel as TauriChannel;
use tokio::sync::{mpsc, oneshot};

use crate::{
    Collection, // worktree::EntryDescription,
    collection::OnDidChangeEvent,
    dirs,
    models::{
        events::StreamEntriesEvent, operations::StreamEntriesOutput, primitives::EntryPath,
        types::EntryInfo,
    },
    services::worktree_service::{EntryDescription, WorktreeService},
};

const EXPANSION_DIRECTORIES: &[&str] = &[
    dirs::REQUESTS_DIR,
    dirs::ENDPOINTS_DIR,
    dirs::COMPONENTS_DIR,
    dirs::SCHEMAS_DIR,
];

impl Collection {
    pub async fn stream_entries(
        &self,
        channel: TauriChannel<StreamEntriesEvent>,
    ) -> OperationResult<StreamEntriesOutput> {
        let (tx, mut rx) = mpsc::unbounded_channel::<EntryDescription>();
        let (done_tx, mut done_rx) = oneshot::channel::<()>();
        let worktree_service = self.service_arc::<WorktreeService>();

        let mut handles = Vec::new();
        for dir in EXPANSION_DIRECTORIES {
            let dir_path = Path::new(dir);
            let entries_tx_clone = tx.clone();
            let worktree_service_clone = worktree_service.clone();

            let handle = tokio::spawn(async move {
                let _ = worktree_service_clone
                    .scan(dir_path, entries_tx_clone)
                    .await;
            });

            handles.push(handle);
        }

        drop(tx);

        let processing_task = tokio::spawn(async move {
            loop {
                tokio::select! {
                    entry_result = rx.recv() => {
                        if let Some(entry) = entry_result {
                            let entry_info = EntryInfo {
                                id: entry.id,
                                name: entry.name,
                                path: EntryPath::new(entry.path.to_path_buf()),
                                class: entry.class,
                                kind: entry.kind,
                                protocol: entry.protocol,
                                order: None, // FIXME: hardcoded
                                expanded: false,  // FIXME: hardcoded
                            };

                            let _ = channel.send(StreamEntriesEvent(entry_info));
                        }
                    }

                    _ = &mut done_rx => {
                        while let Ok(entry) = rx.try_recv() {
                            let entry_info = EntryInfo {
                                id: entry.id,
                                name: entry.name,
                                path: EntryPath {
                                    raw: entry.path.to_path_buf(),
                                    segments: entry.path.to_path_buf().iter().map(|s| s.to_string_lossy().to_string()).collect(),
                                },
                                class: entry.class,
                                kind: entry.kind,
                                protocol: entry.protocol,
                                order: None,  // FIXME: hardcoded
                                expanded: false,  // FIXME: hardcoded
                            };

                            let _ = channel.send(StreamEntriesEvent(entry_info));
                        }
                        break;
                    }
                }
            }
        });

        let completion_task = tokio::spawn(async move {
            futures::future::join_all(handles).await;
            let _ = done_tx.send(());
        });

        let _ = tokio::try_join!(processing_task, completion_task);

        self.on_did_change
            .fire(OnDidChangeEvent::Toggled(true))
            .await;

        Ok(StreamEntriesOutput {})
    }
}
