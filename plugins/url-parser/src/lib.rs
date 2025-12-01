use tauri::{
    AppHandle, Runtime,
    plugin::{Builder, TauriPlugin},
};
use tracing::instrument;

use crate::{
    models::operations::{ParseUrlInput, ParseUrlOutput},
    parser::UrlParser,
};
mod parser;

pub mod models;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("url-parser")
        .invoke_handler(tauri::generate_handler![parse_url])
        .build()
}

#[tauri::command(async)]
#[instrument(level = "trace")]
async fn parse_url<'a, R: Runtime>(
    #[allow(unused)] app_handle: AppHandle<R>,
    input: ParseUrlInput,
) -> joinerror::Result<ParseUrlOutput> {
    let parsed_url = UrlParser::parse_url(&input.url)?;

    Ok(ParseUrlOutput(parsed_url))
}
