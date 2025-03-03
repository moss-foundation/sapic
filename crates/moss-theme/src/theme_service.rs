use anyhow::{anyhow, Result};
use moss_app::service::AppService;
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
}

impl ThemeService {
    pub fn new(fs: Arc<dyn FileSystem>, themes_dir: PathBuf) -> Self {
        Self {
            themes_dir,
            fs,
            themes: Default::default(),
        }
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
                .open_file(&self.themes_dir.join(descriptor.path.clone()))
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

impl AppService for ThemeService {
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    fn dispose(&self) {}

    fn as_any(&self) -> &(dyn std::any::Any + Send) {
        self
    }
}
