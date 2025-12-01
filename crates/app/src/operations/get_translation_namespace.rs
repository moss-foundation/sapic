use moss_applib::AppRuntime;
use sapic_ipc::contracts::language::{GetTranslationNamespaceInput, GetTranslationNamespaceOutput};

use crate::App;

impl<R: AppRuntime> App<R> {
    pub async fn get_translation_namespace(
        &self,
        _ctx: &R::AsyncContext,
        input: &GetTranslationNamespaceInput,
    ) -> joinerror::Result<GetTranslationNamespaceOutput> {
        let contents = self
            .services
            .language_service
            .get_namespace(&input.language, &input.namespace)
            .await?;

        Ok(GetTranslationNamespaceOutput { contents })
    }
}
