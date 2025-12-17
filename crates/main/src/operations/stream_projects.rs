use moss_applib::AppRuntime;
use sapic_ipc::contracts::main::project::{StreamProjectsEvent, StreamProjectsOutput};
use tauri::ipc::Channel as TauriChannel;

use crate::MainWindow;

impl<R: AppRuntime> MainWindow<R> {
    pub async fn stream_projects(
        &self,
        ctx: &R::AsyncContext,
        channel: TauriChannel<StreamProjectsEvent>,
    ) -> joinerror::Result<StreamProjectsOutput> {
        let projects = self.workspace.load().projects(ctx).await?;

        for project in projects {
            let details = if let Ok(details) = project.handle.details(ctx).await {
                details
            } else {
                continue;
            };

            let vcs = if let Some(vcs) = project.handle.vcs::<R>() {
                Some(vcs.summary(ctx).await?)
            } else {
                None
            };

            let event = StreamProjectsEvent {
                id: project.id,
                name: details.name,
                order: project.order,
                expanded: false, // HACK: hardcoded value
                branch: vcs.map(|vcs| vcs.branch),
                icon_path: None, // HACK: hardcoded value
                archived: details.archived,
            };

            if let Err(e) = channel.send(event) {
                tracing::error!(
                    "failed to send project event through tauri channel: {}",
                    e.to_string()
                );
            }
        }

        todo!()
    }
}
