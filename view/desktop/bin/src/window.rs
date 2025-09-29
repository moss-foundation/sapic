use tauri::{AppHandle, Runtime as TauriRuntime, WebviewUrl, WebviewWindow};

#[cfg(target_os = "windows")]
use wry::WebViewBuilderExtWindows;

use crate::{MIN_WINDOW_HEIGHT, MIN_WINDOW_WIDTH};

#[derive(Debug)]
pub struct CreateWindowInput<'a> {
    pub url: &'a str,
    pub label: &'a str,
    pub title: &'a str,
    pub inner_size: (f64, f64),
    pub position: (f64, f64),

    // Optional scroll bar style for Windows
    #[cfg(target_os = "windows")]
    pub scroll_bar_style: Option<wry::ScrollBarStyle>,
}

#[instrument(level = "debug", skip(app_handle))]
pub fn create_window<R: TauriRuntime>(
    app_handle: &AppHandle<R>,
    input: CreateWindowInput<'_>,
) -> WebviewWindow<R> {
    let win_builder = tauri::WebviewWindowBuilder::new(
        app_handle,
        input.label,
        WebviewUrl::App(input.url.into()),
    )
    .title(input.title)
    .center()
    .resizable(true)
    .visible(false)
    .disable_drag_drop_handler()
    .inner_size(input.inner_size.0, input.inner_size.1)
    .position(input.position.0, input.position.1)
    .min_inner_size(MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT);

    #[cfg(target_os = "windows")]
    let win_builder = {
        let mut builder = win_builder
            .transparent(false)
            .shadow(true)
            .decorations(false);

        // Apply scroll bar style if specified
        if let Some(scroll_style) = input.scroll_bar_style {
            builder = builder.with_scroll_bar_style(scroll_style);
        }

        builder
    };

    #[cfg(target_os = "macos")]
    let win_builder = win_builder
        .hidden_title(true)
        .title_bar_style(tauri::TitleBarStyle::Overlay)
        .transparent(false)
        .decorations(true);

    let webview_window = win_builder
        .build()
        .map_err(|e| format!("Failed to build window: {}", e))
        .unwrap();

    if let Err(err) = webview_window.set_focus() {
        warn!(
            "Failed to set focus to window {} when creating it: {}",
            input.label, err
        );
    }

    webview_window
}
