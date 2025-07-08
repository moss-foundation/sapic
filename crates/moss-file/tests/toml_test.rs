use anyhow::Result;
use moss_file::{
    common::FileHandle,
    toml::{EditableInPlaceFileHandle, InPlaceEditor},
};
use moss_fs::{FileSystem, RealFileSystem};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};
use toml_edit::{DocumentMut, value};

// Helper struct to manage a temporary directory for test files
struct TestDir {
    path: PathBuf,
}

impl TestDir {
    fn new(test_name: &str) -> Result<Self> {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let unique_id = nanoid!(10);
        let path = manifest_dir
            .join("data")
            .join(format!("{}-{}", test_name, unique_id));

        fs::create_dir_all(&path)?;
        Ok(Self { path })
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        // Attempt to remove the directory and its contents. Ignore errors during cleanup.
        let _ = fs::remove_dir_all(&self.path);
    }
}

// The TestData struct used in the tests
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct TestData {
    name: String,
    value: i32,
}

// The MockTomlEditor used for EditableFileHandle tests
struct MockTomlEditor {
    new_name: String,
    new_value: i32,
}

impl InPlaceEditor for MockTomlEditor {
    fn edit(&self, doc: &mut DocumentMut) -> Result<()> {
        doc["name"] = value(self.new_name.clone());
        doc["value"] = value(self.new_value as i64);
        Ok(())
    }
}

fn real_fs() -> Arc<dyn FileSystem> {
    Arc::new(RealFileSystem::new())
}

#[tokio::test]
async fn test_file_handle_new_and_load() -> Result<()> {
    let test_dir = TestDir::new("fh_new_load")?;
    let file_path = test_dir.path().join("test_fh.toml");
    let fs_arc = real_fs();

    let initial_data = TestData {
        name: "test".to_string(),
        value: 1,
    };

    let handle = FileHandle::create(
        Arc::clone(&fs_arc),
        &file_path,
        initial_data.clone(),
        |data| {
            toml::to_string(data).map_err(|e| anyhow::anyhow!("Failed to serialize data: {}", e))
        },
    )
    .await?;
    assert_eq!(handle.model().await, initial_data);
    assert_eq!(handle.path().as_ref(), &file_path);

    let content = fs::read_to_string(&file_path)?;
    let loaded_data_direct: TestData = toml::from_str(&content)?;
    assert_eq!(loaded_data_direct, initial_data);

    let loaded_handle = FileHandle::<TestData>::load(fs_arc, &file_path, |content| {
        toml::from_str(&content).map_err(|e| anyhow::anyhow!("Failed to deserialize data: {}", e))
    })
    .await?;
    assert_eq!(loaded_handle.model().await, initial_data);

    Ok(())
}

#[tokio::test]
async fn test_file_handle_edit() -> Result<()> {
    let test_dir = TestDir::new("fh_edit")?;
    let file_path = test_dir.path().join("test_fh_edit.toml");
    let fs_arc = real_fs();

    let initial_data = TestData {
        name: "initial".to_string(),
        value: 10,
    };
    let handle = FileHandle::create(
        Arc::clone(&fs_arc),
        &file_path,
        initial_data.clone(),
        |data| {
            toml::to_string(data).map_err(|e| anyhow::anyhow!("Failed to serialize data: {}", e))
        },
    )
    .await?;

    let new_name = "edited".to_string();
    let new_value = 20;

    handle
        .edit(
            |data| {
                data.name = new_name.clone();
                data.value = new_value;
                Ok(())
            },
            |data| {
                toml::to_string(data)
                    .map_err(|e| anyhow::anyhow!("Failed to serialize data: {}", e))
            },
        )
        .await?;

    let expected_data = TestData {
        name: new_name.clone(),
        value: new_value,
    };
    assert_eq!(handle.model().await, expected_data);

    let loaded_handle = FileHandle::<TestData>::load(fs_arc, &file_path, |content| {
        toml::from_str(&content).map_err(|e| anyhow::anyhow!("Failed to deserialize data: {}", e))
    })
    .await?;
    assert_eq!(loaded_handle.model().await, expected_data);

    Ok(())
}

#[tokio::test]
async fn test_editable_file_handle_new_and_load() -> Result<()> {
    let test_dir = TestDir::new("efh_new_load")?;
    let file_path = test_dir.path().join("test_efh.toml");
    let fs_arc = real_fs();

    let initial_data = TestData {
        name: "test_efh".to_string(),
        value: 101,
    };

    let handle =
        EditableInPlaceFileHandle::create(Arc::clone(&fs_arc), &file_path, initial_data.clone())
            .await?;
    assert_eq!(handle.model().await, initial_data);
    assert_eq!(handle.path().as_ref(), &file_path);

    let content = fs::read_to_string(&file_path)?;
    let loaded_data_direct: TestData = toml::from_str(&content)?;
    assert_eq!(loaded_data_direct, initial_data);

    let loaded_handle = EditableInPlaceFileHandle::<TestData>::load(fs_arc, &file_path).await?;
    assert_eq!(loaded_handle.model().await, initial_data);

    Ok(())
}

#[tokio::test]
async fn test_editable_file_handle_edit() -> Result<()> {
    let test_dir = TestDir::new("efh_edit")?;
    let file_path = test_dir.path().join("test_efh_edit.toml");
    let fs_arc = real_fs();
    let initial_data = TestData {
        name: "efh_initial".to_string(),
        value: 202,
    };

    let handle =
        EditableInPlaceFileHandle::create(Arc::clone(&fs_arc), &file_path, initial_data).await?;

    let editor = MockTomlEditor {
        new_name: "efh_edited".to_string(),
        new_value: 303,
    };
    handle.edit(editor).await?;

    let expected_data = TestData {
        name: "efh_edited".to_string(),
        value: 303,
    };
    assert_eq!(handle.model().await, expected_data);

    let loaded_handle = EditableInPlaceFileHandle::<TestData>::load(fs_arc, &file_path).await?;
    assert_eq!(loaded_handle.model().await, expected_data);

    Ok(())
}
