use moss_applib::AppRuntime;
use tauri::ipc::Channel as TauriChannel;

use crate::{
    collection::Collection,
    errors::ErrorInternal,
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
    ) -> joinerror::Result<BatchUpdateEntryOutput> {
        for entry in input.entries {
            match entry {
                BatchUpdateEntryKind::Item(input) => {
                    let output = self.update_item_entry(ctx, input).await?;
                    channel
                        .send(BatchUpdateEntryEvent::Item(output))
                        .map_err(|e| {
                            joinerror::Error::new::<ErrorInternal>(format!(
                                "failed to send to the tauri channel: {}",
                                e.to_string()
                            ))
                        })?;
                }
                BatchUpdateEntryKind::Dir(input) => {
                    let output = self.update_dir_entry(ctx, input).await?;
                    channel
                        .send(BatchUpdateEntryEvent::Dir(output))
                        .map_err(|e| {
                            joinerror::Error::new::<ErrorInternal>(format!(
                                "failed to send to the tauri channel: {}",
                                e.to_string()
                            ))
                        })?;
                }
            }
        }

        Ok(BatchUpdateEntryOutput {})
    }
}
