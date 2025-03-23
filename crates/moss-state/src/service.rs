use dashmap::DashMap;
use moss_app::service::prelude::AppService;
use moss_nls::models::types::LocaleDescriptor;
use moss_text::ReadOnlyStr;
use moss_theme::models::types::ColorThemeDescriptor;
use parking_lot::RwLock;
use std::sync::Arc;

use crate::command::{CommandCallback, CommandDecl};

pub struct AppPreferences {
    pub theme: RwLock<Option<ColorThemeDescriptor>>,
    pub locale: RwLock<Option<LocaleDescriptor>>,
}

pub struct AppDefaults {
    pub theme: ColorThemeDescriptor,
    pub locale: LocaleDescriptor,
}

pub struct StateService {
    commands: DashMap<ReadOnlyStr, CommandCallback>,
    preferences: AppPreferences,
    defaults: AppDefaults,
}

impl StateService {
    pub fn new(defaults: AppDefaults) -> Self {
        Self {
            preferences: AppPreferences {
                theme: RwLock::new(None),
                locale: RwLock::new(None),
            },
            defaults,
            commands: DashMap::new(),
        }
    }

    pub fn preferences(&self) -> &AppPreferences {
        &self.preferences
    }

    pub fn defaults(&self) -> &AppDefaults {
        &self.defaults
    }

    pub fn set_color_theme(&self, theme_descriptor: ColorThemeDescriptor) {
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

impl AppService for StateService {}
