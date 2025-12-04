// TODO: move to system/application/ services

use joinerror::OptionExt;
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use moss_common::continue_if_err;
use sapic_base::extension::types::LoadedExtensionInfo;
use sapic_platform::extension::scanner::{ExtensionScanner, ExtensionsKind};
use sapic_runtime::extension_point::ExtensionPoint;

use moss_fs::FileSystem;
use rustc_hash::FxHashMap;
use std::sync::Arc;

#[allow(unused)]
pub struct ExtensionService<R: AppRuntime> {
    scanner: ExtensionScanner,
    points: FxHashMap<&'static str, Box<dyn ExtensionPoint<R>>>,
    fs: Arc<dyn FileSystem>,
}

impl<R: AppRuntime> ExtensionService<R> {
    pub async fn new(
        app_delegate: &AppDelegate<R>,
        fs: Arc<dyn FileSystem>,
        points: Vec<Box<dyn ExtensionPoint<R>>>,
    ) -> joinerror::Result<Self> {
        let points: FxHashMap<&'static str, Box<dyn ExtensionPoint<R>>> =
            points.into_iter().map(|p| (p.key().as_str(), p)).collect();

        let scanner = ExtensionScanner::new(
            fs.clone(),
            vec![
                (
                    app_delegate.resource_dir().join("extensions"),
                    ExtensionsKind::BuiltIn,
                ),
                (app_delegate.user_extensions_dir(), ExtensionsKind::User),
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
        })
    }
}
