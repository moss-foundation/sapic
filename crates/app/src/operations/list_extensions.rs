use moss_applib::AppRuntime;
use sapic_base::extension::types::ExtensionInfo;
use sapic_ipc::contracts::extension::ListExtensionsOutput;

use crate::App;

impl<R: AppRuntime> App<R> {
    pub async fn list_extensions(
        &self,
        ctx: &R::AsyncContext,
    ) -> joinerror::Result<ListExtensionsOutput> {
        let extensions = self
            .services
            .extension_api_service
            .list_extensions(ctx)
            .await?;

        Ok(ListExtensionsOutput(
            extensions
                .into_iter()
                .map(|extension| ExtensionInfo {
                    id: extension.id,
                    external_id: extension.external_id,
                    name: extension.name,
                    authors: extension.authors,
                    description: extension.description,
                    repository: extension.repository,
                    downloads: extension.downloads,
                    created_at: extension.created_at,
                    updated_at: extension.updated_at,
                    latest_version: extension.latest_version,
                })
                .collect(),
        ))
    }
}
