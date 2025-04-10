use anyhow::anyhow;
use moss_app::manager::AppManager;
use moss_nls::{
    locale_service::LocaleService,
    models::operations::{GetTranslationsInput, GetTranslationsOutput, ListLocalesOutput},
};
use moss_state::{
    command::CommandContext,
    models::operations::{SetColorThemeInput, SetLocaleInput},
};
use moss_state::{
    models::{
        operations::DescribeAppStateOutput,
        types::{Defaults, Preferences},
    },
    service::StateService,
};
use moss_tauri::{TauriError, TauriResult};
use moss_text::{quote, ReadOnlyStr};
use moss_theme::{
    models::{
        events::ColorThemeChangeEventPayload,
        operations::{GetColorThemeInput, GetColorThemeOutput, ListColorThemesOutput},
    },
    theme_service::ThemeService,
};
use serde_json::{Value as JsonValue, Value};
use std::collections::HashMap;
use tauri::{Emitter, EventTarget, Manager, State, Window};

#[tauri::command]
#[instrument(level = "trace", skip(app_manager), fields(window = window.label()))]
pub async fn set_color_theme(
    app_manager: State<'_, AppManager<tauri::Wry>>,
    window: Window,
    input: SetColorThemeInput,
) -> TauriResult<()> {
    let app_handle = app_manager.app_handle();
    let state_service = app_manager
        .services()
        .get_by_type::<StateService<tauri::Wry>>(&app_handle)
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
#[instrument(level = "trace", skip(app_manager))]
pub async fn get_color_theme(
    app_manager: State<'_, AppManager<tauri::Wry>>,
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
#[instrument(level = "trace", skip(app_manager))]
pub async fn list_color_themes(
    app_manager: State<'_, AppManager<tauri::Wry>>,
) -> TauriResult<ListColorThemesOutput> {
    let app_handle = app_manager.app_handle();
    let theme_service = app_manager
        .services()
        .get_by_type::<ThemeService>(app_handle)
        .await?;

    Ok(theme_service.list_color_themes().await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager))]
pub async fn describe_app_state(
    app_manager: State<'_, AppManager<tauri::Wry>>,
) -> TauriResult<DescribeAppStateOutput> {
    let app_handle = app_manager.app_handle();
    let state_service = app_manager
        .services()
        .get_by_type::<StateService<tauri::Wry>>(&app_handle)
        .await?;

    Ok(DescribeAppStateOutput {
        preferences: Preferences {
            theme: state_service.preferences().theme.read().clone(),
            locale: state_service.preferences().locale.read().clone(),
        },
        defaults: Defaults {
            theme: state_service.defaults().theme.clone(),
            locale: state_service.defaults().locale.clone(),
        },
    })
}

#[tauri::command]
#[instrument(level = "trace", skip(app_manager))]
pub async fn set_locale(
    app_manager: State<'_, AppManager<tauri::Wry>>,
    input: SetLocaleInput,
) -> TauriResult<()> {
    let app_handle = app_manager.app_handle();
    let state_service = app_manager
        .services()
        .get_by_type::<StateService<tauri::Wry>>(app_handle)
        .await?;

    state_service.set_locale(input);

    Ok(())
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager))]
pub async fn list_locales(
    app_manager: State<'_, AppManager<tauri::Wry>>,
) -> TauriResult<ListLocalesOutput> {
    let app_handle = app_manager.app_handle();
    let locale_service = app_manager
        .services()
        .get_by_type::<LocaleService>(app_handle)
        .await?;

    Ok(locale_service.list_locales().await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager))]
pub async fn get_translations(
    app_manager: State<'_, AppManager<tauri::Wry>>,
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
pub async fn execute_command(
    app_manager: State<'_, AppManager<tauri::Wry>>,
    window: Window,
    cmd: ReadOnlyStr,
    args: HashMap<String, Value>,
) -> TauriResult<Value> {
    let app_handle = app_manager.app_handle();
    let state_service = app_manager
        .services()
        .get_by_type::<StateService<tauri::Wry>>(app_handle)
        .await?;

    if let Some(command_handler) = state_service.get_command(&cmd) {
        command_handler(&mut CommandContext::new(app_handle.clone(), window, args)).await
    } else {
        Err(TauriError(format!(
            "command with id {} is not found",
            quote!(cmd)
        )))
    }
}
