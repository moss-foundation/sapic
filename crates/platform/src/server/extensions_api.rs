use async_trait::async_trait;
use joinerror::ResultExt;
use nanoid::nanoid;
use sapic_base::extension::types::{ExtensionInfo, ExtensionVersionInfo};
use sapic_core::context::{self, AnyAsyncContext, ContextResultExt};
use sapic_system::ports::server_api::ExtensionApiOperations;
use std::path::{Path, PathBuf};

use crate::server::types::ExtensionVersionInfoResponse;

use super::{HttpServerApiClient, types::ListExtensionsResponse};

const EXTENSIONS_REGISTRY_BASE_SEGMENT: &str = "extension-registry";

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
    ) -> joinerror::Result<(PathBuf, ExtensionVersionInfo)> {
        let (path, info) = context::abortable(ctx, async {
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

            // We will always get a .tar.gz file from the extension registry
            // Generate a random archive file name
            let archive_name = format!("{}.tar.gz", nanoid!(10));

            // Write the archive file to the provided folder
            let bytes = file_resp
                .bytes()
                .await
                .join_err::<()>("failed to get extension tarball bytes")?;

            let path = archive_folder.join(&archive_name);
            tokio::fs::write(&path, bytes).await?;

            Ok((path, info.into()))
        })
        .await
        .join_err_bare()?;
        Ok((path, info))
    }
}
