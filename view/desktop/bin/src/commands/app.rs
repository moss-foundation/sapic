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

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn set_color_theme<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    input: SetColorThemeInput,
) -> TauriResult<()> {
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

    app.set_color_theme(input).await?;

    Ok(())
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn get_color_theme<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    input: GetColorThemeInput,
) -> TauriResult<GetColorThemeOutput> {
    Ok(app.get_color_theme(input).await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn list_color_themes<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
) -> TauriResult<ListColorThemesOutput> {
    Ok(app.list_color_themes().await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn describe_app_state<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
) -> TauriResult<DescribeAppStateOutput> {
    Ok(app.describe_state().await?)
}

#[tauri::command]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn set_locale<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    input: SetLocaleInput,
) -> TauriResult<()> {
    Ok(app.set_locale(input).await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn list_locales<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
) -> TauriResult<ListLocalesOutput> {
    Ok(app.list_locales().await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn get_translations<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    input: GetTranslationsInput,
) -> TauriResult<GetTranslationsOutput> {
    Ok(app.get_translations(input).await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn execute_command<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    cmd: ReadOnlyStr,
    args: HashMap<String, JsonValue>,
) -> TauriResult<JsonValue> {
    let app_handle = app.app_handle();
    match app.command(&cmd) {
        Some(command_handler) => {
            command_handler(&mut CommandContext::new(app_handle.clone(), window, args)).await
        }
        _ => Err(TauriError(format!(
            "command with id {} is not found",
            quote!(cmd)
        ))),
    }
}
