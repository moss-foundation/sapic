use joinerror::OptionExt;
use moss_applib::errors::NotFound;
use moss_fs::FileSystem;
use moss_theme::{loader::ThemeLoader, models::primitives::ThemeId, registry::ThemeRegistry};

use std::{collections::HashMap, path::PathBuf, sync::Arc};

use crate::models::types::ColorThemeInfo;

pub struct ThemeService {
    loader: ThemeLoader,
    registry: Arc<dyn ThemeRegistry>,
}

impl ThemeService {
    pub async fn new(
        fs: Arc<dyn FileSystem>,
        registry: Arc<dyn ThemeRegistry>,

        // HACK: the paths are temporarily hardcoded here, later they will need
        // to be retrieved either from the app delegate or in some other dynamic way.
        // Task: https://mossland.atlassian.net/browse/SAPIC-546
        application_dir: PathBuf,
    ) -> joinerror::Result<Self> {
        Ok(Self {
            registry,
            loader: ThemeLoader::new(fs, application_dir.join("policies/theme.rego")),
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
                        order: None,
                        source: item.path,
                        is_default: None,
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

        let css = moss_theme::convert(&theme).await?;

        Ok(css)
    }
}
