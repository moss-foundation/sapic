use moss_applib::AppRuntime;
use moss_git_hosting_provider::models::primitives::GitProviderType;

use crate::{
    App,
    models::operations::{AddAccountInput, AddAccountOutput},
};

impl<R: AppRuntime> App<R> {
    pub async fn add_account(
        &self,
        _ctx: &R::AsyncContext,
        input: AddAccountInput,
    ) -> joinerror::Result<AddAccountOutput> {
        let user_info = match input.git_provider_type {
            GitProviderType::GitHub => self._github_client.login().await,
            GitProviderType::GitLab => self._gitlab_client.login().await,
        }?;

        Ok(AddAccountOutput { user_info })
    }
}
