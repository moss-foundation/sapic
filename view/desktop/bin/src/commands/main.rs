use joinerror::{OptionExt, ResultExt};
use moss_applib::errors::Unavailable;
use sapic_ipc::{
    TauriResult,
    contracts::{
        main::{OpenInTarget, workspace::*},
        other::CancelRequestInput,
    },
};
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
) -> TauriResult<()> {
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
) -> TauriResult<UpdateWorkspaceOutput> {
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
) -> TauriResult<CreateWorkspaceOutput> {
    super::with_main_window_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, app, app_delegate, window| async move {
            let output = window.create_workspace(&ctx, &input).await?;

            if matches!(input.open_on_creation, Some(OpenInTarget::CurrentWindow)) {
                app.ensure_main_for_workspace(&ctx, &app_delegate, output.id.clone())
                    .await
                    .join_err::<()>("failed to open a new window for workspace")?;
            }

            Ok(output)
        },
    )
    .await
}
