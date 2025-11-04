use moss_applib::AppRuntime;

use crate::{app::Window, models::operations::ListLanguagesOutput};

impl<R: AppRuntime> Window<R> {
    pub async fn list_languages(
        &self,
        _ctx: &R::AsyncContext,
    ) -> joinerror::Result<ListLanguagesOutput> {
        let languages = self.language_service.languages().await;

        Ok(ListLanguagesOutput(
            languages.into_iter().map(|(_, item)| item).collect(),
        ))
    }
}
