use anyhow::Result;
use moss_fs::FileSystem;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::OnceCell;

use crate::models::{primitives::ThemeId, types::ColorThemeInfo};

const THEMES_REGISTRY_FILE: &str = "themes.json";

pub struct ThemeService {
    pub(crate) themes_dir: PathBuf,
    fs: Arc<dyn FileSystem>,
    themes: OnceCell<HashMap<ThemeId, ColorThemeInfo>>,
    default_theme: OnceCell<ColorThemeInfo>,
}

impl ThemeService {
    pub fn new(fs: Arc<dyn FileSystem>, themes_dir: PathBuf) -> Self {
        Self {
            themes_dir,
            fs,
            themes: Default::default(),
            default_theme: Default::default(),
        }
    }

    pub async fn default_theme(&self) -> Result<&ColorThemeInfo> {
        self.default_theme
            .get_or_try_init(|| async move {
                let themes = self.themes().await?;
                let default_theme = themes
                    .values()
                    .find(|theme| theme.is_default.unwrap_or(false))
                    .cloned();

                Ok::<ColorThemeInfo, anyhow::Error>(
                    default_theme.unwrap_or(
                        themes
                            .values()
                            .next() // We take the first theme as the default theme if no default theme is found
                            .expect("The app must have at least one theme")
                            .clone(),
                    ),
                )
            })
            .await
    }

    pub(crate) async fn themes(&self) -> Result<&HashMap<ThemeId, ColorThemeInfo>> {
        self.themes
            .get_or_try_init(|| async move {
                let descriptors = self.parse_registry_file().await?;
                let result = descriptors
                    .into_iter()
                    .map(|item| (item.identifier.clone(), item))
                    .collect::<HashMap<ThemeId, ColorThemeInfo>>();

                Ok::<_, anyhow::Error>(result)
            })
            .await
    }

    async fn parse_registry_file(&self) -> Result<Vec<ColorThemeInfo>> {
        let reader = self
            .fs
            .open_file(&self.themes_dir.join(THEMES_REGISTRY_FILE))
            .await?;

        Ok(serde_json::from_reader(reader)?)
    }
}
