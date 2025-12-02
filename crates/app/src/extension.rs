// TODO: move to system/application/ services

use joinerror::OptionExt;
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use moss_common::continue_if_err;
use sapic_base::extension::types::LoadedExtensionInfo;
use sapic_platform::extension::scanner::{ExtensionScanner, ExtensionsKind};
use sapic_runtime::extension_point::ExtensionPoint;
use std::path::PathBuf;

use flate2::bufread::GzDecoder;
use moss_fs::FileSystem;
use rustc_hash::FxHashMap;
use sapic_core::context::AnyAsyncContext;
use sapic_system::application::extensions_service::ExtensionsApiService;
use std::sync::Arc;
use tar::Archive;

#[allow(unused)]
pub struct ExtensionService<R: AppRuntime> {
    scanner: ExtensionScanner,
    points: FxHashMap<&'static str, Box<dyn ExtensionPoint<R>>>,
    fs: Arc<dyn FileSystem>,
    user_extensions_path: PathBuf,
}

impl<R: AppRuntime> ExtensionService<R> {
    pub async fn new(
        app_delegate: &AppDelegate<R>,
        fs: Arc<dyn FileSystem>,
        points: Vec<Box<dyn ExtensionPoint<R>>>,
    ) -> joinerror::Result<Self> {
        let points: FxHashMap<&'static str, Box<dyn ExtensionPoint<R>>> =
            points.into_iter().map(|p| (p.key().as_str(), p)).collect();

        let user_extensions_path = app_delegate.user_dir().join("extensions");

        let scanner = ExtensionScanner::new(
            fs.clone(),
            vec![
                (
                    app_delegate.resource_dir().join("extensions"),
                    ExtensionsKind::BuiltIn,
                ),
                (user_extensions_path.clone(), ExtensionsKind::User),
            ],
        );

        let descriptions = scanner.scan().await?;
        for desc in descriptions {
            let info = LoadedExtensionInfo {
                source: desc.abs_path.clone(),
            };

            for (key, value) in desc.contributes {
                if !(value.is_object() || value.is_array()) {
                    // FIXME: cant use session log here because it's not initialized yet
                    // session::warn!(format!(
                    //     "Invalid contribution value: {} from extension: {}",
                    //     value,
                    //     desc.abs_path.display()
                    // ));

                    println!(
                        "ERROR: invalid contribution value: {} from extension: {}",
                        value,
                        desc.abs_path.display()
                    );

                    continue;
                }

                let point = points
                    .get(key.as_str())
                    // Error should never happen, if it does, it's definitely a bug.
                    .ok_or_join_err::<()>(format!(
                        "Failed to find extension point for key: {}",
                        key
                    ))?;

                continue_if_err!(point.handle(app_delegate, &info, value).await, |err| {
                    // FIXME: cant use session log here because it's not initialized yet
                    // session::error!(format!(
                    //     "Failed to handle contribution: {} from extension: {}",
                    //     err,
                    //     desc.abs_path.display()
                    // ));

                    println!(
                        "ERROR: failed to handle contribution: {} from extension: {}",
                        err,
                        desc.abs_path.display()
                    );
                });
            }
        }

        Ok(Self {
            fs,
            points,
            scanner,
            user_extensions_path,
        })
    }

    // TODO: Trigger rescan after downloading extension?
    pub async fn download_extension(
        &self,
        ctx: &dyn AnyAsyncContext,
        extension_id: &str,
        version: &str,
        api: Arc<ExtensionsApiService>,
    ) -> joinerror::Result<()> {
        // TODO: Extract tar ball extraction logic?
        // Our fs only has logic for zip, not tar.gz yet
        let (tar_bytes, extension_name) =
            api.download_extension(ctx, extension_id, version).await?;

        let output_path = self.user_extensions_path.join(&extension_name);
        if output_path.exists() {
            return Err(joinerror::Error::new::<()>(format!(
                "An extension named `{}` already exists",
                extension_name
            )));
        }

        // Decompress the gzipped data
        let gz_decoder = GzDecoder::new(tar_bytes.as_slice());

        // Create a tar archive from the decompressed data
        let mut archive = Archive::new(gz_decoder);

        archive.unpack(output_path)?;

        Ok(())
    }
}
