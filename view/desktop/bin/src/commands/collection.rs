use moss_app::manager::AppManager;
use moss_collection::collection_manager::CollectionManager;
use moss_tauri::TauriResult;
use tauri::State;

#[tauri::command]
#[instrument(level = "trace", skip(app_manager))]
pub fn create_collection(app_manager: State<'_, AppManager>) -> TauriResult<()> {
    let collection_service = app_manager.service::<CollectionManager>()?;

    todo!()
}
