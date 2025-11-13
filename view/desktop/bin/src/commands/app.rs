use joinerror::Error;
use moss_applib::errors::NotFound;
use moss_text::{ReadOnlyStr, quote};
use sapic_app::command::CommandContext;
use sapic_ipc::TauriResult;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use tauri::Window as TauriWindow;

use crate::commands::primitives::*;

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn execute_command<'a, R: tauri::Runtime>(
    app: App<'a, R>,
    window: TauriWindow<R>,
    cmd: ReadOnlyStr,
    args: HashMap<String, JsonValue>,
    options: Options,
) -> TauriResult<JsonValue> {
    let command_cb = app.command(&cmd).ok_or_else(|| {
        Error::new::<NotFound>(format!("command with id {} is not found", quote!(cmd)))
    })?;

    Ok(command_cb(&mut CommandContext::new(window, args)).await?)
}
