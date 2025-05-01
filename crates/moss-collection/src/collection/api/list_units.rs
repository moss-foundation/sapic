use moss_common::api::OperationResult;
use std::{path::Path, sync::Arc, time::Duration, vec};
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
        let limit = worktree.snapshot().await.count_files();
        let (tx, mut rx) = mpsc::channel(limit);

        for entry_prefix in input.0 {
            let tx_clone = tx.clone();
            let worktree_clone = Arc::clone(&worktree);

            tokio::task::spawn(async move {
                let snapshot = worktree_clone.snapshot().await;
                let prefix = Path::new(entry_prefix);

                dbg!(snapshot.entries_by_prefix(prefix).collect::<Vec<_>>());

                for (&id, entry) in snapshot.entries_by_prefix(prefix) {
                    let _ = tx_clone.send(EntryInfo {
                        id,
                        path: entry.path.to_path_buf(),
                    }); // FIXME: await
                }
            });
        }
        drop(tx);

        let mut interval = tokio::time::interval(POLL_INTERVAL);
        tokio::task::spawn(async move {
            loop {
                interval.tick().await;

                dbg!("tick");
                dbg!(limit);

                let mut batch = Vec::with_capacity(limit);
                let received = rx.recv_many(&mut batch, limit).await;
                if received > 0 {
                    dbg!("send");
                    dbg!(&batch.len());

                    let _ = on_event.send(ListEntriesEvent(batch));
                } else {
                    dbg!("send empty");
                    let _ = on_event.send(ListEntriesEvent(vec![]));
                    break;
                }
            }
        });

        Ok(())
    }
}
