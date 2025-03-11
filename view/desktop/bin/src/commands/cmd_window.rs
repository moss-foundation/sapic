use crate::{create_child_window, menu};
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
    manager::AppStateManager,
    models::{
        operations::DescribeAppStateOutput,
        types::{Defaults, Preferences},
    },
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

// According to https://docs.rs/tauri/2.1.1/tauri/webview/struct.WebviewWindowBuilder.html
// We should call WebviewWindowBuilder from async commands
#[tauri::command(async)]
#[instrument(level = "trace", skip(app_handle))]
pub async fn create_new_window(app_handle: AppHandle) -> TauriResult<()> {
    let webview_window = create_child_window(&app_handle, "/")?;
    webview_window.on_menu_event(move |window, event| menu::handle_event(window, &event));
    Ok(())
}

#[tauri::command]
#[instrument(level = "trace", skip(app_handle, state_manager), fields(window = window.label()))]
pub fn change_color_theme(
    app_handle: AppHandle,
    state_manager: State<'_, AppStateManager>,
    window: Window,
    descriptor: ThemeDescriptor,
) -> TauriResult<()> {
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

    state_manager.set_color_theme(descriptor);

    Ok(())
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager))]
pub async fn get_color_theme(
    app_manager: State<'_, AppManager>,
    id: ThemeId,
) -> TauriResult<String> {
    Ok(app_manager
        .service::<ThemeService>()?
        .read_color_theme(&id)
        .await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager))]
pub async fn list_themes(app_manager: State<'_, AppManager>) -> TauriResult<ListThemesOutput> {
    Ok(app_manager.service::<ThemeService>()?.list_themes().await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(state_manager))]
pub fn describe_app_state(
    state_manager: State<'_, AppStateManager>,
) -> Result<DescribeAppStateOutput, String> {
    Ok(DescribeAppStateOutput {
        preferences: Preferences {
            theme: state_manager.preferences.theme.read().clone(),
            locale: state_manager.preferences.locale.read().clone(),
        },
        defaults: Defaults {
            theme: state_manager.defaults.theme.clone(),
            locale: state_manager.defaults.locale.clone(),
        },
    })
}

#[tauri::command]
#[instrument(level = "trace", skip(state_manager))]
pub fn change_language_pack(
    state_manager: State<'_, AppStateManager>,
    descriptor: LocaleDescriptor,
) -> TauriResult<()> {
    state_manager.set_language_pack(descriptor);

    Ok(())
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager))]
pub async fn list_locales(app_manager: State<'_, AppManager>) -> TauriResult<ListLocalesOutput> {
    let locale_service = app_manager.service::<LocaleService>()?;

    Ok(locale_service.list_locales().await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager))]
pub async fn get_translations(
    app_manager: State<'_, AppManager>,
    input: GetTranslationsInput,
) -> TauriResult<JsonValue> {
    let locale_service = app_manager.service::<LocaleService>()?;

    Ok(locale_service.get_translations(&input).await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_handle, app_state), fields(window = window.label()))]
pub async fn execute_command(
    app_handle: AppHandle,
    app_state: State<'_, AppStateManager>,
    window: Window,
    cmd: ReadOnlyStr,
    args: HashMap<String, Value>,
) -> TauriResult<Value> {
    if let Some(command_handler) = app_state.get_command(&cmd) {
        command_handler(
            &mut CommandContext::new(app_handle, window, args),
            &app_state,
        )
        .await
    } else {
        Err(TauriError(format!(
            "command with id {} is not found",
            quote!(cmd)
        )))
    }
}
