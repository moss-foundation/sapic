use dashmap::DashMap;
use moss_app::service::prelude::AppService;
use moss_nls::models::types::LocaleInfo;
use moss_text::ReadOnlyStr;
use moss_theme::models::types::ColorThemeInfo;
use parking_lot::RwLock;
use std::sync::Arc;

use crate::{
    command::{CommandCallback, CommandDecl},
    models::operations::{SetColorThemeInput, SetLocaleInput},
};

pub struct AppPreferences {
    pub theme: RwLock<Option<ColorThemeInfo>>,
    pub locale: RwLock<Option<LocaleInfo>>,
}

pub struct AppDefaults {
    pub theme: ColorThemeInfo,
    pub locale: LocaleInfo,
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

    pub fn set_color_theme(&self, input: SetColorThemeInput) {
        let mut theme_descriptor_lock = self.preferences.theme.write();
        *theme_descriptor_lock = Some(input.theme_info);
    }

    pub fn set_locale(&self, input: SetLocaleInput) {
        let mut locale_lock = self.preferences.locale.write();
        *locale_lock = Some(input.locale_info);
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
