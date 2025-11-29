use joinerror::{OptionExt, ResultExt};
use sapic_ipc::contracts::{
    main::{OpenInTarget, workspace::*},
    other::CancelRequestInput,
};
use sapic_runtime::errors::Unavailable;
use tauri::Window as TauriWindow;

use crate::commands::primitives::*;

#[allow(non_snake_case)]
#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn main__cancel_request<'a, R: tauri::Runtime>(
    app: App<'a, R>,
    window: TauriWindow<R>,
    input: CancelRequestInput,
    options: Options,
) -> joinerror::Result<()> {
    let window = app
        .main_window(window.label())
        .await
        .ok_or_join_err_with::<Unavailable>(|| format!("main window is unavailable"))?;

    window.cancel_request(input).await.map_err(|e| e.into())
}

#[allow(non_snake_case)]
#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn main__update_workspace<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: TauriWindow<R>,
    input: UpdateWorkspaceInput,
    options: Options,
) -> joinerror::Result<UpdateWorkspaceOutput> {
    super::with_main_window_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, _, window| async move { window.update_workspace(&ctx, &input).await },
    )
    .await
}

#[allow(non_snake_case)]
#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn main__create_workspace<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: TauriWindow<R>,
    input: CreateWorkspaceInput,
    options: Options,
) -> joinerror::Result<CreateWorkspaceOutput> {
    super::with_main_window_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, app, app_delegate, window| async move {
            let output = window.create_workspace(&ctx, &input).await?;

            match input.open_on_creation {
                OpenInTarget::NewWindow => {
                    app.ensure_main_for_workspace(&ctx, &app_delegate, output.id.clone())
                        .await
                        .join_err::<()>("failed to open a new window for workspace")?;
                }
                OpenInTarget::CurrentWindow => {
                    app.swap_main_window_workspace(
                        &ctx,
                        &app_delegate,
                        output.id.clone(),
                        window.label(),
                    )
                    .await
                    .join_err::<()>("failed to swap main window workspace")?;
                }
            }

            Ok(output)
        },
    )
    .await
}

#[allow(non_snake_case)]
#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn main__open_workspace<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: TauriWindow<R>,
    input: OpenWorkspaceInput,
    options: Options,
) -> joinerror::Result<()> {
    super::with_main_window_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, app, app_delegate, window| async move {
            match input.open_in_target {
                OpenInTarget::NewWindow => {
                    app.ensure_main_for_workspace(&ctx, &app_delegate, input.id.clone())
                        .await
                        .join_err::<()>("failed to open a new window for workspace")?;
                }
                OpenInTarget::CurrentWindow => {
                    app.swap_main_window_workspace(
                        &ctx,
                        &app_delegate,
                        input.id.clone(),
                        window.label(),
                    )
                    .await
                    .join_err::<()>("failed to swap main window workspace")?;
                }
            }

            Ok(())
        },
    )
    .await
}

#[allow(non_snake_case)]
#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn main__close_workspace<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: TauriWindow<R>,
    options: Options,
) -> joinerror::Result<()> {
    super::with_main_window_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |_, app, app_delegate, window| async move {
            app.ensure_welcome(&app_delegate)
                .await
                .join_err::<()>("failed to ensure welcome window")?;

            app.close_main_window(window.label())
                .await
                .join_err::<()>("failed to close main window")?;

            Ok(())
        },
    )
    .await
}
