#![cfg(feature = "integration-tests")]

use moss_storage::storage::operations::ListByPrefix;
use moss_testutils::random_name::random_environment_name;
use moss_workspace::{
    models::operations::CreateEnvironmentInput,
    storage::{entities::state_store::EnvironmentEntity, segments::SEGKEY_ENVIRONMENT},
};
use std::collections::HashMap;
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
                order: 42,
                color: Some("#3574F0".to_string()),
            },
        )
        .await
        .unwrap();

    let id = create_environment_output.id;

    let channel = Channel::new(move |_| Ok(()));
    let output = workspace.stream_environments(&ctx, channel).await.unwrap();
    assert_eq!(output.total_returned, 2); // Expected two because of 1 global + 1 created

    assert!(create_environment_output.abs_path.exists());

    // Check the newly created environment is stored in the db
    let item_store = workspace.db().item_store();

    let stored_envs = ListByPrefix::list_by_prefix(item_store.as_ref(), &ctx, "environment")
        .await
        .unwrap()
        .into_iter()
        .map(|(k, v)| (k, v.deserialize::<EnvironmentEntity>().unwrap()))
        .collect::<HashMap<_, _>>();

    assert_eq!(stored_envs.len(), 2);
    assert_eq!(
        stored_envs[&SEGKEY_ENVIRONMENT.join(id.as_str())],
        EnvironmentEntity {
            order: 42,
            // Newly created environments are expanded by default
            expanded: true,
        }
    );

    cleanup().await;
}
