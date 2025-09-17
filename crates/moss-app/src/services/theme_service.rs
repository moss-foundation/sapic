use joinerror::{OptionExt, ResultExt};
use moss_applib::errors::{Internal, NotFound};
use moss_fs::{FileSystem, FsResultExt};
use moss_theme::conversion::convert_theme_json_to_css;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use crate::models::{primitives::ThemeId, types::ColorThemeInfo};

const THEMES_REGISTRY_FILE: &str = "themes.json";

struct ServiceState {
    themes: HashMap<ThemeId, ColorThemeInfo>,
    default_theme: ColorThemeInfo,
}

pub struct ThemeService {
    themes_dir: PathBuf,
    fs: Arc<dyn FileSystem>,
    state: RwLock<ServiceState>,
}

impl ThemeService {
    pub async fn new(fs: Arc<dyn FileSystem>, themes_dir: PathBuf) -> joinerror::Result<Self> {
        let rdr = fs.open_file(&themes_dir.join(THEMES_REGISTRY_FILE)).await?;

        let parsed: Vec<ColorThemeInfo> = serde_json::from_reader(rdr)?;
        let themes = parsed
            .into_iter()
            .map(|item| (item.identifier.clone(), item))
            .collect::<HashMap<ThemeId, ColorThemeInfo>>();

        let default_theme = if let Some(theme) = themes
            .values()
            .find(|theme| theme.is_default.unwrap_or(false))
            .cloned()
        {
            theme
        } else {
            themes
                .values()
                .next() // We take the first theme as the default theme if no default theme is found
                .ok_or_join_err::<()>("the app must have at least one theme")?
                .clone()
        };

        Ok(Self {
            themes_dir,
            fs,
            state: RwLock::new(ServiceState {
                themes,
                default_theme,
            }),
        })
    }

    pub async fn default_theme(&self) -> ColorThemeInfo {
        let state = self.state.read().await;
        state.default_theme.clone()
    }

    pub async fn themes(&self) -> HashMap<ThemeId, ColorThemeInfo> {
        let state = self.state.read().await;
        state.themes.clone()
    }

    pub async fn read(&self, id: &ThemeId) -> joinerror::Result<String> {
        let state = self.state.read().await;
        let theme = state
            .themes
            .get(id)
            .ok_or_join_err_with::<NotFound>(|| format!("theme with id `{}` not found", id))?;

        let mut rdr = self
            .fs
            .open_file(&self.themes_dir.join(theme.source.clone()))
            .await
            .join_err_with::<Internal>(|| {
                format!("failed to open theme file `{}`", theme.source.display())
            })?;

        let mut buf = String::new();
        rdr.read_to_string(&mut buf)
            .join_err::<Internal>("failed to read theme file")?;

        Ok(convert_theme_json_to_css(&buf)?)
    }
}
