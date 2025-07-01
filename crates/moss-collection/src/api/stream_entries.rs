use std::{
    collections::{HashMap, HashSet},
    path::Path,
    sync::Arc,
};

use moss_common::api::OperationResult;
use moss_db::primitives::AnyValue;
use moss_storage::primitives::segkey::SegKeyBuf;
use tauri::ipc::Channel as TauriChannel;
use tokio::sync::{mpsc, oneshot};
use uuid::Uuid;

use crate::{
    Collection,
    collection::OnDidChangeEvent,
    dirs,
    models::{
        events::StreamEntriesEvent, operations::StreamEntriesOutput, primitives::EntryPath,
        types::EntryInfo,
    },
    services::{
        storage_service::StorageService,
        worktree_service::{EntryDescription, WorktreeService},
    },
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
        let storage_service = self.service::<StorageService>();

        let mut handles = Vec::new();
        for dir in EXPANSION_DIRECTORIES {
            let dir_path = Path::new(dir);
            let entries_tx_clone = tx.clone();
            let worktree_service_clone = worktree_service.clone();

            // We need to fetch this data from the database here, otherwise we’ll be requesting it every time the scan method is called.

            let expanded_entries: Arc<HashSet<Uuid>> =
                match storage_service.get_expanded_entries::<Uuid>() {
                    Ok(entries) => entries.collect::<HashSet<_>>().into(),
                    Err(error) => {
                        println!("warn: getting expanded entries: {}", error);
                        HashSet::default().into()
                    }
                };

            let all_entry_keys: Arc<HashMap<SegKeyBuf, AnyValue>> =
                match storage_service.get_all_entry_keys() {
                    Ok(keys) => keys.collect::<HashMap<_, _>>().into(),
                    Err(error) => {
                        println!("warn: getting all entry keys: {}", error);
                        HashMap::default().into()
                    }
                };

            let handle = tokio::spawn(async move {
                let _ = worktree_service_clone
                    .scan(
                        dir_path,
                        expanded_entries.clone(),
                        all_entry_keys.clone(),
                        entries_tx_clone,
                    )
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
                                order: entry.order,
                                expanded: entry.expanded,
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
                                order: entry.order,
                                expanded: entry.expanded,
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
