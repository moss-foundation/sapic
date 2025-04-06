use std::path::PathBuf;

use moss_app::manager::AppManager;
// use moss_collection::collection_manager::CollectionManager;
use moss_tauri::TauriResult;
use moss_workspace::{
    models::operations::{CreateWorkspaceInput, OpenCollectionInput, OpenWorkspaceInput},
    workspace_manager::WorkspaceManager,
};
use tauri::State;

// #[tauri::command]
// #[instrument(level = "trace", skip(app_manager))]
// pub fn create_collection(app_manager: State<'_, AppManager>) -> TauriResult<()> {
//     let collection_service = app_manager.service::<CollectionManager>()?;

//     todo!()
// }

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager))]
pub async fn example_index_collection_command(
    app_manager: State<'_, AppManager<tauri::Wry>>,
) -> TauriResult<()> {
    dbg!(111111);
    let app_handle = app_manager.app_handle();
    let workspace_manager = app_manager
        .services()
        .get_by_type::<WorkspaceManager<tauri::Wry>>(&app_handle)
        .await?;

    workspace_manager
        .open_workspace(&OpenWorkspaceInput {
            name: "TestWorkspace".to_string(),
        })
        .await
        .expect("Failed to create workspace");

    let current_workspace = workspace_manager.current_workspace().unwrap();
    let collection_output = current_workspace
        .1
        .open_collection(OpenCollectionInput {
            path: PathBuf::from(
                "/Users/g10z3r/.sapic/workspaces/TestWorkspace/collections/TestCollection",
            ),
        })
        .await
        .expect("Failed to open collection");

    let collections = current_workspace.1.collections().await.unwrap();
    let collections_lock = collections.read().await;
    let (collection, _cache) = collections_lock.get(collection_output.key).unwrap();
    let requests = collection.list_requests().await.unwrap();

    println!("{:?}", requests);

    Ok(())
}
