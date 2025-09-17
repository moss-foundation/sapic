use moss_applib::AppRuntime;

use crate::{
    app::App,
    models::operations::{GetTranslationNamespaceInput, GetTranslationNamespaceOutput},
};

impl<R: AppRuntime> App<R> {
    pub async fn get_translation_namespace(
        &self,
        _ctx: &R::AsyncContext,
        input: &GetTranslationNamespaceInput,
    ) -> joinerror::Result<GetTranslationNamespaceOutput> {
        let contents = self
            .locale_service
            .get_namespace(&input.language, &input.namespace)
            .await?;

        Ok(GetTranslationNamespaceOutput { contents })
    }
}
