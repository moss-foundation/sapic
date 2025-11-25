use moss_applib::AppRuntime;

use crate::{
    models::operations::{GetTranslationNamespaceInput, GetTranslationNamespaceOutput},
    window::OldSapicWindow,
};

impl<R: AppRuntime> OldSapicWindow<R> {
    pub async fn get_translation_namespace(
        &self,
        _ctx: &R::AsyncContext,
        input: &GetTranslationNamespaceInput,
    ) -> joinerror::Result<GetTranslationNamespaceOutput> {
        let contents = self
            .language_service
            .get_namespace(&input.language, &input.namespace)
            .await?;

        Ok(GetTranslationNamespaceOutput { contents })
    }
}
