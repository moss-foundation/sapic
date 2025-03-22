use anyhow::anyhow;
use moss_app::manager::AppManager;
use moss_nls::{
    locale_service::LocaleService,
    models::{
        operations::{GetTranslationsInput, ListLocalesOutput},
        types::LocaleDescriptor,
    },
};
use moss_state::command::CommandContext;
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
        events::ColorThemeChangeEventPayload, operations::ListThemesOutput, types::ThemeDescriptor,
    },
    primitives::ThemeId,
    theme_service::ThemeService,
};
use serde_json::{Value as JsonValue, Value};
use std::collections::HashMap;
use tauri::{AppHandle, Emitter, EventTarget, Manager, State, Window};

#[tauri::command]
#[instrument(level = "trace", skip(app_handle, app_manager), fields(window = window.label()))]
pub async fn change_color_theme(
    app_handle: AppHandle,
    app_manager: State<'_, AppManager>,
    window: Window,
    descriptor: ThemeDescriptor, // FIXME: Should be something like ChangeColorThemeInput
) -> TauriResult<()> {
    let state_service = app_manager
        .services()
        .get_by_type::<StateService>(&app_handle)
        .await?;

    for (label, _) in app_handle.webview_windows() {
        if window.label() == &label {
            continue;
        }

        app_handle
            .emit_to(
                EventTarget::webview_window(&label),
                "core://color-theme-changed",
                ColorThemeChangeEventPayload::new(&descriptor.identifier),
            )
            .map_err(|err| anyhow!("Failed to emit event to webview '{}': {}", label, err))?;
    }

    state_service.set_color_theme(descriptor);

    Ok(())
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_handle, app_manager))]
pub async fn get_color_theme(
    app_handle: AppHandle,
    app_manager: State<'_, AppManager>,
    id: ThemeId, // FIXME: Should be something like GetColorThemeInput
) -> TauriResult<String> {
    let theme_service = app_manager
        .services()
        .get_by_type::<ThemeService>(&app_handle)
        .await?;

    Ok(theme_service.read_color_theme(&id).await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_handle, app_manager))]
pub async fn list_themes(
    app_handle: AppHandle,
    app_manager: State<'_, AppManager>,
) -> TauriResult<ListThemesOutput> {
    let theme_service = app_manager
        .services()
        .get_by_type::<ThemeService>(&app_handle)
        .await?;

    Ok(theme_service.list_themes().await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_handle, app_manager))]
pub async fn describe_app_state(
    app_handle: AppHandle,
    app_manager: State<'_, AppManager>,
) -> TauriResult<DescribeAppStateOutput> {
    let state_service = app_manager
        .services()
        .get_by_type::<StateService>(&app_handle)
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
#[instrument(level = "trace", skip(app_handle, app_manager))]
pub async fn change_language_pack(
    app_handle: AppHandle,
    app_manager: State<'_, AppManager>,
    descriptor: LocaleDescriptor, // FIXME: Should be something like ChangeLanguagePackInput
) -> TauriResult<()> {
    let state_service = app_manager
        .services()
        .get_by_type::<StateService>(&app_handle)
        .await?;

    state_service.set_language_pack(descriptor);

    Ok(())
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_handle, app_manager))]
pub async fn list_locales(
    app_handle: AppHandle,
    app_manager: State<'_, AppManager>,
) -> TauriResult<ListLocalesOutput> {
    let locale_service = app_manager
        .services()
        .get_by_type::<LocaleService>(&app_handle)
        .await?;

    Ok(locale_service.list_locales().await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_handle, app_manager))]
pub async fn get_translations(
    app_handle: AppHandle,
    app_manager: State<'_, AppManager>,
    input: GetTranslationsInput,
) -> TauriResult<JsonValue> {
    let locale_service = app_manager
        .services()
        .get_by_type::<LocaleService>(&app_handle)
        .await?;

    Ok(locale_service.get_translations(&input).await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_handle, app_manager), fields(window = window.label()))]
pub async fn execute_command(
    app_handle: AppHandle,
    app_manager: State<'_, AppManager>,
    window: Window,
    cmd: ReadOnlyStr,
    args: HashMap<String, Value>,
) -> TauriResult<Value> {
    let state_service = app_manager
        .services()
        .get_by_type::<StateService>(&app_handle)
        .await?;

    if let Some(command_handler) = state_service.get_command(&cmd) {
        command_handler(&mut CommandContext::new(app_handle, window, args)).await
    } else {
        Err(TauriError(format!(
            "command with id {} is not found",
            quote!(cmd)
        )))
    }
}
