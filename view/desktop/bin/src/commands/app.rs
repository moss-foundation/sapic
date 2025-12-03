use joinerror::Error;
use moss_text::{ReadOnlyStr, quote};
use sapic_app::command::CommandContext;
use sapic_base::errors::NotFound;
use sapic_ipc::contracts::{configuration::*, extension::*, language::*, theme::*, workspace::*};
use sapic_window::{
    constants::ON_DID_ADD_EXTENSION_CHANNEL, models::events::OnDidAddExtensionForFrontend,
};
use serde_json::Value as JsonValue;
use std::{collections::HashMap, io::ErrorKind};
use tauri::{Emitter, Window as TauriWindow};

use crate::commands::primitives::*;

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn execute_command<'a, R: tauri::Runtime>(
    app: App<'a, R>,
    window: TauriWindow<R>,
    cmd: ReadOnlyStr,
    args: HashMap<String, JsonValue>,
    options: Options,
) -> joinerror::Result<JsonValue> {
    let command_cb = app.command(&cmd).ok_or_else(|| {
        Error::new::<NotFound>(format!("command with id {} is not found", quote!(cmd)))
    })?;

    Ok(command_cb(&mut CommandContext::new(window, args)).await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn list_configuration_schemas<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: TauriWindow<R>,
    options: Options,
) -> joinerror::Result<ListConfigurationSchemasOutput> {
    super::with_app_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, app, app_delegate| async move {
            app.list_configuration_schemas(&ctx, &app_delegate).await
        },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn list_extensions<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: TauriWindow<R>,
    options: Options,
) -> joinerror::Result<ListExtensionsOutput> {
    super::with_app_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, app, _| async move { app.list_extensions(&ctx).await },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn download_extension<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: TauriWindow<R>,
    options: Options,
    input: DownloadExtensionInput,
) -> joinerror::Result<()> {
    super::with_app_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, app, _| async move {
            let id = app
                .download_extension(&ctx, &input.extension_id, &input.version)
                .await?;
            // Is this the right place to emit event?
            app.emit(
                ON_DID_ADD_EXTENSION_CHANNEL,
                OnDidAddExtensionForFrontend { id },
            )
            .map_err(|e| {
                std::io::Error::new(
                    ErrorKind::Other,
                    format!("Unable to emit a tauri event: {}", e),
                )
            })?;
            Ok(())
        },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx,app), fields(window = window.label()))]
pub async fn describe_color_theme<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: TauriWindow<R>,
    input: GetColorThemeInput,
    options: Options,
) -> joinerror::Result<GetColorThemeOutput> {
    super::with_app_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, app, _| async move { app.get_color_theme(&ctx, &input).await },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn list_color_themes<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: TauriWindow<R>,
    options: Options,
) -> joinerror::Result<ListColorThemesOutput> {
    super::with_app_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, app, _| async move { app.list_color_themes(&ctx).await },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn list_workspaces<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: TauriWindow<R>,
    options: Options,
) -> joinerror::Result<ListWorkspacesOutput> {
    super::with_app_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, app, app_delegate| async move { app.list_workspaces(&ctx, &app_delegate).await },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn delete_workspace<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: TauriWindow<R>,
    input: DeleteWorkspaceInput,
    options: Options,
) -> joinerror::Result<DeleteWorkspaceOutput> {
    super::with_app_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, app, app_delegate| async move {
            let output = app.delete_workspace(&ctx, &input).await?;

            app.ensure_welcome(&app_delegate).await?;

            Ok(output)
        },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn list_languages<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: TauriWindow<R>,
    options: Options,
) -> joinerror::Result<ListLanguagesOutput> {
    super::with_app_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, app, _| async move { app.list_languages(&ctx).await },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn get_translation_namespace<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: TauriWindow<R>,
    input: GetTranslationNamespaceInput,
    options: Options,
) -> joinerror::Result<GetTranslationNamespaceOutput> {
    super::with_app_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, app, _| async move { app.get_translation_namespace(&ctx, &input).await },
    )
    .await
}
