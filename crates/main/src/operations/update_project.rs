use moss_applib::AppRuntime;
use sapic_ipc::{
    ValidationResultExt,
    contracts::main::project::{UpdateProjectInput, UpdateProjectOutput},
};
use validator::Validate;

use crate::MainWindow;

impl<R: AppRuntime> MainWindow<R> {
    pub async fn update_project(
        &self,
        ctx: &R::AsyncContext,
        input: &UpdateProjectInput,
    ) -> joinerror::Result<UpdateProjectOutput> {
        input.validate().join_err_bare()?;

        let id = input.inner.id.clone().into();

        self.workspace
            .load()
            .update_project(ctx, input.inner.clone())
            .await?;

        Ok(UpdateProjectOutput { id })
    }
}
