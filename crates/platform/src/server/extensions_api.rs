use async_trait::async_trait;
use joinerror::{OptionExt, ResultExt};
use sapic_base::extension::types::ExtensionInfo;
use sapic_core::context::{self, AnyAsyncContext, ContextResultExt};
use sapic_system::ports::server_api::ExtensionApiOperations;
use std::path::{Path, PathBuf};

use crate::server::types::ExtensionVersionInfoResponse;

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
        archive_folder: &Path,
    ) -> joinerror::Result<(PathBuf, String)> {
        let (path, extension_folder_name) = context::abortable(ctx, async {
            // Fetch info about the particular extension version
            let extension_response = self
                .client
                .get(format!(
                    "{}/{EXTENSIONS_REGISTRY_BASE_SEGMENT}/extensions/{extension_id}/{version}",
                    self.base_url
                ))
                .send()
                .await
                .join_err::<()>("failed to fetch extension info")?;

            if !extension_response.status().is_success() {
                let error_text = extension_response.text().await?;
                return Err(joinerror::Error::new::<()>(error_text));
            }

            let info: ExtensionVersionInfoResponse = extension_response.json().await?;

            let extension_folder_name = format!("{}@{}", info.id, info.version);

            // Download the file
            let file_resp = self
                .client
                .get(format!(
                    "{}/{EXTENSIONS_REGISTRY_BASE_SEGMENT}/extensions/{}/download/{}",
                    self.base_url, extension_id, version
                ))
                .send()
                .await
                .join_err::<()>("failed to download extension")?;

            if !file_resp.status().is_success() {
                let error_text = file_resp.text().await?;
                return Err(joinerror::Error::new::<()>(error_text));
            }

            // Find the archive file name indicated by the response
            let content_disposition = file_resp
                .headers()
                .get(CONTENT_DISPOSITION)
                .and_then(|v| v.to_str().ok())
                .ok_or_join_err::<()>("failed to get extension name")?;

            let archive_name = parse_archive_name_from_content_disposition(content_disposition)?;

            // Write the archive file to the provided folder
            let bytes = file_resp
                .bytes()
                .await
                .join_err::<()>("failed to get extension tarball bytes")?;

            let path = archive_folder.join(&archive_name);
            tokio::fs::write(&path, bytes).await?;

            Ok((path, extension_folder_name))
        })
        .await
        .join_err_bare()?;
        Ok((path, extension_folder_name))
    }
}

fn parse_archive_name_from_content_disposition(
    content_disposition: &str,
) -> joinerror::Result<String> {
    // Content Disposition format:
    // `attachment; filename="xxx.tar.gz"`
    let parts = content_disposition.split("\"").collect::<Vec<_>>();
    let file_name = parts
        .get(1)
        .ok_or_join_err::<()>("failed to get filename")?;
    Ok(file_name.to_string())
}
