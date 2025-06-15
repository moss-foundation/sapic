use anyhow::anyhow;
use moss_app::{
    app::App,
    command::CommandContext,
    models::{
        events::ColorThemeChangeEventPayload,
        operations::{
            DescribeAppStateOutput, GetColorThemeInput, GetColorThemeOutput, GetTranslationsInput,
            GetTranslationsOutput, ListColorThemesOutput, ListLocalesOutput, SetColorThemeInput,
            SetLocaleInput,
        },
    },
};
use moss_tauri::{TauriError, TauriResult};
use moss_text::{ReadOnlyStr, quote};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use tauri::{Emitter, EventTarget, Manager, Runtime as TauriRuntime, State, Window};

use crate::constants::DEFAULT_COMMAND_TIMEOUT;

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn set_color_theme<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    input: SetColorThemeInput,
) -> TauriResult<()> {
    tokio::time::timeout(DEFAULT_COMMAND_TIMEOUT, async move {
        for (label, _) in app.webview_windows() {
            if window.label() == &label {
                continue;
            }

            app.emit_to(
                EventTarget::webview_window(&label),
                "core://color-theme-changed",
                ColorThemeChangeEventPayload::new(&input.theme_info.identifier),
            )
            .map_err(|err| anyhow!("Failed to emit event to webview '{}': {}", label, err))?;
        }

        app.set_color_theme(&input)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn get_color_theme<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    input: GetColorThemeInput,
) -> TauriResult<GetColorThemeOutput> {
    tokio::time::timeout(DEFAULT_COMMAND_TIMEOUT, async move {
        app.get_color_theme(&input)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn list_color_themes<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
) -> TauriResult<ListColorThemesOutput> {
    tokio::time::timeout(DEFAULT_COMMAND_TIMEOUT, async move {
        app.list_color_themes()
            .await
            .map_err(TauriError::OperationError)
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn describe_app_state<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
) -> TauriResult<DescribeAppStateOutput> {
    tokio::time::timeout(DEFAULT_COMMAND_TIMEOUT, async move {
        app.describe_state()
            .await
            .map_err(TauriError::OperationError)
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}

#[tauri::command]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn set_locale<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    input: SetLocaleInput,
) -> TauriResult<()> {
    tokio::time::timeout(DEFAULT_COMMAND_TIMEOUT, async move {
        app.set_locale(&input)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn list_locales<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
) -> TauriResult<ListLocalesOutput> {
    tokio::time::timeout(DEFAULT_COMMAND_TIMEOUT, async move {
        app.list_locales().await.map_err(TauriError::OperationError)
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn get_translations<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    input: GetTranslationsInput,
) -> TauriResult<GetTranslationsOutput> {
    tokio::time::timeout(DEFAULT_COMMAND_TIMEOUT, async move {
        app.get_translations(&input)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn execute_command<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    cmd: ReadOnlyStr,
    args: HashMap<String, JsonValue>,
) -> TauriResult<JsonValue> {
    tokio::time::timeout(DEFAULT_COMMAND_TIMEOUT, async move {
        let app_handle = app.app_handle();
        match app.command(&cmd) {
            Some(command_handler) => {
                command_handler(&mut CommandContext::new(app_handle.clone(), window, args)).await
            }
            _ => Err(TauriError::Other(anyhow!(
                "command with id {} is not found",
                quote!(cmd)
            ))),
        }
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}
