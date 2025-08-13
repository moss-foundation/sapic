use moss_applib::AppRuntime;
use moss_git_hosting_provider::{GitHostingProvider, models::types::UserInfo};

use crate::App;

impl<R: AppRuntime> App<R> {
    pub async fn log_in_with_github(&self, _ctx: &R::AsyncContext) -> joinerror::Result<UserInfo> {
        self._github_client.login().await
    }
}
