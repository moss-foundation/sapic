use joinerror::OptionExt;
use moss_applib::{AppRuntime, errors::NotFound};

use crate::{
    app::App,
    models::operations::{GetLocaleInput, GetLocaleOutput},
};

impl<R: AppRuntime> App<R> {
    pub async fn get_locale(
        &self,
        _ctx: &R::AsyncContext,
        input: &GetLocaleInput,
    ) -> joinerror::Result<GetLocaleOutput> {
        self.locale_service
            .get_locale(&input.identifier)
            .await
            .map(|locale| GetLocaleOutput {
                display_name: locale.display_name,
                code: locale.code,
                direction: locale.direction,
            })
            .ok_or_join_err::<NotFound>("locale not found")
    }
}
