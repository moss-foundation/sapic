use futures::pin_mut;
use moss_common::api::OperationResult;
use std::{time::Duration, vec};
use tauri::ipc::Channel;
use tokio_stream::StreamExt;
use tokio_stream::StreamMap;

use crate::{
    collection::Collection,
    models::operations::{EntryInfo, StreamEntriesByPrefixesEvent, StreamEntriesByPrefixesInput},
};

const POLL_INTERVAL: Duration = Duration::from_millis(100);

impl Collection {
    pub async fn stream_entries_by_prefixes(
        &self,
        on_event: Channel<StreamEntriesByPrefixesEvent>,
        input: StreamEntriesByPrefixesInput,
    ) -> OperationResult<()> {
        let state_store = self.collection_storage.state_store().await;
        let worktree_entries_state = state_store.list_worktree_entries()?;

        let worktree = self.worktree().await?;
        let snapshot_lock = worktree.snapshot().await.lock().await;

        if snapshot_lock.count_files() == 0 {
            // We need to send a final empty event to signal the end of the stream.
            let _ = on_event.send(StreamEntriesByPrefixesEvent(vec![]));
            return Ok(());
        }

        let mut streams = StreamMap::new();
        for prefix in input.0 {
            let s = tokio_stream::iter(snapshot_lock.iter_entries_by_prefix(&prefix).map(
                |(&id, entry)| {
                    let restored_entry_state = worktree_entries_state
                        .iter()
                        .find(|e| e.path == entry.path.as_ref());

                    EntryInfo {
                        id,
                        path: entry.path.to_path_buf(),
                        order: restored_entry_state.map(|e| e.order),
                    }
                },
            ));
            streams.insert(prefix, s);
        }

        let stream = streams.map(|(_key, value)| value);
        let batched = stream.chunks_timeout(snapshot_lock.count_files(), POLL_INTERVAL);
        pin_mut!(batched);

        while let Some(batch) = batched.next().await {
            let _ = on_event.send(StreamEntriesByPrefixesEvent(batch));
        }

        // We need to send a final empty event to signal the end of the stream.
        let _ = on_event.send(StreamEntriesByPrefixesEvent(vec![]));

        Ok(())
    }
}
