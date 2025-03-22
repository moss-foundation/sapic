use moss_app::service_pool::{Instantiation, ServiceKey, ServicePool, ServicePoolBuilder};
use moss_fs::{adapters::disk::DiskFileSystem, ports::FileSystem};
use moss_logging::LoggingService;
use moss_nls::locale_service::LocaleService;
use moss_session::SessionService;
use moss_state::service::StateService;
use moss_theme::theme_service::ThemeService;
use moss_workspace::workspace_manager::WorkspaceManager;
use std::{path::PathBuf, sync::Arc};
use tauri::AppHandle;

pub fn service_pool(app_handle: AppHandle) -> ServicePool {
    let fs: Arc<DiskFileSystem> = Arc::new(DiskFileSystem::new());

    let mut builder = ServicePoolBuilder::new(app_handle, 10);

    let session_service_key = builder.register(Instantiation::Instant(session_service()));

    builder.register(Instantiation::Instant(logging_service(session_service_key)));
    builder.register(Instantiation::Instant(workspace_manager(fs.clone())));

    builder.register(Instantiation::Lazy(locale_service(fs.clone())));
    builder.register(Instantiation::Lazy(theme_service(fs.clone())));

    builder.build()
}

fn state_service(
    theme_service_key: ServiceKey,
) -> impl Fn(&ServicePool, &AppHandle) -> StateService + Send + Sync + 'static {
    move |pool, _| {
        let theme_service = pool
            .get_by_key::<ThemeService>(theme_service_key)
            .expect("Theme service needs to be registered first");

        StateService::new(theme_service.themes_dir())
    }
}

fn session_service() -> impl Fn(&ServicePool, &AppHandle) -> SessionService + Send + Sync + 'static
{
    move |_, _| SessionService::new()
}

fn theme_service(
    fs: Arc<dyn FileSystem>,
) -> impl Fn(&ServicePool, &AppHandle) -> ThemeService + Send + Sync + 'static {
    let themes_dir: PathBuf = std::env::var("THEMES_DIR")
        .expect("Environment variable THEMES_DIR is not set")
        .into();

    move |_, _| ThemeService::new(Arc::clone(&fs), themes_dir.clone())
}

fn locale_service(
    fs: Arc<dyn FileSystem>,
) -> impl Fn(&ServicePool, &AppHandle) -> LocaleService + Send + Sync + 'static {
    let locales_dir: PathBuf = std::env::var("LOCALES_DIR")
        .expect("Environment variable LOCALES_DIR is not set")
        .into();

    move |_, _| LocaleService::new(Arc::clone(&fs), locales_dir.clone())
}

fn logging_service(
    session_service_key: ServiceKey,
) -> impl Fn(&ServicePool, &AppHandle) -> LoggingService + Send + Sync + 'static {
    // FIXME: In the future, we will place logs at appropriate locations
    // Now we put `logs` folder at the project root for easier development
    let app_log_dir: PathBuf = std::env::var("APP_LOG_DIR")
        .expect("Environment variable APP_LOG_DIR is not set")
        .into();
    let session_log_dir: PathBuf = std::env::var("SESSION_LOG_DIR")
        .expect("Environment variable SESSION_LOG_DIR is not set")
        .into();

    move |pool, _| {
        let session_service = pool
            .get_by_key::<SessionService>(session_service_key)
            .expect("Session service needs to be registered first");

        LoggingService::new(
            &app_log_dir,
            &session_log_dir,
            session_service.get_session_uuid(),
        )
        .unwrap()
    }
}

fn workspace_manager(
    fs: Arc<dyn FileSystem>,
) -> impl Fn(&ServicePool, &AppHandle) -> WorkspaceManager + Send + Sync + 'static {
    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspaces_dir: PathBuf = PathBuf::from(dir).join("samples").join("workspaces");

    move |_, _| WorkspaceManager::new(Arc::clone(&fs), workspaces_dir.clone())
}
