#![cfg(feature = "integration-tests")]

use image::{ImageBuffer, Rgb};

use moss_activity_indicator::ActivityIndicator;
use moss_applib::{
    context::{AsyncContext, MutableContext},
    mock::MockAppRuntime,
    providers::{ServiceMap, ServiceProvider},
};
use moss_environment::GlobalEnvironmentRegistry;
use moss_fs::{FileSystem, RealFileSystem, model_registry::GlobalModelRegistry};
use moss_storage::primitives::segkey::SegKeyBuf;
use moss_testutils::random_name::random_workspace_name;
use moss_workspace::{
    Workspace,
    builder::{CreateWorkspaceParams, WorkspaceBuilder},
    models::{
        primitives::{CollectionId, EditorGridOrientation, PanelRenderer},
        types::{
            EditorGridLeafData, EditorGridNode, EditorGridState, EditorPanelState,
            EditorPartStateInfo,
        },
    },
    services::{
        AnyCollectionService, AnyLayoutService, DynCollectionService, DynLayoutService,
        DynStorageService, collection_service::CollectionService,
        environment_service::EnvironmentService, layout_service::LayoutService,
        storage_service::StorageService,
    },
    storage::segments::SEGKEY_COLLECTION,
};
use rand::Rng;
use std::{
    any::TypeId,
    collections::HashMap,
    fs,
    future::Future,
    path::{Path, PathBuf},
    pin::Pin,
    sync::Arc,
    time::Duration,
};

pub type CleanupFn = Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>;

pub async fn setup_test_workspace() -> (
    AsyncContext, // TODO: this is temporary, should be a mock
    Arc<Path>,
    Workspace<MockAppRuntime>,
    ServiceProvider,
    CleanupFn,
) {
    let fs = Arc::new(RealFileSystem::new());
    let mock_app = tauri::test::mock_app();
    let app_handle = mock_app.handle().clone();

    <dyn FileSystem>::set_global(fs.clone(), &app_handle);

    let ctx = MutableContext::background_with_timeout(Duration::from_secs(30)).freeze();

    let abs_path: Arc<Path> = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("workspaces")
        .join(random_workspace_name())
        .into();
    fs::create_dir_all(&abs_path).unwrap();

    let mut services: ServiceMap = Default::default();

    let activity_indicator = ActivityIndicator::new(app_handle.clone());

    let storage_service: Arc<StorageService<MockAppRuntime>> =
        StorageService::new(&abs_path).unwrap().into();
    let storage_service_dyn: Arc<DynStorageService<MockAppRuntime>> =
        DynStorageService::new(storage_service.clone());

    let layout_service: Arc<LayoutService<MockAppRuntime>> =
        LayoutService::new(storage_service_dyn.clone()).into();
    let layout_service_dyn: Arc<DynLayoutService<MockAppRuntime>> =
        DynLayoutService::new(layout_service.clone() as Arc<dyn AnyLayoutService<MockAppRuntime>>);

    let collection_service: Arc<CollectionService<MockAppRuntime>> = CollectionService::new(
        &ctx,
        abs_path.clone(),
        fs.clone(),
        storage_service_dyn.clone(),
    )
    .await
    .unwrap()
    .into();
    let collection_service_dyn: Arc<DynCollectionService<MockAppRuntime>> =
        DynCollectionService::new(
            collection_service.clone() as Arc<dyn AnyCollectionService<MockAppRuntime>>
        );

    let global_env_registry = GlobalEnvironmentRegistry::new();
    let global_model_registry = GlobalModelRegistry::new();

    let environment_service: Arc<EnvironmentService<MockAppRuntime>> = EnvironmentService::new(
        &abs_path,
        fs.clone(),
        Arc::new(global_env_registry),
        Arc::new(global_model_registry),
    )
    .into();

    {
        services.insert(
            TypeId::of::<LayoutService<MockAppRuntime>>(),
            layout_service.clone(),
        );
        services.insert(
            TypeId::of::<StorageService<MockAppRuntime>>(),
            storage_service.clone(),
        );
        services.insert(
            TypeId::of::<CollectionService<MockAppRuntime>>(),
            collection_service.clone(),
        );
        services.insert(
            TypeId::of::<EnvironmentService<MockAppRuntime>>(),
            environment_service.clone(),
        );
    }

    let workspace = WorkspaceBuilder::new(fs.clone())
        .with_service::<DynStorageService<MockAppRuntime>>(storage_service_dyn)
        .with_service::<DynCollectionService<MockAppRuntime>>(collection_service_dyn)
        .with_service::<DynLayoutService<MockAppRuntime>>(layout_service_dyn)
        .with_service::<EnvironmentService<MockAppRuntime>>(environment_service.clone())
        .create(
            &ctx,
            activity_indicator,
            CreateWorkspaceParams {
                name: random_workspace_name(),
                abs_path: abs_path.clone(),
            },
        )
        .await
        .unwrap();

    let cleanup_fn = Box::new({
        let abs_path_clone = abs_path.clone();
        move || {
            Box::pin(async move {
                if let Err(e) = tokio::fs::remove_dir_all(&abs_path_clone).await {
                    eprintln!("Failed to clean up test workspace directory: {}", e);
                }
            }) as Pin<Box<dyn Future<Output = ()> + Send>>
        }
    });

    (ctx, abs_path, workspace, services.into(), cleanup_fn)
}

pub fn create_simple_editor_state() -> EditorPartStateInfo {
    // Create a simple grid with one leaf
    let leaf_data = EditorGridLeafData {
        views: vec!["panel1".to_string()],
        active_view: "panel1".to_string(),
        id: "group1".to_string(),
    };

    let grid_node = EditorGridNode::Leaf {
        data: leaf_data,
        size: 100.0,
    };

    // Create grid state
    let grid = EditorGridState {
        root: grid_node,
        width: 800.0,
        height: 600.0,
        orientation: EditorGridOrientation::Horizontal,
    };

    // Create some panels
    let mut panels = HashMap::new();

    panels.insert(
        "panel1".to_string(),
        EditorPanelState {
            id: "panel1".to_string(),
            content_component: Some("TestComponent".to_string()),
            tab_component: None,
            title: Some("Test Panel".to_string()),
            renderer: Some(PanelRenderer::OnlyWhenVisible),
            params: Some(HashMap::new()),
            minimum_width: None,
            minimum_height: None,
            maximum_width: None,
            maximum_height: None,
        },
    );

    panels.insert(
        "panel2".to_string(),
        EditorPanelState {
            id: "panel2".to_string(),
            content_component: Some("AnotherComponent".to_string()),
            tab_component: None,
            title: Some("Another Panel".to_string()),
            renderer: None,
            params: None,
            minimum_width: Some(200.0),
            minimum_height: Some(150.0),
            maximum_width: None,
            maximum_height: None,
        },
    );

    EditorPartStateInfo {
        grid,
        panels,
        active_group: Some("group1".to_string()),
    }
}

pub fn collection_key(id: &CollectionId) -> SegKeyBuf {
    SEGKEY_COLLECTION.join(id)
}

pub fn generate_random_icon(output_path: &Path) {
    // Create an empty RGB image buffer
    let mut img = ImageBuffer::new(128, 128);
    let mut rng = rand::rng();

    // Fill each pixel with random values [0..255]
    for pixel in img.pixels_mut() {
        let r: u8 = rng.random();
        let g: u8 = rng.random();
        let b: u8 = rng.random();
        *pixel = Rgb([r, g, b]);
    }

    img.save(output_path).unwrap();
}
