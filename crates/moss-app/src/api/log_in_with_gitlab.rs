use crate::App;
use moss_applib::AppRuntime;
use moss_git_hosting_provider::models::types::UserInfo;

impl<R: AppRuntime> App<R> {
    pub async fn log_in_with_gitlab(&self, _ctx: &R::AsyncContext) -> joinerror::Result<UserInfo> {
        self._gitlab_client.login().await
    }
}
