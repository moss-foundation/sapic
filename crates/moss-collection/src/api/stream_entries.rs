use moss_applib::{
    AppRuntime,
    context::{AnyAsyncContext, Reason},
};
use moss_db::primitives::AnyValue;
use moss_storage::primitives::segkey::SegKeyBuf;
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    sync::Arc,
};
use tauri::ipc::Channel as TauriChannel;
use tokio::sync::{mpsc, oneshot};

use crate::{
    Collection,
    collection::OnDidChangeEvent,
    dirs,
    models::{
        events::StreamEntriesEvent,
        operations::{StreamEntriesInput, StreamEntriesOutput},
        primitives::{EntryId, FrontendEntryPath},
    },
    worktree::{Worktree, entry::EntryDescription},
};

const EXPANSION_DIRECTORIES: &[&str] = &[
    dirs::REQUESTS_DIR,
    dirs::ENDPOINTS_DIR,
    dirs::COMPONENTS_DIR,
    dirs::SCHEMAS_DIR,
];

impl<R: AppRuntime> Collection<R> {
    pub async fn stream_entries(
        &self,
        ctx: &R::AsyncContext,
        channel: TauriChannel<StreamEntriesEvent>,
        input: StreamEntriesInput,
    ) -> joinerror::Result<StreamEntriesOutput> {
        let (tx, mut rx) = mpsc::unbounded_channel::<EntryDescription>();
        let (done_tx, mut done_rx) = oneshot::channel::<()>();

        let mut handles = Vec::new();
        let expansion_dirs = match input {
            StreamEntriesInput::LoadRoot => EXPANSION_DIRECTORIES
                .iter()
                .map(|dir| PathBuf::from(dir))
                .collect::<Vec<_>>(),
            StreamEntriesInput::ReloadPath(path) => vec![path],
        };

        let expanded_entries: Arc<HashSet<EntryId>> =
            match self.storage_service.get_expanded_entries(ctx).await {
                Ok(entries) => HashSet::from_iter(entries).into(),
                Err(error) => {
                    println!("warn: getting expanded entries: {}", error);
                    HashSet::default().into()
                }
            };

        let all_entry_keys: Arc<HashMap<SegKeyBuf, AnyValue>> =
            match self.storage_service.get_all_entry_keys(ctx).await {
                Ok(keys) => keys.into(),
                Err(error) => {
                    println!("warn: getting all entry keys: {}", error);
                    HashMap::default().into()
                }
            };

        for dir in expansion_dirs {
            let entries_tx_clone = tx.clone();
            let worktree_service_clone = self
                .worktree
                .get_or_init(|| async {
                    Arc::new(Worktree::new(
                        self.abs_path.clone(),
                        self.fs.clone(),
                        self.broadcaster.clone(),
                        self.storage_service.clone(),
                    ))
                })
                .await
                .to_owned();
            // We need to fetch this data from the database here, otherwise we'll be requesting it every time the scan method is called.

            let handle = tokio::spawn({
                let expanded_entries_clone = expanded_entries.clone();
                let all_entry_keys_clone = all_entry_keys.clone();
                let ctx_clone = ctx.clone();

                async move {
                    let _ = worktree_service_clone
                        .scan(
                            &ctx_clone,
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
                            let _ = channel.send(StreamEntriesEvent{
                                id: entry.id,
                                name: entry.name,
                                path: FrontendEntryPath::new(entry.path.to_path_buf()),
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
                            let _ = channel.send(StreamEntriesEvent{
                                id: entry.id,
                                name: entry.name,
                                path: FrontendEntryPath {
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

        Ok(StreamEntriesOutput {})
    }
}
