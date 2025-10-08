use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use tauri::ipc::Channel as TauriChannel;

use crate::{
    errors::ErrorInternal,
    models::{
        events::BatchUpdateEntryEvent,
        operations::{BatchUpdateEntryInput, BatchUpdateEntryKind, BatchUpdateEntryOutput},
    },
    project::Project,
};

impl<R: AppRuntime> Project<R> {
    pub async fn batch_update_entry(
        &self,
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        input: BatchUpdateEntryInput,
        channel: TauriChannel<BatchUpdateEntryEvent>,
    ) -> joinerror::Result<BatchUpdateEntryOutput> {
        for entry in input.entries {
            match entry {
                BatchUpdateEntryKind::Item(input) => {
                    let output = self.update_item_entry(ctx, app_delegate, input).await?;
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
