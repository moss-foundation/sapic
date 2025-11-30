use moss_applib::AppRuntime;
use sapic_ipc::contracts::language::ListLanguagesOutput;

use crate::App;

impl<R: AppRuntime> App<R> {
    pub async fn list_languages(
        &self,
        _ctx: &R::AsyncContext,
    ) -> joinerror::Result<ListLanguagesOutput> {
        let languages = self.services.language_service.languages().await;

        Ok(ListLanguagesOutput(
            languages.into_iter().map(|(_, item)| item).collect(),
        ))
    }
}
