use joinerror::ResultExt;
use moss_applib::AppRuntime;
use sapic_ipc::contracts::user::RemoveUserAccountInput;

use crate::App;

impl<R: AppRuntime> App<R> {
    pub async fn remove_user_account(
        &self,
        ctx: &R::AsyncContext,
        input: &RemoveUserAccountInput,
    ) -> joinerror::Result<()> {
        self.user
            .remove_account(ctx, &input.id)
            .await
            .join_err_with::<()>(|| format!("failed to remove account: {}", input.id))?;

        Ok(())
    }
}
