use joinerror::OptionExt;
use moss_applib::{AppRuntime, errors::NotFound};

use crate::{app::App, models::operations::GetProfileOutput};

impl<R: AppRuntime> App<R> {
    pub async fn get_profile(&self, _ctx: &R::AsyncContext) -> joinerror::Result<GetProfileOutput> {
        let profile = self.profile_service.active_profile().await;
        let details = self
            .profile_service
            .profile(&profile.id())
            .await
            .ok_or_join_err_with::<NotFound>(|| format!("profile `{}` not found", profile.id()))?; // INFO: should never happen since we should always have the active profile

        Ok(GetProfileOutput {
            id: profile.id().clone(),
            name: details.name,
            accounts: details.accounts,
        })
    }
}
