use moss_applib::AppRuntime;

use crate::{
    app::App,
    models::operations::{GetTranslationsInput, GetTranslationsOutput},
};

impl<R: AppRuntime> App<R> {
    pub async fn get_translations(
        &self,
        _ctx: &R::AsyncContext,
        input: &GetTranslationsInput,
    ) -> joinerror::Result<GetTranslationsOutput> {
        let translations = self
            .locale_service
            .read_translations_from_file(&input.language, &input.namespace)
            .await?;

        Ok(GetTranslationsOutput(translations))
    }
}
