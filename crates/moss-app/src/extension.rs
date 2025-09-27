mod scanner;

use async_trait::async_trait;
use joinerror::OptionExt;
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use moss_common::continue_if_err;
use moss_fs::FileSystem;
use moss_logging::session;
use rustc_hash::FxHashMap;
use serde_json::Value as JsonValue;
use std::{path::PathBuf, sync::Arc};

use crate::extension::scanner::{ExtensionKind, ExtensionScanner};

pub struct ContributionInfo {
    pub source: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContributionKey {
    Configuration,
    Themes,
    Localizations,
    ResourceParams,
}

impl std::fmt::Display for ContributionKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContributionKey::Configuration => write!(f, "configuration"),
            ContributionKey::Themes => write!(f, "themes"),
            ContributionKey::Localizations => write!(f, "localizations"),
            ContributionKey::ResourceParams => write!(f, "resource_params"),
        }
    }
}

impl TryFrom<&str> for ContributionKey {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "configuration" => Ok(ContributionKey::Configuration),
            "themes" => Ok(ContributionKey::Themes),
            "localizations" => Ok(ContributionKey::Localizations),
            "resource_params" => Ok(ContributionKey::ResourceParams),
            _ => Err(format!("Unknown contribution key: {}", value)),
        }
    }
}

#[async_trait]
pub trait ExtensionPoint<R: AppRuntime>: Send + Sync + 'static {
    fn key(&self) -> ContributionKey;
    async fn handle(
        &self,
        app_delegate: &AppDelegate<R>,
        info: &ContributionInfo,
        data: JsonValue,
    ) -> joinerror::Result<()>;
}

pub struct ExtensionService<R: AppRuntime> {
    scanner: ExtensionScanner,
    points: FxHashMap<ContributionKey, Box<dyn ExtensionPoint<R>>>,
    fs: Arc<dyn FileSystem>,
}

impl<R: AppRuntime> ExtensionService<R> {
    pub async fn new(
        app_delegate: &AppDelegate<R>,
        fs: Arc<dyn FileSystem>,
        points: impl IntoIterator<Item = impl ExtensionPoint<R>>,
    ) -> joinerror::Result<Self> {
        let points: FxHashMap<ContributionKey, Box<dyn ExtensionPoint<R>>> = points
            .into_iter()
            .map(|p| (p.key(), Box::new(p) as Box<dyn ExtensionPoint<R>>))
            .collect();

        let scanner = ExtensionScanner::new(
            fs.clone(),
            // HACK: the paths are temporarily hardcoded here, later they will need
            // to be retrieved either from the app delegate or in some other dynamic way.
            vec![
                (
                    PathBuf::from(
                        std::env::var("DEV_APPLICATION_DIR")
                            .expect("DEV_APPLICATION_DIR is not set"),
                    )
                    .join("addons"),
                    ExtensionKind::BuiltIn,
                ),
                (
                    PathBuf::from(std::env::var("DEV_USER_DIR").expect("DEV_USER_DIR is not set"))
                        .join("addons"),
                    ExtensionKind::User,
                ),
            ],
        );

        let descriptions = scanner.scan().await?;
        for desc in descriptions {
            let info = ContributionInfo {
                source: desc.abs_path.clone(),
            };

            for (key, value) in desc.contributes {
                if !(value.is_object() || value.is_array()) {
                    session::warn!(format!(
                        "Invalid contribution value: {} from extension: {}",
                        value,
                        desc.abs_path.display()
                    ));
                    continue;
                }

                let key = continue_if_err!(ContributionKey::try_from(key.as_str()), |err| {
                    session::warn!(format!(
                        "Failed to parse contribution key: {} from extension: {}",
                        err,
                        desc.abs_path.display()
                    ));
                });

                let point = points
                    .get(&key)
                    // Error should never happen, if it does, it's definitely a bug.
                    .ok_or_join_err::<()>(format!(
                        "Failed to find extension point for key: {}",
                        key
                    ))?;

                continue_if_err!(point.handle(app_delegate, &info, value).await, |err| {
                    session::error!(format!(
                        "Failed to handle contribution: {} from extension: {}",
                        err,
                        desc.abs_path.display()
                    ));
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
