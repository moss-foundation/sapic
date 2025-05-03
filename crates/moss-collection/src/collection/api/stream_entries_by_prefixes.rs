use futures::pin_mut;
use moss_common::api::OperationResult;
use std::{time::Duration, vec};
use tauri::ipc::Channel;
use tokio_stream::StreamExt;
use tokio_stream::StreamMap;

use crate::{
    collection::Collection,
    models::operations::{EntryInfo, ListEntriesEvent, ListUnitsInput},
};

const POLL_INTERVAL: Duration = Duration::from_millis(100);

impl Collection {
    pub async fn stream_entries_by_prefixes(
        &self,
        on_event: Channel<ListEntriesEvent>,
        input: ListUnitsInput,
    ) -> OperationResult<()> {
        let worktree = self.worktree().await?;
        let snapshot = worktree.snapshot().await;

        if snapshot.count_files() == 0 {
            // We need to send a final empty event to signal the end of the stream.
            let _ = on_event.send(ListEntriesEvent(vec![]));
            return Ok(());
        }

        let mut streams = StreamMap::new();
        for prefix in input.0 {
            let s = tokio_stream::iter(snapshot.iter_entries_by_prefix(&prefix).map(
                |(&id, entry)| EntryInfo {
                    id,
                    path: entry.path.to_path_buf(),
                },
            ));
            streams.insert(prefix, s);
        }

        let stream = streams.map(|(_key, value)| value);
        let batched = stream.chunks_timeout(snapshot.count_files(), POLL_INTERVAL);
        pin_mut!(batched);

        while let Some(batch) = batched.next().await {
            let _ = on_event.send(ListEntriesEvent(batch));
        }

        // We need to send a final empty event to signal the end of the stream.
        let _ = on_event.send(ListEntriesEvent(vec![]));

        Ok(())
    }
}
