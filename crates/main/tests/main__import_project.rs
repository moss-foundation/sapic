#![cfg(feature = "integration-tests")]

use moss_testutils::random_name::random_project_name;
use sapic_ipc::contracts::main::project::{
    CreateProjectInput, CreateProjectParams, ImportDiskParams, ImportProjectInput,
    ImportProjectParams, ImportProjectSource,
};

use crate::shared::{set_up_test_main_window, test_stream_projects};

mod shared;

// Skip testing cloning git repos for now
#[tokio::test]
async fn import_external_project_success() {
    let (main_window, _delegate, main_window_services, ctx, cleanup, test_path) =
        set_up_test_main_window().await;

    let project_name = random_project_name();
    let external_path = test_path.join(&project_name);

    let id = main_window
        .create_project(
            &ctx,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: project_name.to_string(),
                    order: 0,
                    external_path: Some(external_path.clone()),
                    git_params: None,
                    icon_path: None,
                },
            },
        )
        .await
        .unwrap()
        .id;

    let new_id = main_window
        .import_project(
            &ctx,
            &ImportProjectInput {
                inner: ImportProjectParams {
                    name: "Imported".to_string(),
                    order: 0,
                    source: ImportProjectSource::Disk(ImportDiskParams { external_path }),
                    icon_path: None,
                },
            },
        )
        .await
        .unwrap();

    let (_, projects) = test_stream_projects(&main_window, &ctx).await;
    assert_eq!(projects.len(), 2);
    cleanup().await;
}
