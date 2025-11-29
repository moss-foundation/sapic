#![cfg(feature = "integration-tests")]
pub mod shared;

use moss_applib::mock::MockAppRuntime;
use moss_project::{
    dirs,
    errors::ErrorNotFound,
    models::{operations::DeleteResourceInput, primitives::ResourceId},
};
use std::path::PathBuf;

use crate::shared::{
    RESOURCES_ROOT_DIR, create_test_component_dir_entry, create_test_endpoint_dir_entry,
    create_test_project, create_test_schema_dir_entry, random_entry_name,
};

#[tokio::test]
async fn delete_entry_success() {
    let (ctx, _, project_path, mut project, cleanup) = create_test_project().await;
    let resources_dir = project_path.join(dirs::RESOURCES_DIR);

    let entry_name = random_entry_name();
    let entry_path = PathBuf::from(RESOURCES_ROOT_DIR);
    let entry_id = create_test_endpoint_dir_entry(&ctx, &mut project, &entry_name).await;

    // Verify entry was created
    let expected_dir = resources_dir.join(&entry_path).join(&entry_name);
    assert!(expected_dir.exists());

    // Delete the entry
    let delete_input = DeleteResourceInput { id: entry_id };

    let result = project
        .delete_resource::<MockAppRuntime>(&ctx, delete_input)
        .await;
    let _ = result.unwrap();

    // Verify the directory was removed
    assert!(!expected_dir.exists());

    // Cleanup
    cleanup().await;
}

#[tokio::test]
async fn delete_entry_not_found() {
    let (ctx, _, _, project, cleanup) = create_test_project().await;

    let delete_input = DeleteResourceInput {
        id: ResourceId::new(),
    };

    let result = project
        .delete_resource::<MockAppRuntime>(&ctx, delete_input)
        .await;
    assert!(result.is_err());

    if let Err(error) = result {
        assert!(error.is::<ErrorNotFound>());
    }

    // Cleanup
    cleanup().await;
}

#[tokio::test]
async fn delete_entry_with_subdirectories() {
    let (ctx, _, project_path, mut project, cleanup) = create_test_project().await;
    let resources_dir = project_path.join(dirs::RESOURCES_DIR);

    let entry_name = random_entry_name();
    let entry_path = PathBuf::from(RESOURCES_ROOT_DIR);
    let entry_id = create_test_endpoint_dir_entry(&ctx, &mut project, &entry_name).await;

    // Create some subdirectories and files inside the entry
    let entry_dir = resources_dir.join(&entry_path).join(&entry_name);
    let sub_dir = entry_dir.join("subdir");
    let sub_sub_dir = sub_dir.join("subsubdir");

    std::fs::create_dir_all(&sub_sub_dir).unwrap();
    std::fs::write(sub_dir.join("test_file.txt"), "test content").unwrap();
    std::fs::write(sub_sub_dir.join("nested_file.md"), "nested content").unwrap();

    // Verify structure was created
    assert!(entry_dir.exists());
    assert!(sub_dir.exists());
    assert!(sub_sub_dir.exists());

    // Delete the entry
    let delete_input = DeleteResourceInput { id: entry_id };

    let result = project
        .delete_resource::<MockAppRuntime>(&ctx, delete_input)
        .await;
    let _ = result.unwrap();

    // Verify the entire directory tree was removed
    assert!(!entry_dir.exists());
    assert!(!sub_dir.exists());
    assert!(!sub_sub_dir.exists());

    // Cleanup
    cleanup().await;
}

