use joinerror::{OptionExt, ResultExt};
use sapic_ipc::contracts::{other::*, welcome::workspace::*};
use sapic_runtime::errors::Unavailable;
use tauri::Window as TauriWindow;

use crate::commands::primitives::*;

#[allow(non_snake_case)]
#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn welcome__cancel_request<'a, R: tauri::Runtime>(
    app: App<'a, R>,
    window: TauriWindow<R>,
    input: CancelRequestInput,
    options: Options,
) -> joinerror::Result<()> {
    let window = app
        .welcome_window()
        .await
        .ok_or_join_err_with::<Unavailable>(|| format!("welcome window is unavailable"))?;

    window.cancel_request(input).await.map_err(|e| e.into())
}

#[allow(non_snake_case)]
#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn welcome__open_workspace<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: TauriWindow<R>,
    input: OpenWorkspaceInput,
    options: Options,
) -> joinerror::Result<()> {
    super::with_welcome_window_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, app, app_delegate, _| async move {
            app.ensure_main_for_workspace(&ctx, &app_delegate, input.id.clone())
                .await
                .unwrap();

            if let Err(err) = app.close_welcome_window().await {
                tracing::error!("Failed to close welcome window: {}", err);
            }

            Ok(())
        },
    )
    .await
}

#[allow(non_snake_case)]
#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn welcome__create_workspace<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: TauriWindow<R>,
    input: CreateWorkspaceInput,
    options: Options,
) -> joinerror::Result<CreateWorkspaceOutput> {
    super::with_welcome_window_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, app, app_delegate, window| async move {
            let output = window.create_workspace(&ctx, &input).await?;

            app.ensure_main_for_workspace(&ctx, &app_delegate, output.id.clone())
                .await
                .join_err::<()>("failed to open a new window for workspace")?;

            Ok(output)
        },
    )
    .await
}

#[allow(non_snake_case)]
#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn welcome__update_workspace<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: TauriWindow<R>,
    input: UpdateWorkspaceInput,
    options: Options,
) -> joinerror::Result<UpdateWorkspaceOutput> {
    super::with_welcome_window_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, _, window| async move { window.update_workspace(&ctx, &input).await },
    )
    .await
}
