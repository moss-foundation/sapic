use anyhow::anyhow;
use moss_api::{TauriError, TauriResult};
use moss_app::{
    command::CommandContext,
    models::{events::*, operations::*},
};
use moss_applib::context::{AnyContext, MutableContext};
use moss_text::{ReadOnlyStr, quote};
use serde_json::Value as JsonValue;
use std::{collections::HashMap, time::Duration};
use tauri::{Emitter, EventTarget, Manager, Window};

use crate::commands::primitives::*;

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn set_color_theme<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: SetColorThemeInput,
    options: Options,
) -> TauriResult<()> {
    let ctx = {
        let mut ctx =
            MutableContext::new_with_timeout(ctx.inner().clone(), Duration::from_secs(30));
        if let Some(request_id) = options.and_then(|opts| opts.request_id) {
            ctx.with_value("request_id", request_id);
        }

        ctx.freeze()
    };

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

    Ok(app.set_color_theme(&ctx, &input).await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx,app), fields(window = window.label()))]
pub async fn get_color_theme<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: GetColorThemeInput,
    options: Options,
) -> TauriResult<GetColorThemeOutput> {
    let ctx = {
        let mut ctx =
            MutableContext::new_with_timeout(ctx.inner().clone(), Duration::from_secs(30));
        if let Some(request_id) = options.and_then(|opts| opts.request_id) {
            ctx.with_value("request_id", request_id);
        }

        ctx.freeze()
    };

    Ok(app.get_color_theme(&ctx, &input).await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn list_color_themes<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    options: Options,
) -> TauriResult<ListColorThemesOutput> {
    let ctx = {
        let mut ctx =
            MutableContext::new_with_timeout(ctx.inner().clone(), Duration::from_secs(30));
        if let Some(request_id) = options.and_then(|opts| opts.request_id) {
            ctx.with_value("request_id", request_id);
        }

        ctx.freeze()
    };

    Ok(app.list_color_themes(&ctx).await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn describe_app_state<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    options: Options,
) -> TauriResult<DescribeAppStateOutput> {
    let ctx = {
        let mut ctx =
            MutableContext::new_with_timeout(ctx.inner().clone(), Duration::from_secs(30));
        if let Some(request_id) = options.and_then(|opts| opts.request_id) {
            ctx.with_value("request_id", request_id);
        }

        ctx.freeze()
    };

    Ok(app.describe_app_state(&ctx).await?)
}

#[tauri::command]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn set_locale<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: SetLocaleInput,
    options: Options,
) -> TauriResult<()> {
    let ctx = {
        let mut ctx =
            MutableContext::new_with_timeout(ctx.inner().clone(), Duration::from_secs(30));
        if let Some(request_id) = options.and_then(|opts| opts.request_id) {
            ctx.with_value("request_id", request_id);
        }

        ctx.freeze()
    };

    Ok(app.set_locale(&ctx, &input).await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn list_locales<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    options: Options,
) -> TauriResult<ListLocalesOutput> {
    let ctx = {
        let mut ctx =
            MutableContext::new_with_timeout(ctx.inner().clone(), Duration::from_secs(30));
        if let Some(request_id) = options.and_then(|opts| opts.request_id) {
            ctx.with_value("request_id", request_id);
        }

        ctx.freeze()
    };

    Ok(app.list_locales(&ctx).await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn get_translations<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: GetTranslationsInput,
    options: Options,
) -> TauriResult<GetTranslationsOutput> {
    let ctx = {
        let mut ctx =
            MutableContext::new_with_timeout(ctx.inner().clone(), Duration::from_secs(30));
        if let Some(request_id) = options.and_then(|opts| opts.request_id) {
            ctx.with_value("request_id", request_id);
        }

        ctx.freeze()
    };

    Ok(app.get_translations(&ctx, &input).await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn create_workspace<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: CreateWorkspaceInput,
    options: Options,
) -> TauriResult<CreateWorkspaceOutput> {
    let ctx = {
        let mut ctx =
            MutableContext::new_with_timeout(ctx.inner().clone(), Duration::from_secs(30));
        if let Some(request_id) = options.and_then(|opts| opts.request_id) {
            ctx.with_value("request_id", request_id);
        }

        ctx.freeze()
    };

    Ok(app.create_workspace(&ctx, &input).await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn close_workspace<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: CloseWorkspaceInput,
    options: Options,
) -> TauriResult<CloseWorkspaceOutput> {
    let ctx = {
        let mut ctx =
            MutableContext::new_with_timeout(ctx.inner().clone(), Duration::from_secs(30));
        if let Some(request_id) = options.and_then(|opts| opts.request_id) {
            ctx.with_value("request_id", request_id);
        }

        ctx.freeze()
    };

    Ok(app.close_workspace(&ctx, &input).await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn list_workspaces<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    options: Options,
) -> TauriResult<ListWorkspacesOutput> {
    let ctx = {
        let mut ctx =
            MutableContext::new_with_timeout(ctx.inner().clone(), Duration::from_secs(30));
        if let Some(request_id) = options.and_then(|opts| opts.request_id) {
            ctx.with_value("request_id", request_id);
        }

        ctx.freeze()
    };

    Ok(app.list_workspaces(&ctx).await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn delete_workspace<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: DeleteWorkspaceInput,
    options: Options,
) -> TauriResult<()> {
    let ctx = {
        let mut ctx =
            MutableContext::new_with_timeout(ctx.inner().clone(), Duration::from_secs(30));
        if let Some(request_id) = options.and_then(|opts| opts.request_id) {
            ctx.with_value("request_id", request_id);
        }

        ctx.freeze()
    };

    Ok(app.delete_workspace(&ctx, &input).await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn open_workspace<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: OpenWorkspaceInput,
    options: Options,
) -> TauriResult<OpenWorkspaceOutput> {
    let ctx = {
        let mut ctx =
            MutableContext::new_with_timeout(ctx.inner().clone(), Duration::from_secs(30));
        if let Some(request_id) = options.and_then(|opts| opts.request_id) {
            ctx.with_value("request_id", request_id);
        }

        ctx.freeze()
    };

    Ok(app.open_workspace(&ctx, &input).await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn update_workspace<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: UpdateWorkspaceInput,
    options: Options,
) -> TauriResult<()> {
    let ctx = {
        let mut ctx =
            MutableContext::new_with_timeout(ctx.inner().clone(), Duration::from_secs(30));
        if let Some(request_id) = options.and_then(|opts| opts.request_id) {
            ctx.with_value("request_id", request_id);
        }

        ctx.freeze()
    };

    Ok(app.update_workspace(&ctx, &input).await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn execute_command<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    cmd: ReadOnlyStr,
    args: HashMap<String, JsonValue>,
    options: Options,
) -> TauriResult<JsonValue> {
    let _ctx = {
        let mut ctx =
            MutableContext::new_with_timeout(ctx.inner().clone(), Duration::from_secs(30));
        if let Some(request_id) = options.and_then(|opts| opts.request_id) {
            ctx.with_value("request_id", request_id);
        }

        ctx.freeze()
    };

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
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn cancel_request<'a, R: tauri::Runtime>(
    app: App<'a, R>,
    window: Window<R>,
    input: CancelRequestInput,
    options: Options,
) -> TauriResult<()> {
    Ok(app.cancel_request(input).await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn add_account<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: AddAccountInput,
) -> TauriResult<AddAccountOutput> {
    let output = app.add_account(&ctx, input).await?;
    Ok(output)
}
