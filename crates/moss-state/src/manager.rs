use std::path::PathBuf;

use moss_theme::models::types::ThemeDescriptor;
use parking_lot::RwLock;

pub struct AppPreferences {
    pub theme: RwLock<Option<ThemeDescriptor>>,
    // pub locale: RwLock<Option<LocaleDescriptor>>,
}

pub struct AppDefaults {
    pub theme: ThemeDescriptor,
    // pub locale: LocaleDescriptor,
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
                // locale: RwLock::new(None),
            },
            defaults: AppDefaults {
                theme: ThemeDescriptor {
                    identifier: "moss.sapic-light-default".to_string(),
                    display_name: "Light Default".to_string(),
                    order: 1,
                    source: themes_dir.join("light.css"),
                },
                // locale: LocaleDescriptor {
                //     code: "en".to_string(),
                //     name: "English".to_string(),
                //     direction: Some("ltr".to_string()),
                // },
            },
        }
    }

    pub fn set_color_theme(&self, theme_descriptor: ThemeDescriptor) {
        let mut theme_descriptor_lock = self.preferences.theme.write();
        *theme_descriptor_lock = Some(theme_descriptor);
    }
}
