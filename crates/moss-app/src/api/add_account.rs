use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;

use crate::{
    App,
    models::operations::{AddAccountInput, AddAccountOutput},
};

impl<R: AppRuntime> App<R> {
    pub async fn add_account(
        &self,
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        input: AddAccountInput,
    ) -> joinerror::Result<AddAccountOutput> {
        let id = self
            .profile_service
            .add_account(
                ctx,
                app_delegate,
                input.profile_id,
                input.host,
                input.provider,
            )
            .await?;

        Ok(AddAccountOutput {
            account_id: id.to_string(),
        })
    }
}
