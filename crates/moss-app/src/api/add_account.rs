use moss_applib::AppRuntime;

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
        let id = self
            .profile_service
            .add_account(input.profile_id, input.host, input.provider)
            .await?;

        Ok(AddAccountOutput {
            account_id: id.to_string(),
        })

        // let user_info = match input.git_provider_type {
        //     GitProviderType::GitHub => self._github_client.login().await,
        //     GitProviderType::GitLab => self._gitlab_client.login().await,
        // }?;

        // Ok(AddAccountOutput { user_info })
    }
}
