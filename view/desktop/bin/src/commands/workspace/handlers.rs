use anyhow::anyhow;
use axum::{Json, extract::State};
use moss_api::{TauriError, TauriResult};
use moss_app::App;
use moss_workspace::models::operations::{CreateCollectionInput, CreateCollectionOutput};
use tauri::{Manager, Runtime as TauriRuntime};
use tauri_plugin_dapis::DapisState;

use crate::commands::create_collection_impl;
// FIXME: I'm not sure how to split request input
// So we will use default options when dispatching to the commands

pub async fn create_collection_handler<R: TauriRuntime>(
    State(state): State<DapisState<R>>,
    Json(input): Json<CreateCollectionInput>,
) -> TauriResult<String> {
    match create_collection_impl(state.app_handle.state::<App<R>>(), input, None).await {
        Ok(output) => Ok(serde_json::to_string_pretty(&output)
            .map_err(|err| TauriError::from(anyhow!("Cannot serialize output to string")))?),
        Err(err) => Err(err),
    }
}
