use moss_applib::{AppHandle, AppRuntime};

use crate::{
    App,
    models::operations::{AddAccountInput, AddAccountOutput},
};

impl<R: AppRuntime> App<R> {
    pub async fn add_account(
        &self,
        _ctx: &R::AsyncContext,
        app_handle: &AppHandle<R>,
        input: AddAccountInput,
    ) -> joinerror::Result<AddAccountOutput> {
        let id = self
            .profile_service
            .add_account(app_handle, input.profile_id, input.host, input.provider)
            .await?;

        Ok(AddAccountOutput {
            account_id: id.to_string(),
        })
    }
}
