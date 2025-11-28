use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use moss_project::models::primitives::ProjectId;
use sapic_ipc::ValidationResultExt;
use validator::Validate;

use crate::{
    models::operations::{CreateProjectInput, CreateProjectOutput},
    workspace::Workspace,
};

impl<R: AppRuntime> Workspace<R> {
    pub async fn create_project(
        &self,
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        input: &CreateProjectInput,
    ) -> joinerror::Result<CreateProjectOutput> {
        input.validate().join_err_bare()?;

        let id = ProjectId::new();

        let account = if input.inner.git_params.is_some() {
            self.active_profile.first().await
        } else {
            None
        };
        let description = self
            .project_service
            .create_project(ctx, app_delegate, &id, account, &input.inner)
            .await?;

        Ok(CreateProjectOutput {
            id: description.id,
            name: description.name,
            order: description.order,
            expanded: description.expanded,
            icon_path: description.icon_path,
            abs_path: description.internal_abs_path,
            external_path: description.external_path,
        })
    }
}
