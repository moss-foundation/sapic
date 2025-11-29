use sapic_window::models::operations::*;
use tauri::Window as TauriWindow;

use crate::commands::primitives::*;

// DEPRECATED
#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn describe_app<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: TauriWindow<R>,
    options: Options,
) -> joinerror::Result<DescribeAppOutput> {
    super::with_main_window_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, app_delegate, window| async move {
            window.inner().describe_app(&ctx, &app_delegate).await
        },
    )
    .await
}

// DEPRECATED
#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn update_configuration<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: TauriWindow<R>,
    input: UpdateConfigurationInput,
    options: Options,
) -> joinerror::Result<()> {
    super::with_main_window_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, app_delegate, window| async move {
            window
                .inner()
                .update_configuration(&ctx, &app_delegate, input)
                .await
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
    super::with_main_window_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, _, window| async move { window.inner().list_languages(&ctx).await },
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
    super::with_main_window_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, _, window| async move { window.inner().get_translation_namespace(&ctx, &input).await },
    )
    .await
}

#[allow(dead_code)]
#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn create_profile<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: TauriWindow<R>,
    input: CreateProfileInput,
    options: Options,
) -> joinerror::Result<CreateProfileOutput> {
    super::with_main_window_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, app_delegate, window| async move {
            window
                .inner()
                .create_profile(&ctx, &app_delegate, input)
                .await
        },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn update_profile<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: TauriWindow<R>,
    input: UpdateProfileInput,
    options: Options,
) -> joinerror::Result<UpdateProfileOutput> {
    super::with_main_window_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, app_delegate, window| async move {
            window
                .inner()
                .update_profile(&ctx, &app_delegate, input)
                .await
        },
    )
    .await
}

// TODO: Replace this with fetching the api key from the server
#[tauri::command(async)]
#[instrument(level = "trace", fields(window = window.label()))]
pub async fn get_mistral_api_key<'a, R: tauri::Runtime>(
    window: TauriWindow<R>,
) -> joinerror::Result<String> {
    let api_key =
        dotenvy::var("MISTRAL_API_KEY").map_err(|_| anyhow::anyhow!("MISTRAL_API_KEY not set"))?;
    Ok(api_key)
}
