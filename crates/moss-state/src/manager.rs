use dashmap::DashMap;
use moss_nls::models::types::LocaleDescriptor;
use moss_text::ReadOnlyStr;
use moss_theme::models::types::{ThemeDescriptor, ThemeMode};
use parking_lot::RwLock;
use std::path::PathBuf;
use std::sync::Arc;

use crate::command::{CommandCallback, CommandDecl};

pub struct AppPreferences {
    pub theme: RwLock<Option<ThemeDescriptor>>,
    pub locale: RwLock<Option<LocaleDescriptor>>,
}

pub struct AppDefaults {
    pub theme: ThemeDescriptor,
    pub locale: LocaleDescriptor,
}

pub struct AppStateManager {
    commands: DashMap<ReadOnlyStr, CommandCallback>,
    preferences: AppPreferences,
    defaults: AppDefaults,
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
            commands: DashMap::new(),
        }
    }

    pub fn preferences(&self) -> &AppPreferences {
        &self.preferences
    }

    pub fn defaults(&self) -> &AppDefaults {
        &self.defaults
    }

    pub fn set_color_theme(&self, theme_descriptor: ThemeDescriptor) {
        let mut theme_descriptor_lock = self.preferences.theme.write();
        *theme_descriptor_lock = Some(theme_descriptor);
    }

    pub fn set_language_pack(&self, locale_descriptor: LocaleDescriptor) {
        let mut locale_lock = self.preferences.locale.write();
        *locale_lock = Some(locale_descriptor);
    }

    pub fn with_commands(self, decls: impl IntoIterator<Item = CommandDecl>) -> Self {
        let commands = DashMap::new();
        for decl in decls {
            commands.insert(decl.name, decl.callback as CommandCallback);
        }
        Self { commands, ..self }
    }

    pub fn get_command(&self, id: &ReadOnlyStr) -> Option<CommandCallback> {
        self.commands.get(id).map(|cmd| Arc::clone(&cmd))
    }
}
