use crate::{
    collection::Collection,
    models::{
        events::StreamEntriesByPrefixesEvent, operations::StreamEntriesByPrefixesInput,
        types::EntryInfo,
    },
};
use futures::pin_mut;
use moss_common::api::{OperationError, OperationResult};
use moss_fs::utils::encode_path;
use std::path::Path;
use std::{time::Duration, vec};
use tauri::ipc::Channel;
use tokio_stream::StreamExt;
use tokio_stream::StreamMap;

const POLL_INTERVAL: Duration = Duration::from_millis(100);
const MAX_CHUNK_SIZE: usize = 100;

impl Collection {
    pub async fn stream_entries_by_prefixes(
        &self,
        channel: Channel<StreamEntriesByPrefixesEvent>,
        input: StreamEntriesByPrefixesInput,
    ) -> OperationResult<()> {
        // TODO: Integrate collection storage
        // let unit_store = self.collection_storage.unit_store();
        //
        // let worktree_entries_state = unit_store.list_worktree_entries()?;

        let worktree = self.worktree().await?;
        let read_lock = worktree.read().await;
        //
        // if snapshot_lock.count_files() == 0 {
        //     // We need to send a final empty event to signal the end of the stream.
        //     let _ = channel.send(StreamEntriesByPrefixesEvent(vec![]));
        //     return Ok(());
        // }

        let mut streams = StreamMap::new();
        for prefix in input.0 {
            let normalized_prefix = encode_path(Path::new(prefix), None)?;
            let s = tokio_stream::iter(read_lock.iter_entries_by_prefix(normalized_prefix).map(
                |(&id, entry)| {
                    // TODO: Get order from collection storage
                    EntryInfo {
                        id,
                        name: entry
                            .path()
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string(),
                        path: entry.path().to_path_buf(),
                        is_dir: entry.is_dir(),
                        classification: entry.classification(),
                        protocol: entry.protocol(),
                        order: entry.order(),
                    }
                },
            ));
            streams.insert(prefix, s);
        }

        let stream = streams.map(|(_key, value)| value);
        let batched = stream.chunks_timeout(MAX_CHUNK_SIZE, POLL_INTERVAL);
        pin_mut!(batched);

        while let Some(batch) = batched.next().await {
            let _ = channel.send(StreamEntriesByPrefixesEvent(batch));
        }

        // We need to send a final empty event to signal the end of the stream.
        let _ = channel.send(StreamEntriesByPrefixesEvent(vec![]));

        Ok(())
    }
}
