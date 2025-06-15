// use moss_app::service::prelude::*;
// use moss_fs::FileSystem;
// use moss_logging::LoggingService;
// use moss_nls::locale_service::LocaleService;
// use moss_session::SessionService;
// use moss_state::service::{AppDefaults, StateService};
// use moss_theme::theme_service::ThemeService;
// use std::{marker::PhantomData, path::PathBuf, sync::Arc};
// use tauri::{AppHandle, Runtime as TauriRuntime};

// pub fn service_pool<R: TauriRuntime>(
//     app_handle: &AppHandle<R>,
//     fs: Arc<dyn FileSystem>,
// ) -> ServicePool<R> {
//     let mut builder = ServicePoolBuilder::new();

//     let session_service_key = builder.register(
//         Instantiation::Instant(session_service(), PhantomData),
//         app_handle,
//     );
//     let locale_service_key = builder.register(
//         Instantiation::Instant(locale_service(fs.clone()), PhantomData),
//         app_handle,
//     );
//     let theme_service_key = builder.register(
//         Instantiation::Instant(theme_service(fs.clone()), PhantomData),
//         app_handle,
//     );

//     builder.register(
//         Instantiation::Instant(
//             state_service(theme_service_key, locale_service_key),
//             PhantomData,
//         ),
//         app_handle,
//     );
//     builder.register(
//         Instantiation::Instant(
//             logging_service(session_service_key, fs.clone()),
//             PhantomData,
//         ),
//         app_handle,
//     );

//     builder.build()
// }

// fn state_service<R: TauriRuntime>(
//     theme_service_key: ServiceKey,
//     locale_service_key: ServiceKey,
// ) -> impl FnOnce(&ServicePool<R>, &AppHandle<R>) -> StateService<R> + Send + Sync + 'static {
//     move |pool, app_handle| {
//         let default_theme = futures::executor::block_on(async move {
//             let theme_service = pool
//                 .get_by_key::<ThemeService>(theme_service_key, app_handle)
//                 .await
//                 .expect("Theme service needs to be registered first");

//             theme_service
//                 .default_theme()
//                 .await
//                 .expect("Failed to get default theme")
//         });

//         let default_locale = futures::executor::block_on(async move {
//             let locale_service = pool
//                 .get_by_key::<LocaleService>(locale_service_key, app_handle)
//                 .await
//                 .expect("Locale service needs to be registered first");

//             locale_service
//                 .default_locale()
//                 .await
//                 .expect("Failed to get default locale")
//         });

//         let defaults = AppDefaults {
//             theme: default_theme.clone(),
//             locale: default_locale.clone(),
//         };

//         StateService::new(defaults)
//     }
// }

// fn session_service<R: TauriRuntime>()
// -> impl Fn(&ServicePool<R>, &AppHandle<R>) -> SessionService + Send + Sync + 'static {
//     move |_, _| SessionService::new()
// }

// fn theme_service<R: TauriRuntime>(
//     fs: Arc<dyn FileSystem>,
// ) -> impl FnOnce(&ServicePool<R>, &AppHandle<R>) -> ThemeService + Send + Sync + 'static {
//     let themes_dir: PathBuf = std::env::var("THEMES_DIR")
//         .expect("Environment variable THEMES_DIR is not set")
//         .into();

//     move |_, _| ThemeService::new(fs, themes_dir.clone())
// }

// fn locale_service<R: TauriRuntime>(
//     fs: Arc<dyn FileSystem>,
// ) -> impl FnOnce(&ServicePool<R>, &AppHandle<R>) -> LocaleService + Send + Sync + 'static {
//     let locales_dir: PathBuf = std::env::var("LOCALES_DIR")
//         .expect("Environment variable LOCALES_DIR is not set")
//         .into();

//     move |_, _| LocaleService::new(fs, locales_dir.clone())
// }

// fn logging_service<R: TauriRuntime>(
//     session_service_key: ServiceKey,
//     fs: Arc<dyn FileSystem>,
// ) -> impl FnOnce(&ServicePool<R>, &AppHandle<R>) -> LoggingService + Send + Sync + 'static {
//     // FIXME: In the future, we will place logs at appropriate locations
//     // Now we put `logs` folder at the project root for easier development
//     let app_log_dir: PathBuf = std::env::var("APP_LOG_DIR")
//         .expect("Environment variable APP_LOG_DIR is not set")
//         .into();

//     move |pool, app_handle| {
//         let session_service = futures::executor::block_on(
//             pool.get_by_key::<SessionService>(session_service_key, app_handle),
//         )
//         .expect("Session service needs to be registered first");

//         LoggingService::new(
//             fs,
//             app_handle.clone(),
//             &app_log_dir,
//             session_service.get_session_uuid(),
//         )
//         .unwrap()
//     }
// }
