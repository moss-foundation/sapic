#![cfg(feature = "integration-tests")]

use moss_testutils::random_name::random_project_name;
use sapic_ipc::contracts::main::project::{
    CreateProjectInput, CreateProjectParams, ExportProjectInput, ExportProjectParams,
    ImportArchiveParams, ImportProjectInput, ImportProjectParams, ImportProjectSource,
};

use crate::shared::{set_up_test_main_window, test_stream_projects};

mod shared;

// Create an archive file from a project and import it back
#[tokio::test]
async fn export_project_success() {
    let (main_window, _delegate, ctx, cleanup, test_path) = set_up_test_main_window().await;

    let project_name = random_project_name();
    let id = main_window
        .create_project(
            &ctx,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: project_name.to_string(),
                    order: 0,
                    external_path: None,
                    git_params: None,
                    icon_path: None,
                },
            },
        )
        .await
        .unwrap()
        .id;

    let archive_destination = test_path.join("archive");

    tokio::fs::create_dir_all(&archive_destination)
        .await
        .unwrap();

    let archive_path = main_window
        .export_project(
            &ctx,
            &ExportProjectInput {
                inner: ExportProjectParams {
                    id,
                    destination: archive_destination.clone(),
                },
            },
        )
        .await
        .unwrap()
        .archive_path;

    let _imported_id = main_window
        .import_project(
            &ctx,
            &ImportProjectInput {
                inner: ImportProjectParams {
                    name: "Imported".to_string(),
                    order: 0,
                    source: ImportProjectSource::Archive(ImportArchiveParams { archive_path }),
                    icon_path: None,
                },
            },
        )
        .await
        .unwrap();

    let (_, projects) = test_stream_projects(&main_window, &ctx).await;
    // 1 original + 1 imported
    assert_eq!(projects.len(), 2);

    cleanup().await;
}
