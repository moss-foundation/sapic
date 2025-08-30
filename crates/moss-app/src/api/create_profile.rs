use moss_applib::AppRuntime;

use crate::{
    app::App,
    models::operations::{CreateProfileInput, CreateProfileOutput},
};

impl<R: AppRuntime> App<R> {
    pub async fn create_profile(
        &self,
        _ctx: &R::AsyncContext,
        input: CreateProfileInput,
    ) -> joinerror::Result<CreateProfileOutput> {
        let id = self.profile_service.create_profile(input.name).await?;

        Ok(CreateProfileOutput {
            profile_id: id.to_string(),
        })
    }
}
