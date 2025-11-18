use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use moss_extension::models::types::ExtensionInfo;
use moss_server_api::extension_registry::ExtensionRegistryApiClient;

// use crate::{Window, models::operations::ListExtensionsOutput};

// impl<R: AppRuntime> Window<R> {
//     pub async fn list_extensions(
//         &self,
//         ctx: &R::AsyncContext,
//         app_delegate: &AppDelegate<R>,
//     ) -> joinerror::Result<ListExtensionsOutput> {
//         let api_client = <dyn ExtensionRegistryApiClient<R>>::global(app_delegate);

//         let result = api_client.list_extensions(ctx).await?;
//         Ok(ListExtensionsOutput(
//             result
//                 .extensions
//                 .into_iter()
//                 .map(|info| ExtensionInfo {
//                     id: info.id,
//                     external_id: info.external_id,
//                     name: info.name,
//                     authors: info.authors,
//                     description: info.description,
//                     repository: info.repository,
//                     downloads: info.downloads,
//                     created_at: info.created_at,
//                     updated_at: info.updated_at,
//                     latest_version: info.latest_version,
//                 })
//                 .collect(),
//         ))
//     }
// }