#[tokio::test]
async fn delete_multiple_entries() {
    let (ctx, _, project_path, mut project, cleanup) = create_test_project().await;
    let resources_dir = project_path.join(dirs::RESOURCES_DIR);

    let entry1_name = format!("{}_1", random_entry_name());
    let entry2_name = format!("{}_2", random_entry_name());

    let entry1_path = PathBuf::from(RESOURCES_ROOT_DIR);
    let entry1_id = create_test_endpoint_dir_entry(&ctx, &mut project, &entry1_name).await;

    let entry2_path = PathBuf::from(RESOURCES_ROOT_DIR);
    let entry2_id = create_test_endpoint_dir_entry(&ctx, &mut project, &entry2_name).await;

    // Verify both entries were created
    let expected_dir1 = resources_dir.join(&entry1_path).join(&entry1_name);
    let expected_dir2 = resources_dir.join(&entry2_path).join(&entry2_name);
    assert!(expected_dir1.exists());
    assert!(expected_dir2.exists());

    // Delete first entry
    let delete_input1 = DeleteResourceInput { id: entry1_id };

    let result1 = project
        .delete_resource::<MockAppRuntime>(&ctx, delete_input1)
        .await;
    let _ = result1.unwrap();

    // Verify first entry was removed, second still exists
    assert!(!expected_dir1.exists());
    assert!(expected_dir2.exists());

    // Delete second entry
    let delete_input2 = DeleteResourceInput { id: entry2_id };

    let result2 = project
        .delete_resource::<MockAppRuntime>(&ctx, delete_input2)
        .await;
    let _ = result2.unwrap();

    // Verify both entries are now removed
    assert!(!expected_dir1.exists());
    assert!(!expected_dir2.exists());

    // Cleanup
    cleanup().await;
}

#[tokio::test]
async fn delete_entry_twice() {
    let (ctx, _, project_path, mut project, cleanup) = create_test_project().await;
    let resources_dir = project_path.join(dirs::RESOURCES_DIR);

    let entry_name = random_entry_name();
    let entry_path = PathBuf::from(RESOURCES_ROOT_DIR);
    let entry_id = create_test_endpoint_dir_entry(&ctx, &mut project, &entry_name).await;

    // Verify entry was created
    let expected_dir = resources_dir.join(&entry_path).join(&entry_name);
    assert!(expected_dir.exists());

    // Delete the entry first time - should succeed
    let delete_input = DeleteResourceInput { id: entry_id };

    let result1 = project
        .delete_resource::<MockAppRuntime>(&ctx, delete_input.clone())
        .await;
    let _ = result1.unwrap();

    // Verify the directory was removed
    assert!(!expected_dir.exists());

    // Try to delete the same entry again - should fail
    let result2 = project
        .delete_resource::<MockAppRuntime>(&ctx, delete_input)
        .await;
    assert!(result2.is_err());

    if let Err(error) = result2 {
        assert!(error.is::<ErrorNotFound>());
    }

    // Cleanup
    cleanup().await;
}

#[tokio::test]
async fn delete_entries_from_different_directories() {
    let (ctx, _, project_path, mut project, cleanup) = create_test_project().await;
    let resources_dir = project_path.join(dirs::RESOURCES_DIR);

    let mut entries = Vec::new();

    {
        let name = random_entry_name();
        let endpoint_id = create_test_endpoint_dir_entry(&ctx, &mut project, &name).await;
        entries.push((endpoint_id, name));
    }

    {
        let name = random_entry_name();
        let component_id = create_test_component_dir_entry(&ctx, &mut project, &name).await;
        entries.push((component_id, name));
    }

    {
        let name = random_entry_name();
        let schema_id = create_test_schema_dir_entry(&ctx, &mut project, &name).await;
        entries.push((schema_id, name));
    }

    // Create entries in different directories
    for (id, _) in &entries {
        let _ = project
            .delete_resource::<MockAppRuntime>(&ctx, DeleteResourceInput { id: id.clone() })
            .await
            .unwrap();
    }

    // Verify that all the dir entries are removed
    for (_, name) in entries {
        let expected_dir = resources_dir.join(&name);
        assert!(
            !expected_dir.exists(),
            "Entry not deleted at {:?}",
            expected_dir
        );
    }

    // Cleanup
    cleanup().await;
}
