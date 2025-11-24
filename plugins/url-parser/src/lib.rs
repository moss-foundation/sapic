use pest::Parser;
use sapic_ipc::TauriResult;
use tauri::{
    Runtime,
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
#[instrument(level = "trace", skip(app_handle))]
async fn parse_url<'a, R: Runtime>(input: ParseUrlInput) -> TauriResult<ParseUrlOutput> {
    let parsed_url = UrlParser::parse_url(&input.url)?;

    Ok(ParseUrlOutput(parsed_url))
}
