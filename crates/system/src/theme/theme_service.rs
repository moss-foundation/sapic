use joinerror::OptionExt;
use sapic_base::theme::types::{ColorThemeInfo, primitives::ThemeId};
use sapic_errors::NotFound;
use std::collections::HashMap;

use super::{DynThemeLoader, DynThemeRegistry};

pub struct ThemeService {
    loader: DynThemeLoader,
    registry: DynThemeRegistry,
}

impl ThemeService {
    pub async fn new(
        registry: DynThemeRegistry,
        loader: DynThemeLoader,
    ) -> joinerror::Result<Self> {
        Ok(Self { registry, loader })
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
