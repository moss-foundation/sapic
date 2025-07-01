use moss_common::api::OperationResult;
use tauri::ipc::Channel as TauriChannel;

use crate::{
    collection::Collection,
    models::{
        events::BatchUpdateEntryEvent,
        operations::{BatchUpdateEntryInput, BatchUpdateEntryKind, BatchUpdateEntryOutput},
    },
};

impl Collection {
    pub async fn batch_update_entry(
        &mut self,
        input: BatchUpdateEntryInput,
        channel: TauriChannel<BatchUpdateEntryEvent>,
    ) -> OperationResult<BatchUpdateEntryOutput> {
        for entry in input.entries {
            match entry {
                BatchUpdateEntryKind::Item(input) => {
                    let output = self.update_item_entry(input).await?;
                    channel.send(BatchUpdateEntryEvent::Item(output))?;
                }
                BatchUpdateEntryKind::Dir(input) => {
                    let output = self.update_dir_entry(input).await?;
                    channel.send(BatchUpdateEntryEvent::Dir(output))?;
                }
            }
        }

        Ok(BatchUpdateEntryOutput {})
    }
}
