use moss_applib::AppRuntime;
use sapic_ipc::ValidationResultExt;
use validator::Validate;

use crate::{
    models::operations::{DeleteResourceInput, DeleteResourceOutput},
    project::Project,
};

impl Project {
    pub async fn delete_resource<R: AppRuntime>(
        &self,
        ctx: &R::AsyncContext,
        input: DeleteResourceInput,
    ) -> joinerror::Result<DeleteResourceOutput> {
        input.validate().join_err_bare()?;
        self.worktree().await.remove_entry(ctx, &input.id).await?;

        Ok(DeleteResourceOutput { id: input.id })
    }
}
