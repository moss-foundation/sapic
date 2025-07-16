use moss_applib::AppRuntime;
use moss_common::api::OperationResult;
use tauri::ipc::Channel as TauriChannel;

use crate::{
    collection::Collection,
    models::{
        events::BatchUpdateEntryEvent,
        operations::{BatchUpdateEntryInput, BatchUpdateEntryKind, BatchUpdateEntryOutput},
    },
};

impl<R: AppRuntime> Collection<R> {
    pub async fn batch_update_entry(
        &self,
        ctx: &R::AsyncContext,
        input: BatchUpdateEntryInput,
        channel: TauriChannel<BatchUpdateEntryEvent>,
    ) -> OperationResult<BatchUpdateEntryOutput> {
        for entry in input.entries {
            match entry {
                BatchUpdateEntryKind::Item(input) => {
                    let output = self.update_item_entry(ctx, input).await?;
                    channel.send(BatchUpdateEntryEvent::Item(output))?;
                }
                BatchUpdateEntryKind::Dir(input) => {
                    let output = self.update_dir_entry(ctx, input).await?;
                    channel.send(BatchUpdateEntryEvent::Dir(output))?;
                }
            }
        }

        Ok(BatchUpdateEntryOutput {})
    }
}
