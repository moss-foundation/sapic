use moss_app::manager::AppManager;
use moss_app::service::prelude::*;
use moss_fs::ports::FileSystem;
use moss_logging::{LogPayload, LogScope, LoggingService};
use moss_nls::locale_service::LocaleService;
use moss_session::SessionService;
use moss_state::service::AppDefaults;
use moss_state::{command, command::CommandContext, service::StateService};
use moss_tauri::TauriResult;
use moss_text::read_only_str;
use moss_theme::theme_service::ThemeService;
use moss_workspace::workspace_manager::WorkspaceManager;
use std::{path::PathBuf, sync::Arc};
use tauri::{AppHandle, Manager};

pub fn service_pool(app_handle: &AppHandle, fs: Arc<dyn FileSystem>) -> ServicePool {
    let mut builder = ServicePoolBuilder::new();

    let session_service_key =
        builder.register(Instantiation::Instant(session_service()), app_handle);
    let locale_service_key = builder.register(
        Instantiation::Instant(locale_service(fs.clone())),
        app_handle,
    );
    let theme_service_key = builder.register(
        Instantiation::Instant(theme_service(fs.clone())),
        app_handle,
    );

    builder.register(
        Instantiation::Instant(state_service(theme_service_key, locale_service_key)),
        app_handle,
    );
    builder.register(
        Instantiation::Instant(logging_service(session_service_key)),
        app_handle,
    );
    builder.register(
        Instantiation::Instant(workspace_manager(fs.clone())),
        app_handle,
    );

    builder.build()
}

fn state_service(
    theme_service_key: ServiceKey,
    locale_service_key: ServiceKey,
) -> impl FnOnce(&ServicePool, &AppHandle) -> StateService + Send + Sync + 'static {
    move |pool, app_handle| {
        let default_theme = futures::executor::block_on(async move {
            let theme_service = pool
                .get_by_key::<ThemeService>(theme_service_key, app_handle)
                .await
                .expect("Theme service needs to be registered first");

            theme_service
                .default_theme()
                .await
                .expect("Failed to get default theme")
        });

        let default_locale = futures::executor::block_on(async move {
            let locale_service = pool
                .get_by_key::<LocaleService>(locale_service_key, app_handle)
                .await
                .expect("Locale service needs to be registered first");

            locale_service
                .default_locale()
                .await
                .expect("Failed to get default locale")
        });

        let defaults = AppDefaults {
            theme: default_theme.clone(),
            locale: default_locale.clone(),
        };

        StateService::new(defaults).with_commands([
            // FIXME: Remove this example command
            command!("example.generateLog", generate_log),
        ])
    }
}

fn session_service() -> impl Fn(&ServicePool, &AppHandle) -> SessionService + Send + Sync + 'static
{
    move |_, _| SessionService::new()
}

fn theme_service<'a>(
    fs: Arc<dyn FileSystem>,
) -> impl FnOnce(&ServicePool, &AppHandle) -> ThemeService + Send + Sync + 'static {
    let themes_dir: PathBuf = std::env::var("THEMES_DIR")
        .expect("Environment variable THEMES_DIR is not set")
        .into();

    move |_, _| ThemeService::new(fs, themes_dir.clone())
}

fn locale_service(
    fs: Arc<dyn FileSystem>,
) -> impl FnOnce(&ServicePool, &AppHandle) -> LocaleService + Send + Sync + 'static {
    let locales_dir: PathBuf = std::env::var("LOCALES_DIR")
        .expect("Environment variable LOCALES_DIR is not set")
        .into();

    move |_, _| LocaleService::new(fs, locales_dir.clone())
}

fn logging_service(
    session_service_key: ServiceKey,
) -> impl FnOnce(&ServicePool, &AppHandle) -> LoggingService + Send + Sync + 'static {
    // FIXME: In the future, we will place logs at appropriate locations
    // Now we put `logs` folder at the project root for easier development
    let app_log_dir: PathBuf = std::env::var("APP_LOG_DIR")
        .expect("Environment variable APP_LOG_DIR is not set")
        .into();
    let session_log_dir: PathBuf = std::env::var("SESSION_LOG_DIR")
        .expect("Environment variable SESSION_LOG_DIR is not set")
        .into();

    move |pool, app_handle| {
        let session_service = futures::executor::block_on(
            pool.get_by_key::<SessionService>(session_service_key, app_handle),
        )
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
) -> impl FnOnce(&ServicePool, &AppHandle) -> WorkspaceManager + Send + Sync + 'static {
    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspaces_dir: PathBuf = PathBuf::from(dir).join("samples").join("workspaces");

    move |_, _| WorkspaceManager::new(fs, workspaces_dir.clone())
}

async fn generate_log<'a>(ctx: &mut CommandContext) -> TauriResult<String> {
    let app_manager = ctx.app_handle().state::<AppManager>();
    let logging_service = app_manager
        .services()
        .get_by_type::<LoggingService>(&ctx.app_handle())
        .await?;

    logging_service.info(
        LogScope::App,
        LogPayload {
            collection: None,
            request: None,
            message: "Generate a log from the frontend".to_string(),
        },
    );

    Ok("Successfully generated a log!".to_string())
}
