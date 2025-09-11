use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;

use crate::{
    app::App,
    models::operations::{CreateProfileInput, CreateProfileOutput},
};

impl<R: AppRuntime> App<R> {
    pub async fn create_profile(
        &self,
        _ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        input: CreateProfileInput,
    ) -> joinerror::Result<CreateProfileOutput> {
        let id = self
            .profile_service
            .create_profile(app_delegate, input.name, input.is_default.unwrap_or(false))
            .await?;

        Ok(CreateProfileOutput {
            profile_id: id.to_string(),
        })
    }
}
