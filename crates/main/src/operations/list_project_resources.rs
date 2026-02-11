use std::path::PathBuf;

use moss_applib::AppRuntime;
use sapic_ipc::contracts::main::resource::{
    ListProjectResourcesInput, ListProjectResourcesMode, ListProjectResourcesOutput,
};

use crate::MainWindow;

impl<R: AppRuntime> MainWindow<R> {
    pub async fn list_project_resources(
        &self,
        ctx: &R::AsyncContext,
        input: ListProjectResourcesInput,
    ) -> joinerror::Result<ListProjectResourcesOutput> {
        let project = self
            .workspace
            .load()
            .project(ctx, &input.project_id)
            .await?;

        let dirs = match input.mode {
            ListProjectResourcesMode::LoadRoot => vec![PathBuf::from("")],
            ListProjectResourcesMode::ReloadPath(path) => vec![path],
        };

        let items = project.resources(ctx, dirs).await?;

        Ok(ListProjectResourcesOutput { items })
    }
}
