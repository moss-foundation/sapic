use joinerror::ResultExt;
use moss_applib::AppRuntime;
use sapic_ipc::contracts::user::UpdateUserAccountInput;
use sapic_system::user::UpdateAccountParams;

use crate::App;

impl<R: AppRuntime> App<R> {
    pub async fn update_user_account(
        &self,
        ctx: &R::AsyncContext,
        input: &UpdateUserAccountInput,
    ) -> joinerror::Result<()> {
        self.user
            .update_account(
                ctx,
                &input.id,
                UpdateAccountParams {
                    pat: input.pat.clone(),
                },
            )
            .await
            .join_err_with::<()>(|| format!("failed to update account: {}", input.id))?;

        Ok(())
    }
}
