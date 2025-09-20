use moss_applib::AppRuntime;
use moss_logging::session;

use crate::{models::operations::UpdateLayoutInput, workspace::Workspace};

impl<R: AppRuntime> Workspace<R> {
    pub async fn update_layout(
        &self,
        ctx: &R::AsyncContext,
        input: UpdateLayoutInput,
    ) -> joinerror::Result<()> {
        if let Some(data) = input.editor {
            if let Err(err) = self.layout_service.update_editor_layout(ctx, data).await {
                session::error!(format!("failed to update editor layout: {}", err));
            };
        }

        if let Some(data) = input.sidebar {
            if let Err(err) = self.layout_service.update_sidebar_layout(ctx, data).await {
                session::error!(format!("failed to update sidebar layout: {}", err));
            };
        }

        if let Some(data) = input.panel {
            if let Err(err) = self.layout_service.update_panel_layout(ctx, data).await {
                session::error!(format!("failed to update panel layout: {}", err));
            };
        }

        if let Some(data) = input.activitybar {
            if let Err(err) = self
                .layout_service
                .update_activitybar_layout(ctx, data)
                .await
            {
                session::error!(format!("failed to update activitybar layout: {}", err));
            };
        }

        Ok(())
    }
}
