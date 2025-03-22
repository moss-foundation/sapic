use anyhow::{anyhow, Result};
use moss_app::service_pool::AppService;
use moss_fs::ports::FileSystem;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::OnceCell;

use crate::{
    models::{operations::ListThemesOutput, types::ThemeDescriptor},
    primitives::ThemeId,
};

const THEMES_REGISTRY_FILE: &str = "themes.json";

pub struct ThemeService {
    themes_dir: PathBuf,
    fs: Arc<dyn FileSystem>,
    themes: OnceCell<HashMap<ThemeId, ThemeDescriptor>>,
    default_theme: OnceCell<ThemeDescriptor>,
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

    pub async fn default_theme(&self) -> Result<&ThemeDescriptor> {
        self.default_theme
            .get_or_try_init(|| async move {
                let themes = self.themes().await?;
                let default_theme = themes
                    .values()
                    .find(|theme| theme.is_default.unwrap_or(false))
                    .cloned();

                Ok::<ThemeDescriptor, anyhow::Error>(
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

    async fn themes(&self) -> Result<&HashMap<ThemeId, ThemeDescriptor>> {
        self.themes
            .get_or_try_init(|| async move {
                let descriptors = self.parse_registry_file().await?;
                let result = descriptors
                    .into_iter()
                    .map(|item| (item.identifier.clone(), item))
                    .collect::<HashMap<ThemeId, ThemeDescriptor>>();

                Ok::<HashMap<ThemeId, ThemeDescriptor>, anyhow::Error>(result)
            })
            .await
    }

    pub async fn list_themes(&self) -> Result<ListThemesOutput> {
        let themes = self.themes().await?;

        Ok(ListThemesOutput {
            contents: themes.into_iter().map(|(_, item)| item).cloned().collect(),
        })
    }

    pub async fn read_color_theme(&self, id: &ThemeId) -> Result<String> {
        let themes = self.themes().await?;

        if let Some(descriptor) = themes.get(id) {
            let mut reader = self
                .fs
                .open_file(&self.themes_dir.join(descriptor.source.clone()))
                .await?;

            let mut content = String::new();
            reader.read_to_string(&mut content)?;

            Ok(content)
        } else {
            Err(anyhow!("theme with id `{id}` was not found"))
        }
    }

    async fn parse_registry_file(&self) -> Result<Vec<ThemeDescriptor>> {
        let reader = self
            .fs
            .open_file(&self.themes_dir.join(THEMES_REGISTRY_FILE))
            .await?;

        Ok(serde_json::from_reader(reader)?)
    }
}

impl AppService for ThemeService {}
