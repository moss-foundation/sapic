use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;

use crate::{
    App,
    models::operations::{RemoveAccountInput, RemoveAccountOutput},
};

impl<R: AppRuntime> App<R> {
    pub async fn remove_account(
        &self,
        app_delegate: &AppDelegate<R>,
        input: RemoveAccountInput,
    ) -> joinerror::Result<RemoveAccountOutput> {
        self.profile_service
            .remove_account(app_delegate, input.account_id)
            .await?;

        Ok(RemoveAccountOutput {})
    }
}
