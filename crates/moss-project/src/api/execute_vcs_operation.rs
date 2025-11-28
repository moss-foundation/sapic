use joinerror::OptionExt;
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;

use crate::{
    Project,
    models::{
        operations::{ExecuteVcsOperationInput, ExecuteVcsOperationOutput},
        types::VcsOperation,
    },
};

impl Project {
    pub async fn execute_vcs_operation<R: AppRuntime>(
        &self,
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
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
            VcsOperation::Push => {
                vcs.push(ctx).await?;
            }
            VcsOperation::Pull => {
                vcs.pull(ctx, app_delegate).await?;
            }
            VcsOperation::Fetch => {
                vcs.fetch(ctx).await?;
            }
        }

        Ok(ExecuteVcsOperationOutput {})
    }
}
