use joinerror::OptionExt;
use moss_applib::errors::NotFound;
use moss_fs::FileSystem;
use sapic_base::theme::types::{ColorThemeInfo, primitives::ThemeId};
use sapic_platform::theme::loader::ThemeLoader;
use std::{collections::HashMap, path::PathBuf, sync::Arc};

use super::theme_registry::ThemeRegistry;

pub struct ThemeService {
    loader: ThemeLoader,
    registry: Arc<dyn ThemeRegistry>,
}

impl ThemeService {
    pub async fn new(
        resource_dir: PathBuf,
        fs: Arc<dyn FileSystem>,
        registry: Arc<dyn ThemeRegistry>,
    ) -> joinerror::Result<Self> {
        Ok(Self {
            registry,
            loader: ThemeLoader::new(fs, resource_dir.join("policies/theme.rego")),
        })
    }

    pub async fn themes(&self) -> HashMap<ThemeId, ColorThemeInfo> {
        let themes = self.registry.list().await;
        themes
            .into_iter()
            .map(|(id, item)| {
                (
                    id,
                    ColorThemeInfo {
                        identifier: item.id,
                        display_name: item.display_name,
                        mode: item.mode,
                        order: None, // FIXME
                        source: item.path,
                        is_default: None, // FIXME
                    },
                )
            })
            .collect()
    }

    pub async fn read(&self, id: &ThemeId) -> joinerror::Result<String> {
        let item = self
            .registry
            .get(id)
            .await
            .ok_or_join_err_with::<NotFound>(|| format!("theme with id `{}` not found", id))?;

        let theme = self.loader.load(&item.path).await?;

        // TODO: apply color theme token overrides

        let css = sapic_base::theme::convert(&theme).await?;

        Ok(css)
    }
}
