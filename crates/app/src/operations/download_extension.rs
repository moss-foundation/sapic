use moss_applib::AppRuntime;
use sapic_core::context::AnyAsyncContext;

use crate::App;

impl<R: AppRuntime> App<R> {
    pub async fn download_extension(
        &self,
        ctx: &dyn AnyAsyncContext,
        extension_id: &str,
        version: &str,
    ) -> joinerror::Result<String> {
        let id = self
            .extension_service
            .download_extension(
                ctx,
                extension_id,
                version,
                self.services.extension_api_service.clone(),
            )
            .await?;

        Ok(id)
    }
}
