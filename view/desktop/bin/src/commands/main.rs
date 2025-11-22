use joinerror::OptionExt;
use moss_applib::errors::Unavailable;
use sapic_ipc::{
    TauriResult,
    contracts::{main::workspace::*, other::CancelRequestInput},
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
        |ctx, _, window| async move { window.update_workspace(&ctx, &input).await },
    )
    .await
}
