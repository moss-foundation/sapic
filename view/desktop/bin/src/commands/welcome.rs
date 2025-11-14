use joinerror::OptionExt;
use moss_applib::errors::Unavailable;
use sapic_ipc::{TauriResult, contracts::other::CancelRequestInput};
use sapic_window::models::operations::OpenWorkspaceInput;
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
) -> TauriResult<()> {
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
) -> TauriResult<()> {
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
