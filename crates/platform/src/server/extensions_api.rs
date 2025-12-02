use async_trait::async_trait;
use joinerror::{OptionExt, ResultExt};
use sapic_base::extension::types::ExtensionInfo;
use sapic_core::context::{self, AnyAsyncContext, ContextResultExt};
use sapic_system::ports::server_api::ExtensionApiOperations;

use super::{HttpServerApiClient, types::ListExtensionsResponse};

const EXTENSIONS_REGISTRY_BASE_SEGMENT: &str = "extension-registry";
const CONTENT_DISPOSITION: &str = "Content-Disposition";

#[async_trait]
impl ExtensionApiOperations for HttpServerApiClient {
    async fn list_extensions(
        &self,
        ctx: &dyn AnyAsyncContext,
    ) -> joinerror::Result<Vec<ExtensionInfo>> {
        let response: ListExtensionsResponse = context::abortable(ctx, async {
            let resp = self
                .client
                .get(format!(
                    "{}/{EXTENSIONS_REGISTRY_BASE_SEGMENT}/extensions",
                    self.base_url
                ))
                .send()
                .await
                .join_err::<()>("failed to list extensions")?;

            if !resp.status().is_success() {
                let error_text = resp.text().await?;
                return Err(joinerror::Error::new::<()>(error_text));
            }

            resp.json()
                .await
                .join_err::<()>("failed to parse list extensions response")
        })
        .await
        .join_err_bare()?;

        Ok(response
            .extensions
            .into_iter()
            .map(|response| response.into())
            .collect())
    }

    async fn download_extension(
        &self,
        ctx: &dyn AnyAsyncContext,
        extension_id: &str,
        version: &str,
    ) -> joinerror::Result<(Vec<u8>, String)> {
        let (bytes, extension_name) = context::abortable(ctx, async {
            let resp = self
                .client
                .get(format!(
                    "{}/{EXTENSIONS_REGISTRY_BASE_SEGMENT}/extensions/{}/download/{}",
                    self.base_url, extension_id, version
                ))
                .send()
                .await
                .join_err::<()>("failed to download extension")?;

            if !resp.status().is_success() {
                let error_text = resp.text().await?;
                return Err(joinerror::Error::new::<()>(error_text));
            }

            // Extract extension name from file name

            let content_disposition = resp
                .headers()
                .get(CONTENT_DISPOSITION)
                .and_then(|v| v.to_str().ok())
                .ok_or_join_err::<()>("failed to get extension name")?;

            let extension_name =
                parse_extension_name_from_content_disposition(content_disposition)?;

            let bytes = resp
                .bytes()
                .await
                .join_err::<()>("failed to get extension tarball bytes")?;
            Ok((bytes, extension_name))
        })
        .await
        .join_err_bare()?;
        Ok((bytes.to_vec(), extension_name))
    }
}

fn parse_extension_name_from_content_disposition(
    content_disposition: &str,
) -> joinerror::Result<String> {
    // Content Disposition format:
    // `attachment; filename="xxx.tar.gz"`
    let parts = content_disposition.split("\"").collect::<Vec<_>>();
    let file_name = parts
        .get(1)
        .ok_or_join_err::<()>("failed to get filename")?;
    let extension_name = file_name
        .strip_suffix(".tar.gz")
        .ok_or_join_err::<()>("failed to get extension name")?;
    Ok(extension_name.to_string())
}
