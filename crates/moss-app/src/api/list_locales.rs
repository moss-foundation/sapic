use moss_applib::AppRuntime;
use moss_common::api::OperationResult;

use crate::{
    app::App, models::operations::ListLocalesOutput, services::locale_service::LocaleService,
};

impl<R: AppRuntime> App<R> {
    pub async fn list_locales(&self, _ctx: &R::AsyncContext) -> OperationResult<ListLocalesOutput> {
        let locale_service = self.services.get::<LocaleService>();
        let locales = locale_service.locales().await?;

        Ok(ListLocalesOutput(
            locales.into_iter().map(|(_, item)| item).cloned().collect(),
        ))
    }
}
