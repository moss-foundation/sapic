pub mod handlers;

use anyhow::anyhow;
use moss_api::{self as api, TauriError, TauriResult};
use moss_app::{
    app::App,
    command::CommandContext,
    context::AppContext,
    models::{events::*, operations::*},
};
use moss_text::{ReadOnlyStr, quote};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use tauri::{Emitter, EventTarget, Manager, Runtime as TauriRuntime, State, Window};

use crate::commands::Options;

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn set_color_theme<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    input: SetColorThemeInput,
    options: Options,
) -> TauriResult<()> {
    api::with_timeout(options, async move {
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
    options: Options,
) -> TauriResult<GetColorThemeOutput> {
    api::with_timeout(options, async move {
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
    options: Options,
) -> TauriResult<ListColorThemesOutput> {
    api::with_timeout(options, async move {
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
    options: Options,
) -> TauriResult<DescribeAppStateOutput> {
    api::with_timeout(options, async move {
        app.describe_app_state()
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
    options: Options,
) -> TauriResult<()> {
    api::with_timeout(options, async move {
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
    options: Options,
) -> TauriResult<ListLocalesOutput> {
    api::with_timeout(options, async move {
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
    options: Options,
) -> TauriResult<GetTranslationsOutput> {
    api::with_timeout(options, async move {
        app.get_translations(&input)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn create_workspace<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    input: CreateWorkspaceInput,
    options: Options,
) -> TauriResult<CreateWorkspaceOutput> {
    api::with_timeout(options, async move {
        let ctx = AppContext::from(&app);
        app.create_workspace(&ctx, &input)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn close_workspace<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    input: CloseWorkspaceInput,
    options: Options,
) -> TauriResult<CloseWorkspaceOutput> {
    api::with_timeout(options, async move {
        let ctx = AppContext::from(&app);
        app.close_workspace(&ctx, &input)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn list_workspaces<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    options: Options,
) -> TauriResult<ListWorkspacesOutput> {
    api::with_timeout(options, async move {
        let ctx = AppContext::from(&app);
        app.list_workspaces(&ctx)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn delete_workspace<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    input: DeleteWorkspaceInput,
    options: Options,
) -> TauriResult<()> {
    api::with_timeout(options, async move {
        let ctx = AppContext::from(&app);
        app.delete_workspace(&ctx, &input)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn open_workspace<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    input: OpenWorkspaceInput,
    options: Options,
) -> TauriResult<OpenWorkspaceOutput> {
    api::with_timeout(options, async move {
        let ctx = AppContext::from(&app);
        app.open_workspace(&ctx, &input)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn update_workspace<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    input: UpdateWorkspaceInput,
    options: Options,
) -> TauriResult<()> {
    api::with_timeout(options, async move {
        let ctx = AppContext::from(&app);
        app.update_workspace(&ctx, &input)
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
    options: Options,
) -> TauriResult<JsonValue> {
    api::with_timeout(options, async move {
        let app_handle = app.handle();
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
