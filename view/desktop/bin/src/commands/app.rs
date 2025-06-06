use anyhow::anyhow;
use moss_app::manager::AppManager;
use moss_logging::{LogPayload, LogScope, LoggingService};
use moss_nls::{
    locale_service::LocaleService,
    models::operations::{GetTranslationsInput, GetTranslationsOutput, ListLocalesOutput},
};
use moss_state::{
    command::CommandContext,
    models::{
        operations::{DescribeAppStateOutput, SetColorThemeInput, SetLocaleInput},
        types::{Defaults, Preferences},
    },
    service::StateService,
};
use moss_tauri::{TauriError, TauriResult};
use moss_text::{ReadOnlyStr, quote};
use moss_theme::{
    models::{
        events::ColorThemeChangeEventPayload,
        operations::{GetColorThemeInput, GetColorThemeOutput, ListColorThemesOutput},
    },
    theme_service::ThemeService,
};
use moss_workbench::workbench::Workbench;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use tauri::{Emitter, EventTarget, Manager, Runtime as TauriRuntime, State, Window};

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager), fields(window = window.label()))]
pub async fn set_color_theme<R: TauriRuntime>(
    app_manager: State<'_, AppManager<R>>,
    window: Window<R>,
    input: SetColorThemeInput,
) -> TauriResult<()> {
    let app_handle = app_manager.app_handle();
    let state_service = app_manager
        .services()
        .get_by_type::<StateService<R>>(&app_handle)
        .await?;

    for (label, _) in app_handle.webview_windows() {
        if window.label() == &label {
            continue;
        }

        app_handle
            .emit_to(
                EventTarget::webview_window(&label),
                "core://color-theme-changed",
                ColorThemeChangeEventPayload::new(&input.theme_info.identifier),
            )
            .map_err(|err| anyhow!("Failed to emit event to webview '{}': {}", label, err))?;
    }

    state_service.set_color_theme(input);

    Ok(())
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager), fields(window = window.label()))]
pub async fn get_color_theme<R: TauriRuntime>(
    app_manager: State<'_, AppManager<R>>,
    window: Window<R>,
    input: GetColorThemeInput,
) -> TauriResult<GetColorThemeOutput> {
    let app_handle = app_manager.app_handle();
    let theme_service = app_manager
        .services()
        .get_by_type::<ThemeService>(app_handle)
        .await?;

    Ok(theme_service.get_color_theme(input).await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager), fields(window = window.label()))]
pub async fn list_color_themes<R: TauriRuntime>(
    app_manager: State<'_, AppManager<R>>,
    window: Window<R>,
) -> TauriResult<ListColorThemesOutput> {
    let app_handle = app_manager.app_handle();
    let theme_service = app_manager
        .services()
        .get_by_type::<ThemeService>(app_handle)
        .await?;

    Ok(theme_service.list_color_themes().await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager), fields(window = window.label()))]
pub async fn describe_app_state<R: TauriRuntime>(
    app_manager: State<'_, AppManager<R>>,
    window: Window<R>,
) -> TauriResult<DescribeAppStateOutput> {
    let app_handle = app_manager.app_handle();
    let state_service = app_manager
        .services()
        .get_by_type::<StateService<R>>(&app_handle)
        .await?;

    let workspace_manager = app_manager
        .services()
        .get_by_type::<Workbench<R>>(&app_handle)
        .await?;

    // HACK: This is a hack to get the last workspace name
    let last_workspace_name = workspace_manager
        .active_workspace()
        .map(|active_workspace| {
            active_workspace
                .inner
                .abs_path()
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string()
        });

    Ok(DescribeAppStateOutput {
        preferences: Preferences {
            theme: state_service.preferences().theme.read().clone(),
            locale: state_service.preferences().locale.read().clone(),
        },
        defaults: Defaults {
            theme: state_service.defaults().theme.clone(),
            locale: state_service.defaults().locale.clone(),
        },
        last_workspace: last_workspace_name, // Some("TestWorkspace".to_string())
    })
}

#[tauri::command]
#[instrument(level = "trace", skip(app_manager), fields(window = window.label()))]
pub async fn set_locale<R: TauriRuntime>(
    app_manager: State<'_, AppManager<R>>,
    window: Window<R>,
    input: SetLocaleInput,
) -> TauriResult<()> {
    let app_handle = app_manager.app_handle();
    let state_service = app_manager
        .services()
        .get_by_type::<StateService<R>>(app_handle)
        .await?;

    state_service.set_locale(input);

    Ok(())
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager), fields(window = window.label()))]
pub async fn list_locales<R: TauriRuntime>(
    app_manager: State<'_, AppManager<R>>,
    window: Window<R>,
) -> TauriResult<ListLocalesOutput> {
    let app_handle = app_manager.app_handle();
    let locale_service = app_manager
        .services()
        .get_by_type::<LocaleService>(app_handle)
        .await?;

    Ok(locale_service.list_locales().await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager), fields(window = window.label()))]
pub async fn get_translations<R: TauriRuntime>(
    app_manager: State<'_, AppManager<R>>,
    window: Window<R>,
    input: GetTranslationsInput,
) -> TauriResult<GetTranslationsOutput> {
    let app_handle = app_manager.app_handle();
    let locale_service = app_manager
        .services()
        .get_by_type::<LocaleService>(app_handle)
        .await?;

    Ok(locale_service.get_translations(&input).await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager), fields(window = window.label()))]
pub async fn execute_command<R: TauriRuntime>(
    app_manager: State<'_, AppManager<R>>,
    window: Window<R>,
    cmd: ReadOnlyStr,
    args: HashMap<String, JsonValue>,
) -> TauriResult<JsonValue> {
    let app_handle = app_manager.app_handle();
    let state_service = app_manager
        .services()
        .get_by_type::<StateService<R>>(app_handle)
        .await?;

    match state_service.get_command(&cmd) {
        Some(command_handler) => {
            command_handler(&mut CommandContext::new(app_handle.clone(), window, args)).await
        }
        _ => Err(TauriError(format!(
            "command with id {} is not found",
            quote!(cmd)
        ))),
    }
}
