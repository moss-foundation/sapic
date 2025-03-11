use std::path::PathBuf;
use std::sync::Arc;
use dashmap::DashMap;
use moss_nls::models::types::LocaleDescriptor;
use moss_theme::models::types::{ThemeDescriptor, ThemeMode};
use parking_lot::RwLock;
use moss_text::ReadOnlyStr;
use crate::command::{CommandDecl, CommandHandler};

pub struct AppPreferences {
    pub theme: RwLock<Option<ThemeDescriptor>>,
    pub locale: RwLock<Option<LocaleDescriptor>>,
}

pub struct AppDefaults {
    pub theme: ThemeDescriptor,
    pub locale: LocaleDescriptor,
}

pub struct AppStateManager {
    commands: DashMap<ReadOnlyStr, CommandHandler>,
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
            commands: DashMap::new()
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

    pub fn with_commands(self, decls: impl IntoIterator<Item = CommandDecl>) -> Self {
        let mut commands = DashMap::new();
        for decl in decls {
            commands.insert(decl.name, Arc::new(decl.callback) as CommandHandler);
        }
        Self {
            commands,
            ..self
        }
    }

    pub fn get_command(&self, id: &ReadOnlyStr) -> Option<CommandHandler> {
        self.commands
            .get(id)
            .map(|cmd| Arc::clone(&cmd))
    }
}
