use moss_app::manager::AppManager;
use moss_app::service::prelude::*;
use moss_fs::FileSystem;
use moss_logging::{LogPayload, LogScope, LoggingService};
use moss_nls::locale_service::LocaleService;
use moss_session::SessionService;
use moss_state::service::AppDefaults;
use moss_state::{command, command::CommandContext, service::StateService};
use moss_storage::GlobalStorage;
use moss_tauri::TauriResult;
use moss_text::read_only_str;
use moss_theme::theme_service::ThemeService;
// use moss_workspace::workspace_manager::WorkspaceManager;
use std::marker::PhantomData;
use std::{path::PathBuf, sync::Arc};
use tauri::Runtime as TauriRuntime;
use tauri::{AppHandle, Manager};

pub fn service_pool<R: TauriRuntime>(
    app_handle: &AppHandle<R>,
    app_dir: &PathBuf,
    fs: Arc<dyn FileSystem>,
    global_storage: Arc<dyn GlobalStorage>,
) -> ServicePool<R> {
    let mut builder = ServicePoolBuilder::new();

    let session_service_key = builder.register(
        Instantiation::Instant(session_service(), PhantomData),
        app_handle,
    );
    let locale_service_key = builder.register(
        Instantiation::Instant(locale_service(fs.clone()), PhantomData),
        app_handle,
    );
    let theme_service_key = builder.register(
        Instantiation::Instant(theme_service(fs.clone()), PhantomData),
        app_handle,
    );

    builder.register(
        Instantiation::Instant(
            state_service(theme_service_key, locale_service_key),
            PhantomData,
        ),
        app_handle,
    );
    builder.register(
        Instantiation::Instant(logging_service(session_service_key), PhantomData),
        app_handle,
    );
    // builder.register(
    //     Instantiation::Instant(
    //         workspace_manager(fs.clone(), global_storage.clone(), app_dir),
    //         PhantomData,
    //     ),
    //     app_handle,
    // );

    builder.build()
}

fn state_service<R: TauriRuntime>(
    theme_service_key: ServiceKey,
    locale_service_key: ServiceKey,
) -> impl FnOnce(&ServicePool<R>, &AppHandle<R>) -> StateService<R> + Send + Sync + 'static {
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

fn session_service<R: TauriRuntime>()
-> impl Fn(&ServicePool<R>, &AppHandle<R>) -> SessionService + Send + Sync + 'static {
    move |_, _| SessionService::new()
}

fn theme_service<R: TauriRuntime>(
    fs: Arc<dyn FileSystem>,
) -> impl FnOnce(&ServicePool<R>, &AppHandle<R>) -> ThemeService + Send + Sync + 'static {
    let themes_dir: PathBuf = std::env::var("THEMES_DIR")
        .expect("Environment variable THEMES_DIR is not set")
        .into();

    move |_, _| ThemeService::new(fs, themes_dir.clone())
}

fn locale_service<R: TauriRuntime>(
    fs: Arc<dyn FileSystem>,
) -> impl FnOnce(&ServicePool<R>, &AppHandle<R>) -> LocaleService + Send + Sync + 'static {
    let locales_dir: PathBuf = std::env::var("LOCALES_DIR")
        .expect("Environment variable LOCALES_DIR is not set")
        .into();

    move |_, _| LocaleService::new(fs, locales_dir.clone())
}

fn logging_service<R: TauriRuntime>(
    session_service_key: ServiceKey,
) -> impl FnOnce(&ServicePool<R>, &AppHandle<R>) -> LoggingService + Send + Sync + 'static {
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

// fn workspace_manager<R: tauri::Runtime>(
//     fs: Arc<dyn FileSystem>,
//     global_storage: Arc<dyn GlobalStorage>,
//     app_dir: &PathBuf,
// ) -> impl FnOnce(&ServicePool<R>, &AppHandle<R>) -> WorkspaceManager<R> + Send + Sync + 'static {
//     let workspaces_dir: PathBuf = app_dir.join("workspaces");

//     move |_, app_handle| {
//         WorkspaceManager::new(
//             app_handle.clone(),
//             fs,
//             workspaces_dir.clone(),
//             global_storage,
//         )
//         .unwrap()
//     }
// }

async fn generate_log<R: TauriRuntime>(ctx: &mut CommandContext<R>) -> TauriResult<String> {
    let app_handle = ctx.app_handle();
    let app_manager = app_handle.state::<AppManager<R>>();
    let logging_service = app_manager
        .services()
        .get_by_type::<LoggingService>(app_handle)
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
