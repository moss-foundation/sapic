use moss_applib::AppRuntime;
use moss_common::api::OperationResult;

use crate::{
    app::App,
    models::operations::{GetTranslationsInput, GetTranslationsOutput},
    services::locale_service::LocaleService,
};

impl<R: AppRuntime> App<R> {
    pub async fn get_translations(
        &self,
        _ctx: &R::AsyncContext,
        input: &GetTranslationsInput,
    ) -> OperationResult<GetTranslationsOutput> {
        let locale_service = self.services.get::<LocaleService>();
        let translations = locale_service
            .read_translations_from_file(&input.language, &input.namespace)
            .await?;

        Ok(GetTranslationsOutput(translations))
    }
}
