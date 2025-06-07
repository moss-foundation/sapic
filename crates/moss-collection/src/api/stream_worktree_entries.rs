use crate::{
    collection::Collection,
    models::{
        events::StreamWorktreeEntriesEvent, operations::StreamWorktreeEntriesInput,
        types::EntryInfo,
    },
};
use futures::pin_mut;
use moss_common::api::OperationResult;
use moss_fs::utils::normalize_path;
use std::{path::Path, time::Duration, vec};
use tauri::ipc::Channel as TauriChannel;
use tokio_stream::{StreamExt, StreamMap};

const POLL_INTERVAL: Duration = Duration::from_millis(100);
const MAX_CHUNK_SIZE: usize = 100;

impl Collection {
    pub async fn stream_worktree_entries(
        &self,
        channel: TauriChannel<StreamWorktreeEntriesEvent>,
        input: StreamWorktreeEntriesInput,
    ) -> OperationResult<()> {
        // TODO: Integrate collection storage
        // let unit_store = self.collection_storage.unit_store();
        //
        // let worktree_entries_state = unit_store.list_worktree_entries()?;

        // let worktree = self.worktree().await?;
        // if worktree.is_empty() {
        //     // We need to send a final empty event to signal the end of the stream.
        //     let _ = channel.send(StreamWorktreeEntriesEvent(vec![]));
        //     return Ok(());
        // }

        // let mut streams = StreamMap::new();
        // for prefix in input.prefixes {
        //     let normalized_prefix = normalize_path(Path::new(prefix));
        //     let s = tokio_stream::iter(worktree.iter_entries_by_prefix(normalized_prefix).map(
        //         |(&id, entry)| {
        //             // TODO: Get order from collection storage
        //             EntryInfo {
        //                 id,
        //                 name: entry
        //                     .path()
        //                     .file_name()
        //                     .unwrap_or_default()
        //                     .to_string_lossy()
        //                     .to_string(),
        //                 path: entry.path().to_path_buf(),
        //                 is_dir: entry.is_dir(),
        //                 classification: entry.classification(),
        //                 protocol: entry.protocol(),
        //                 order: entry.order(),
        //             }
        //         },
        //     ));
        //     streams.insert(prefix, s);
        // }

        // let stream = streams.map(|(_key, value)| value);
        // let batched = stream.chunks_timeout(MAX_CHUNK_SIZE, POLL_INTERVAL);
        // pin_mut!(batched);

        // while let Some(batch) = batched.next().await {
        //     let _ = channel.send(StreamWorktreeEntriesEvent(batch));
        // }

        // // We need to send a final empty event to signal the end of the stream.
        // let _ = channel.send(StreamWorktreeEntriesEvent(vec![]));

        // Ok(())

        todo!()
    }
}
