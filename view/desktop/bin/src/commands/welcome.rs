use joinerror::OptionExt;
use moss_applib::errors::Unavailable;
use sapic_ipc::{TauriResult, contracts::other::CancelRequestInput};
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
        .main_window(window.label())
        .await
        .ok_or_join_err_with::<Unavailable>(|| format!("main window is unavailable"))?;

    window.cancel_request(input).await.map_err(|e| e.into())
}
