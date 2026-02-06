use joinerror::ResultExt;
use moss_applib::AppRuntime;
use sapic_ipc::contracts::main::project::{ListProjectItem, ListProjectsOutput};

use crate::MainWindow;

impl<R: AppRuntime> MainWindow<R> {
    pub async fn list_projects(
        &self,
        ctx: &R::AsyncContext,
    ) -> joinerror::Result<ListProjectsOutput> {
        let projects = self.workspace.load().projects(ctx).await?;

        let mut items = vec![];
        for project in projects {
            let details = project.handle.details(ctx).await.join_err_with::<()>(|| {
                format!(
                    "failed to get details for project {}",
                    project.id.to_string()
                )
            })?;

            let vcs = if let Some(vcs) = project.handle.vcs::<R>() {
                Some(vcs.summary(ctx).await?)
            } else {
                None
            };

            items.push(ListProjectItem {
                id: project.id.clone(),
                name: details.name,
                branch: vcs.map(|vcs| vcs.branch),
                icon_path: project.handle.icon_path(),
                archived: details.archived,
            });
        }

        Ok(ListProjectsOutput { items })
    }
}
