#![cfg(feature = "integration-tests")]

use moss_testutils::random_name::random_environment_name;
use moss_workspace::models::operations::CreateEnvironmentInput;
use tauri::ipc::Channel;

use crate::shared::setup_test_workspace;

pub mod shared;

#[tokio::test]
async fn create_environment_success() {
    let (ctx, workspace, cleanup) = setup_test_workspace().await;

    let environment_name = random_environment_name();
    let create_environment_output = workspace
        .create_environment(
            &ctx,
            CreateEnvironmentInput {
                name: environment_name.clone(),
                collection_id: None,
                order: 0,
                color: Some("#3574F0".to_string()),
            },
        )
        .await
        .unwrap();

    let channel = Channel::new(move |_| Ok(()));
    let output = workspace.stream_environments(&ctx, channel).await.unwrap();
    assert_eq!(output.total_returned, 2); // Expected two because of 1 global + 1 created

    assert!(create_environment_output.abs_path.exists());

    // TODO: check the database when it's implemented

    cleanup().await;
}
