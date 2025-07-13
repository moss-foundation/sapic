// mod context;

// pub use context::*;

// use moss_activity_indicator::ActivityIndicator;
// use moss_app::{
//     App, AppBuilder,
//     app::AppDefaults,
//     models::{
//         primitives::ThemeMode,
//         types::{ColorThemeInfo, LocaleInfo},
//     },
//     services::{
//         log_service::LogService, session_service::SessionId, storage_service::StorageService,
//         workspace_service::WorkspaceService,
//     },
// };
// use moss_applib::{
//     ctx::MutableContext,
//     providers::{ServiceMap, ServiceProvider},
// };
// use moss_fs::{FileSystem, RealFileSystem};
// use moss_testutils::random_name::random_string;
// use std::{any::TypeId, future::Future, path::PathBuf, pin::Pin, sync::Arc};
// use tauri::test::MockRuntime;

// pub type CleanupFn = Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>;

// pub fn random_app_dir_path() -> PathBuf {
//     PathBuf::from(env!("CARGO_MANIFEST_DIR"))
//         .join("tests")
//         .join("data")
//         .join(random_string(10))
// }

// pub async fn set_up_test_app() -> (
//     App<MockRuntime>,
//     MutableContext, // TODO: this is temporary, should be a mock
//     ServiceProvider,
//     CleanupFn,
//     PathBuf,
// ) {
//     let fs = Arc::new(RealFileSystem::new());
//     let tauri_app = tauri::test::mock_app();
//     let app_handle = tauri_app.handle().to_owned();

//     <dyn FileSystem>::set_global(fs.clone(), &app_handle);

//     let app_path = random_app_dir_path();

//     let logs_abs_path = app_path.join("logs");
//     let workspaces_abs_path = app_path.join("workspaces");
//     let globals_abs_path = app_path.join("globals");

//     {
//         tokio::fs::create_dir_all(&app_path).await.unwrap();
//         tokio::fs::create_dir(&logs_abs_path).await.unwrap();
//         tokio::fs::create_dir(&workspaces_abs_path).await.unwrap();
//         tokio::fs::create_dir(&globals_abs_path).await.unwrap();
//     }

//     let storage_service: Arc<StorageService> = StorageService::new(&app_path).unwrap().into();

//     let session_id = SessionId::new();
//     let mut services: ServiceMap = Default::default();

//     let log_service: Arc<LogService> = LogService::new(
//         fs.clone(),
//         app_handle.clone(),
//         &logs_abs_path,
//         &session_id,
//         storage_service.clone(),
//     )
//     .unwrap()
//     .into();

//     let workspace_service: Arc<WorkspaceService<MockRuntime>> =
//         WorkspaceService::new(storage_service.clone(), fs.clone(), &app_path)
//             .await
//             .expect("Failed to create workspace service")
//             .into();

//     {
//         services.insert(TypeId::of::<LogService>(), log_service.clone());
//         services.insert(
//             TypeId::of::<WorkspaceService<MockRuntime>>(),
//             workspace_service.clone(),
//         );
//         services.insert(TypeId::of::<StorageService>(), storage_service.clone());
//     }

//     let cleanup_fn = Box::new({
//         let path = app_path.clone();
//         move || {
//             Box::pin(async move {
//                 if let Err(e) = tokio::fs::remove_dir_all(&path).await {
//                     eprintln!("Failed to clean up test directory: {}", e);
//                 }
//             }) as Pin<Box<dyn Future<Output = ()> + Send>>
//         }
//     });

//     // FIXME: This is a hack, should be a mock
//     let activity_indicator = ActivityIndicator::new(app_handle.clone());
//     let ctx = MutableContext::background(); // TODO: this is temporary, should be a mock
//     let app_builder = AppBuilder::new(
//         app_handle.clone(),
//         activity_indicator,
//         AppDefaults {
//             theme: ColorThemeInfo {
//                 identifier: "".to_string(),
//                 display_name: "".to_string(),
//                 mode: ThemeMode::Light,
//                 order: None,
//                 source: Default::default(),
//                 is_default: None,
//             },
//             locale: LocaleInfo {
//                 identifier: "".to_string(),
//                 display_name: "".to_string(),
//                 code: "".to_string(),
//                 direction: None,
//                 is_default: None,
//             },
//         },
//         fs.clone(),
//         app_path.clone(),
//     )
//     .with_service::<LogService>(log_service)
//     .with_service::<WorkspaceService<MockRuntime>>(workspace_service)
//     .with_service::<StorageService>(storage_service);

//     (
//         app_builder.build().await.unwrap(),
//         ctx,
//         services.into(),
//         cleanup_fn,
//         app_path,
//     )
// }
