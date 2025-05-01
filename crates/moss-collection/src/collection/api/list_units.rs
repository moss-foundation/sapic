use moss_common::api::OperationResult;
use std::{sync::Arc, time::Duration, vec};
use tauri::ipc::Channel;
use tokio::sync::mpsc;

use crate::{
    collection::Collection,
    models::operations::{EntryInfo, ListEntriesEvent, ListUnitsInput},
};

const POLL_INTERVAL: Duration = Duration::from_millis(100);

impl Collection {
    pub async fn list_entries(
        &self,
        on_event: Channel<ListEntriesEvent>,
        input: ListUnitsInput,
    ) -> OperationResult<()> {
        let worktree = self.worktree().await?;
        let files_count = worktree.snapshot().await.count_files();
        let (tx, mut rx) = mpsc::channel(files_count);

        // We don't want to spawn any tasks if there are no files in the worktree.
        if files_count == 0 {
            let _ = on_event.send(ListEntriesEvent(vec![]));
            return Ok(());
        }

        for entry_prefix in input.0 {
            tokio::task::spawn({
                let tx_clone = tx.clone();
                let worktree_clone = Arc::clone(&worktree);

                async move {
                    let entries = tokio::task::spawn_blocking(move || {
                        let snapshot = futures::executor::block_on(worktree_clone.snapshot());
                        snapshot.entries_by_prefix(entry_prefix)
                    })
                    .await
                    .expect("Failed to spawn blocking task for listing entries");

                    for (id, entry) in entries {
                        let _ = tx_clone
                            .send(EntryInfo {
                                id,
                                path: entry.path.to_path_buf(),
                            })
                            .await;
                    }
                }
            });
        }
        drop(tx);

        let mut interval = tokio::time::interval(POLL_INTERVAL);
        tokio::task::spawn(async move {
            loop {
                interval.tick().await;

                let mut batch = Vec::with_capacity(files_count);
                let received = rx.recv_many(&mut batch, files_count).await;
                if received > 0 {
                    let _ = on_event.send(ListEntriesEvent(batch));
                } else {
                    // We send empty event if there are no new entries, so the UI can stop polling.
                    let _ = on_event.send(ListEntriesEvent(vec![]));
                    break;
                }
            }
        });

        Ok(())
    }
}
