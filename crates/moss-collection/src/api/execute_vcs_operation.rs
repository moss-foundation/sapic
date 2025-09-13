use joinerror::OptionExt;
use moss_applib::AppRuntime;

use crate::{
    Collection,
    models::{
        operations::{ExecuteVcsOperationInput, ExecuteVcsOperationOutput},
        types::VcsOperation,
    },
};

impl<R: AppRuntime> Collection<R> {
    pub async fn execute_vcs_operation(
        &self,
        ctx: &R::AsyncContext,
        input: ExecuteVcsOperationInput,
    ) -> joinerror::Result<ExecuteVcsOperationOutput> {
        let vcs = self.vcs().ok_or_join_err::<()>("vcs not found")?;

        match input.operation {
            VcsOperation::Commit {
                message,
                paths,
                push,
            } => {
                // Stage and commit all those changes
                vcs.stage_and_commit(paths, &message).await?;
                if push {
                    vcs.push(ctx).await?;
                }
            }
            VcsOperation::Discard { paths } => {
                vcs.discard_changes(paths).await?;
            }
        }

        Ok(ExecuteVcsOperationOutput {})
    }
}
