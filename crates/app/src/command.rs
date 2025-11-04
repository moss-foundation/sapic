use joinerror::{OptionExt, ResultExt, errors};
use moss_text::ReadOnlyStr;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};
use tauri::{Runtime as TauriRuntime, Window};

errors! {
    /// The argument is not found.
    ArgNotFound => "arg_not_found",

    /// The argument is not deserializable.
    Deserialization => "deserialization",
}

pub struct CommandContext<R: TauriRuntime> {
    window: Window<R>,
    args: HashMap<String, Value>,
}

impl<R: TauriRuntime> CommandContext<R> {
    pub fn new(window: Window<R>, args: HashMap<String, Value>) -> Self {
        Self { window, args }
    }

    pub fn window(&self) -> &Window<R> {
        &self.window
    }

    pub fn take_arg<T>(&mut self, key: &str) -> joinerror::Result<T>
    where
        T: DeserializeOwned,
    {
        let value = self
            .args
            .remove(key)
            .ok_or_join_err_with::<ArgNotFound>(|| format!("argument '{}' is not found", key))?;

        serde_json::from_value(value).join_err_with::<Deserialization>(|| {
            format!("failed to deserialize argument '{}'", key)
        })
    }

    pub fn get_arg<T>(&self, key: &str) -> joinerror::Result<T>
    where
        T: DeserializeOwned,
    {
        let value = self
            .args
            .get(key)
            .ok_or_join_err_with::<ArgNotFound>(|| format!("argument '{}' is not found", key))?;

        serde_json::from_value(value.clone()).join_err_with::<Deserialization>(|| {
            format!("failed to deserialize argument '{}'", key)
        })
    }
}

#[macro_export]
macro_rules! command {
    ($name:expr, $callback:expr) => {
        $crate::command::CommandDecl::new(read_only_str!($name), |ctx| {
            Box::pin(async move {
                let value = $callback(ctx).await?;
                Ok(serde_json::to_value(value)?)
            })
        })
    };
}

type CommandResult<'a> = Pin<Box<dyn Future<Output = joinerror::Result<Value>> + Send + 'a>>;

pub type CommandCallback<R> =
    Arc<dyn for<'a> Fn(&'a mut CommandContext<R>) -> CommandResult<'a> + Send + Sync>;

pub struct CommandDecl<R: TauriRuntime> {
    pub name: ReadOnlyStr,
    pub callback: CommandCallback<R>,
}

impl<R: TauriRuntime> CommandDecl<R> {
    pub fn new<F>(name: ReadOnlyStr, f: F) -> Self
    where
        F: for<'a> Fn(&'a mut CommandContext<R>) -> CommandResult<'a> + Send + Sync + 'static,
    {
        Self {
            name,
            callback: Arc::new(f),
        }
    }
}
