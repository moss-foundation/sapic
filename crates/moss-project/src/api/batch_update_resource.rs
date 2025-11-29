use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use tauri::ipc::Channel as TauriChannel;

use crate::{
    errors::ErrorInternal,
    models::{
        events::BatchUpdateResourceEvent,
        operations::{
            BatchUpdateResourceInput, BatchUpdateResourceKind, BatchUpdateResourceOutput,
        },
    },
    project::Project,
};

impl Project {
    pub async fn batch_update_resource<R: AppRuntime>(
        &self,
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        input: BatchUpdateResourceInput,
        channel: TauriChannel<BatchUpdateResourceEvent>,
    ) -> joinerror::Result<BatchUpdateResourceOutput> {
        for entry in input.resources {
            match entry {
                BatchUpdateResourceKind::Item(input) => {
                    let output = self.update_item_resource(ctx, app_delegate, input).await?;
                    channel
                        .send(BatchUpdateResourceEvent::Item(output))
                        .map_err(|e| {
                            joinerror::Error::new::<ErrorInternal>(format!(
                                "failed to send to the tauri channel: {}",
                                e.to_string()
                            ))
                        })?;
                }
                BatchUpdateResourceKind::Dir(input) => {
                    let output = self.update_dir_resource::<R>(ctx, input).await?;
                    channel
                        .send(BatchUpdateResourceEvent::Dir(output))
                        .map_err(|e| {
                            joinerror::Error::new::<ErrorInternal>(format!(
                                "failed to send to the tauri channel: {}",
                                e.to_string()
                            ))
                        })?;
                }
            }
        }

        Ok(BatchUpdateResourceOutput {})
    }
}
