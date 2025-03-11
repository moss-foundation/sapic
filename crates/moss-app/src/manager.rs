use std::sync::Arc;
use anyhow::Result;
use dashmap::DashMap;
use tauri::AppHandle;
use moss_text::ReadOnlyStr;
use crate::command::{CommandDecl, CommandHandler};
use super::service::{AppService, InstantiationType, ServiceCollection, ServiceHandle};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MockVault {}

    
pub struct AppManager {
    services: ServiceCollection,
    commands: DashMap<ReadOnlyStr, CommandHandler>
    // TODO: Registry
}

unsafe impl Send for AppManager {}
unsafe impl Sync for AppManager {}

impl AppManager {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            services: ServiceCollection::new(app_handle),
            commands: DashMap::new()
        }
    }

    pub fn with_service<T, F>(self, service: F, activation_type: InstantiationType) -> Self
    where
        T: AppService + 'static,
        F: FnOnce(&AppHandle) -> T + 'static,
    {
        self.services.register(service, activation_type);
        self
    }

    pub fn service<T: AppService>(&self) -> Result<ServiceHandle<T>> {
        self.services.get()
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
