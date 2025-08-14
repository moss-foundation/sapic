use anyhow::anyhow;
use moss_api::{TauriError, TauriResult};
use tauri::Window;

use crate::commands::primitives::App;

// TODO: Replace this with fetching the api key from the server
#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn get_mistral_api_key<'a, R: tauri::Runtime>(
    _app: App<'a, R>,
    window: Window<R>,
) -> TauriResult<String> {
    let api_key = dotenv::var("MISTRAL_API_KEY")
        .map_err(|_| TauriError::Other(anyhow!("MISTRAL_API_KEY not set")))?;
    Ok(api_key)
}
