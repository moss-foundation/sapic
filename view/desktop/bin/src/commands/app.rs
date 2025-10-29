use joinerror::Error;
use moss_api::{TauriError, TauriResult};
use moss_app::{command::CommandContext, models::operations::*};
use moss_applib::errors::NotFound;
use moss_text::{ReadOnlyStr, quote};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use tauri::Window;

use crate::commands::primitives::*;

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn describe_app<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    options: Options,
) -> TauriResult<DescribeAppOutput> {
    super::with_app_timeout(ctx.inner(), app, options, |ctx, _, app| async move {
        app.describe_app(&ctx).await
    })
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn update_configuration<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: UpdateConfigurationInput,
    options: Options,
) -> TauriResult<()> {
    super::with_app_timeout(
        ctx.inner(),
        app,
        options,
        |ctx, app_delegate, app| async move {
            app.update_configuration(&ctx, &app_delegate, input).await
        },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn list_configuration_schemas<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    options: Options,
) -> TauriResult<ListConfigurationSchemasOutput> {
    super::with_app_timeout(ctx.inner(), app, options, |ctx, _, app| async move {
        app.list_configuration_schemas(&ctx).await
    })
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn list_extensions<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    options: Options,
) -> TauriResult<ListExtensionsOutput> {
    super::with_app_timeout(
        ctx.inner(),
        app,
        options,
        |ctx, app_delegate, app| async move { app.list_extensions(&ctx, &app_delegate).await },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx,app), fields(window = window.label()))]
pub async fn describe_color_theme<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: GetColorThemeInput,
    options: Options,
) -> TauriResult<GetColorThemeOutput> {
    super::with_app_timeout(ctx.inner(), app, options, |ctx, _, app| async move {
        app.get_color_theme(&ctx, &input).await
    })
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn list_color_themes<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    options: Options,
) -> TauriResult<ListColorThemesOutput> {
    super::with_app_timeout(ctx.inner(), app, options, |ctx, _, app| async move {
        app.list_color_themes(&ctx).await
    })
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn list_languages<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    options: Options,
) -> TauriResult<ListLanguagesOutput> {
    super::with_app_timeout(ctx.inner(), app, options, |ctx, _, app| async move {
        app.list_languages(&ctx).await
    })
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn get_translation_namespace<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: GetTranslationNamespaceInput,
    options: Options,
) -> TauriResult<GetTranslationNamespaceOutput> {
    super::with_app_timeout(ctx.inner(), app, options, |ctx, _, app| async move {
        app.get_translation_namespace(&ctx, &input).await
    })
    .await
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
    super::with_app_timeout(
        ctx.inner(),
        app,
        options,
        |ctx, app_delegate, app| async move { app.create_workspace(&ctx, &app_delegate, &input).await },
    )
    .await
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
    super::with_app_timeout(ctx.inner(), app, options, |ctx, _, app| async move {
        app.close_workspace(&ctx, &input).await
    })
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn list_workspaces<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    options: Options,
) -> TauriResult<ListWorkspacesOutput> {
    super::with_app_timeout(ctx.inner(), app, options, |ctx, _, app| async move {
        app.list_workspaces(&ctx).await
    })
    .await
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
    super::with_app_timeout(ctx.inner(), app, options, |ctx, _, app| async move {
        app.delete_workspace(&ctx, &input).await
    })
    .await
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
    super::with_app_timeout(
        ctx.inner(),
        app,
        options,
        |ctx, app_delegate, app| async move { app.open_workspace(&ctx, &app_delegate, &input).await },
    )
    .await
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
    super::with_app_timeout(ctx.inner(), app, options, |ctx, _, app| async move {
        app.update_workspace(&ctx, &input).await
    })
    .await
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
    super::with_app_timeout(ctx.inner(), app, options, |_, _, app| async move {
        let command_cb = app.command(&cmd).ok_or_else(|| {
            Error::new::<NotFound>(format!("command with id {} is not found", quote!(cmd)))
        })?;

        command_cb(&mut CommandContext::new(window, args)).await
    })
    .await
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

#[allow(dead_code)]
#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn create_profile<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: CreateProfileInput,
    options: Options,
) -> TauriResult<CreateProfileOutput> {
    super::with_app_timeout(ctx.inner(), app, options, |ctx, app_delegate, app| async move {
        app.create_profile(&ctx, &app_delegate, input).await
    })
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn update_profile<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: UpdateProfileInput,
    options: Options,
) -> TauriResult<UpdateProfileOutput> {
    super::with_app_timeout(
        ctx.inner(),
        app,
        options,
        |ctx, app_delegate, app| async move { app.update_profile(&ctx, &app_delegate, input).await },
    )
    .await
}

// TODO: Replace this with fetching the api key from the server
#[tauri::command(async)]
#[instrument(level = "trace", fields(window = window.label()))]
pub async fn get_mistral_api_key<'a, R: tauri::Runtime>(window: Window<R>) -> TauriResult<String> {
    let api_key = dotenvy::var("MISTRAL_API_KEY")
        .map_err(|_| TauriError::Other(anyhow::anyhow!("MISTRAL_API_KEY not set")))?;
    Ok(api_key)
}
