use crate::service::StateService;
use anyhow::Result;
use moss_tauri::TauriResult;
use moss_text::ReadOnlyStr;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tauri::{AppHandle, Window};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandContextError {
    #[error("Argument '{key}' is not found")]
    ArgNotFound { key: String },

    #[error("Failed to deserialize argument '{key}': {source}")]
    DeserializationError {
        key: String,
        #[source]
        source: serde_json::Error,
    },
}

impl From<CommandContextError> for String {
    fn from(err: CommandContextError) -> Self {
        err.to_string()
    }
}

pub struct CommandContext {
    app_handle: AppHandle,
    window: Window,
    args: HashMap<String, Value>,
}

impl CommandContext {
    pub fn new(app_handle: AppHandle, window: Window, args: HashMap<String, Value>) -> Self {
        Self {
            app_handle,
            window,
            args,
        }
    }

    pub fn app_handle(&self) -> &AppHandle {
        &self.app_handle
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn take_arg<T>(&mut self, key: &str) -> Result<T, CommandContextError>
    where
        T: DeserializeOwned,
    {
        let value = self
            .args
            .remove(key)
            .ok_or(CommandContextError::ArgNotFound {
                key: key.to_string(),
            })?;

        serde_json::from_value(value).map_err(|e| CommandContextError::DeserializationError {
            key: key.to_string(),
            source: e,
        })
    }

    pub fn get_arg<T>(&self, key: &str) -> Result<T, CommandContextError>
    where
        T: DeserializeOwned,
    {
        let value = self.args.get(key).ok_or(CommandContextError::ArgNotFound {
            key: key.to_string(),
        })?;

        serde_json::from_value(value.clone()).map_err(|e| {
            CommandContextError::DeserializationError {
                key: key.to_string(),
                source: e,
            }
        })
    }
}

#[macro_export]
macro_rules! command {
    ($name:expr, $callback:expr) => {
        CommandDecl::new(read_only_str!($name), |ctx, state| {
            Box::pin(async move {
                let value = $callback(ctx, state).await?;
                Ok(serde_json::to_value(value)?)
            })
        })
    };
}

type CommandResult<'a> = Pin<Box<dyn Future<Output = TauriResult<Value>> + Send + 'a>>;

pub type CommandCallback = Arc<
    dyn for<'a> Fn(&'a mut CommandContext, &'a StateService) -> CommandResult<'a> + Send + Sync,
>;

pub struct CommandDecl {
    pub name: ReadOnlyStr,
    pub callback: CommandCallback,
}

impl CommandDecl {
    pub fn new<F>(name: ReadOnlyStr, f: F) -> Self
    where
        F: for<'a> Fn(&'a mut CommandContext, &'a StateService) -> CommandResult<'a>
            + Send
            + Sync
            + 'static,
    {
        Self {
            name,
            callback: Arc::new(f),
        }
    }
}
