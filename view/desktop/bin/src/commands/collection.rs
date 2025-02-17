use moss_desktop::{
    app::{
        manager::AppManager, repositories::collection_repository::SledCollectionRepository,
        state::AppStateManager,
    },
    services::collection_service::CollectionService,
};
use moss_tauri::TauriResult;
use tauri::State;

use crate::MockLocalFileSystem;

#[tauri::command]
#[instrument(level = "trace", skip(app_manager))]
pub fn create_collection(app_manager: State<'_, AppManager>) -> TauriResult<()> {
    let collection_service = app_manager
        .service::<CollectionService<SledCollectionRepository, MockLocalFileSystem>>()?;

    todo!()
}
