use joinerror::ResultExt;
use moss_applib::AppRuntime;
use sapic_ipc::contracts::user::AddUserAccountInput;
use sapic_runtime::user::AddAccountParams;

use crate::App;

impl<R: AppRuntime> App<R> {
    pub async fn add_user_account(
        &self,
        ctx: &R::AsyncContext,
        input: &AddUserAccountInput,
    ) -> joinerror::Result<()> {
        self.user
            .add_account(
                ctx,
                AddAccountParams {
                    host: input.host.clone(),
                    kind: input.kind.clone(),
                    pat: input.pat.clone(),
                },
            )
            .await
            .join_err::<()>("failed to add account")?;

        Ok(())
    }
}
