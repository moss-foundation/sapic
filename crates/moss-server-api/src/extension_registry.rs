// use async_trait::async_trait;
// use joinerror::ResultExt;
// use moss_app_delegate::AppDelegate;
// use moss_applib::AppRuntime;
// use reqwest::Client as HttpClient;
// use sapic_core::context::{self, ContextResultExt};
// use serde::Deserialize;
// use std::sync::Arc;

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct ExtensionInfoResponse {
//     pub id: String,
//     pub external_id: String,
//     pub name: String,
//     pub authors: Vec<String>,
//     pub description: String,
//     pub repository: String,
//     pub downloads: u64,
//     pub created_at: String,
//     pub updated_at: String,
//     pub latest_version: String,
// }

// #[derive(Debug, Deserialize)]
// pub struct ListExtensionsResponse {
//     pub extensions: Vec<ExtensionInfoResponse>,
// }

// #[async_trait]
// pub trait ListExtensionsApiReq<R: AppRuntime>: Send + Sync {
//     async fn list_extensions(
//         &self,
//         ctx: &R::AsyncContext,
//     ) -> joinerror::Result<ListExtensionsResponse>;
// }

// #[async_trait]
// pub trait ExtensionRegistryApiClient<R: AppRuntime>: Send + Sync + ListExtensionsApiReq<R> {}

// struct GlobalExtensionRegistryApiClient<R: AppRuntime>(Arc<dyn ExtensionRegistryApiClient<R>>);

// impl<R: AppRuntime> dyn ExtensionRegistryApiClient<R> {
//     pub fn global(delegate: &AppDelegate<R>) -> Arc<dyn ExtensionRegistryApiClient<R>> {
//         delegate
//             .global::<GlobalExtensionRegistryApiClient<R>>()
//             .0
//             .clone()
//     }
//     pub fn set_global(delegate: &AppDelegate<R>, v: Arc<dyn ExtensionRegistryApiClient<R>>) {
//         delegate.set_global(GlobalExtensionRegistryApiClient(v))
//     }
// }

// #[derive(Clone)]
// pub struct AppExtensionRegistryApiClient {
//     base_url: Arc<String>,
//     client: HttpClient,
// }

// impl AppExtensionRegistryApiClient {
//     pub fn new(client: HttpClient, base_url: String) -> Self {
//         Self {
//             base_url: base_url.into(),
//             client,
//         }
//     }

//     pub fn base_url(&self) -> Arc<String> {
//         self.base_url.clone()
//     }
// }

// #[async_trait]
// impl<R: AppRuntime> ListExtensionsApiReq<R> for AppExtensionRegistryApiClient {
//     async fn list_extensions(
//         &self,
//         ctx: &R::AsyncContext,
//     ) -> joinerror::Result<ListExtensionsResponse> {
//         context::abortable(ctx, async {
//             let resp = self
//                 .client
//                 .get(format!("{}/extensions", self.base_url))
//                 .send()
//                 .await
//                 .join_err::<()>("failed to list extensions")?;

//             if !resp.status().is_success() {
//                 let error_text = resp.text().await?;
//                 return Err(joinerror::Error::new::<()>(error_text));
//             }

//             resp.json()
//                 .await
//                 .join_err::<()>("failed to parse list extensions response")
//         })
//         .await
//         .join_err_bare()
//     }
// }

// impl<R: AppRuntime> ExtensionRegistryApiClient<R> for AppExtensionRegistryApiClient {}
