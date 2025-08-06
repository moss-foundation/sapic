#![cfg(feature = "integration-tests")]

use crate::shared::setup_test_workspace;
use moss_storage::storage::operations::ListByPrefix;
use moss_testutils::random_name::random_environment_name;
use moss_workspace::{
    models::operations::CreateEnvironmentInput, storage::entities::state_store::EnvironmentEntity,
};
use tauri::ipc::Channel;

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

    let item_store = workspace.db().item_store();

    let stored_envs = ListByPrefix::list_by_prefix(item_store.as_ref(), &ctx, "environment")
        .await
        .unwrap()
        .into_iter()
        .map(|(_, v)| v.deserialize::<EnvironmentEntity>().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(stored_envs.len(), 2);

    cleanup().await;
}
