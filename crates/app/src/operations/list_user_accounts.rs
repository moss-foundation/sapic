use moss_applib::AppRuntime;
use sapic_ipc::contracts::user::ListUserAccountsOutput;

use crate::App;

impl<R: AppRuntime> App<R> {
    pub async fn list_user_accounts(
        &self,
        _ctx: &R::AsyncContext,
    ) -> joinerror::Result<ListUserAccountsOutput> {
        Ok(ListUserAccountsOutput {
            accounts: self.user.accounts().await,
        })
    }
}
