use std::path::PathBuf;

use moss_nls::models::types::LocaleDescriptor;
use moss_theme::models::types::{ThemeDescriptor, ThemeMode};
use parking_lot::RwLock;

pub struct AppPreferences {
    pub theme: RwLock<Option<ThemeDescriptor>>,
    pub locale: RwLock<Option<LocaleDescriptor>>,
}

pub struct AppDefaults {
    pub theme: ThemeDescriptor,
    pub locale: LocaleDescriptor,
}

pub struct AppStateManager {
    pub preferences: AppPreferences,
    pub defaults: AppDefaults,
}

impl AppStateManager {
    pub fn new(themes_dir: &PathBuf) -> Self {
        Self {
            preferences: AppPreferences {
                theme: RwLock::new(None),
                locale: RwLock::new(None),
            },
            defaults: AppDefaults {
                theme: ThemeDescriptor {
                    identifier: "moss.sapic-theme.lightDefault".to_string(),
                    display_name: "Light Default".to_string(),
                    order: Some(1),
                    mode: ThemeMode::Light,
                    source: themes_dir.join("light.css"),
                },
                locale: LocaleDescriptor {
                    identifier: "moss.sapic-locale.en".to_string(),
                    code: "en".to_string(),
                    display_name: "English".to_string(),
                    direction: Some("ltr".to_string()),
                },
            },
        }
    }

    pub fn set_color_theme(&self, theme_descriptor: ThemeDescriptor) {
        let mut theme_descriptor_lock = self.preferences.theme.write();
        *theme_descriptor_lock = Some(theme_descriptor);
    }

    pub fn set_language_pack(&self, locale_descriptor: LocaleDescriptor) {
        let mut locale_lock = self.preferences.locale.write();
        *locale_lock = Some(locale_descriptor);
    }
}
