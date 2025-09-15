use moss_applib::AppRuntime;

use crate::{
    app::App,
    models::operations::{GetTranslationInput, GetTranslationOutput},
};

impl<R: AppRuntime> App<R> {
    pub async fn get_translation(
        &self,
        _ctx: &R::AsyncContext,
        input: &GetTranslationInput,
    ) -> joinerror::Result<GetTranslationOutput> {
        let translations = self
            .locale_service
            .get_translation(&input.identifier)
            .await?;

        Ok(GetTranslationOutput {
            namespaces: translations.keys().cloned().collect(),
            translations,
        })
    }
}
