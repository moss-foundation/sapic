// use crate::create_child_window;
use tauri::{
    AppHandle, Manager, Runtime as TauriRuntime, Window,
    menu::{Menu, MenuEvent},
};

#[cfg(target_os = "macos")]
use strum::{AsRefStr as StrumAsRefStr, Display as StrumDisplay, EnumString as StrumEnumString};

#[cfg(target_os = "macos")]
#[derive(Debug, StrumEnumString, StrumDisplay, StrumAsRefStr)]
pub enum BuiltInMenuEvent {
    #[strum(serialize = "file.newWindow")]
    NewWindow,
    #[strum(serialize = "file.closeWindow")]
    CloseWindow,
}

#[allow(unused)]
pub fn handle_event<R: TauriRuntime>(_window: &Window<R>, event: &MenuEvent) {
    let event_id = event.id().0.as_str();
    let _app_handle = _window.app_handle().clone();
    match event_id {
        // "file.newWindow" => handle_new_window(app_handle),
        _ => {}
    }
}

#[allow(dead_code)]
pub fn app_menu<R: TauriRuntime>(app_handle: &AppHandle<R>) -> tauri::Result<Menu<R>> {
    #[cfg(not(target_os = "macos"))]
    {
        Menu::new(app_handle)
    }

    #[cfg(target_os = "macos")]
    {
        use tauri::menu::{
            AboutMetadataBuilder, MenuBuilder, MenuItemBuilder, PredefinedMenuItem, SubmenuBuilder,
        };

        unsafe {
            macos_trampoline::set_app_name(&"Moss Studio".into());
        }

        let app_menu = SubmenuBuilder::new(app_handle, "Moss")
            .item(&PredefinedMenuItem::about(
                app_handle,
                Some("About Moss Studio"),
                Some(
                    AboutMetadataBuilder::new()
                        .license(Some(env!("CARGO_PKG_VERSION")))
                        .version(Some(env!("CARGO_PKG_VERSION")))
                        // TODO: .website(Some("https://mossland.dev/"))
                        // TODO: .website_label(Some("mossland.dev.com"))
                        .build(),
                ),
            )?)
            .separator()
            .item(&PredefinedMenuItem::hide(
                app_handle,
                Some("Hide Moss Studio"),
            )?)
            .hide_others()
            .show_all()
            .separator()
            .quit()
            .build()?;

        let edit_menu = SubmenuBuilder::new(app_handle, "Edit")
            .undo()
            .redo()
            .separator()
            .cut()
            .copy()
            .paste()
            .separator()
            .select_all()
            .build()?;

        let window_menu = SubmenuBuilder::new(app_handle, "Window")
            .minimize()
            .item(
                &MenuItemBuilder::with_id(BuiltInMenuEvent::NewWindow, "New Window")
                    .build(app_handle)?,
            )
            .build()?;

        let menu = MenuBuilder::new(app_handle)
            .item(&app_menu)
            .item(&edit_menu)
            .item(&window_menu)
            .build()?;

        Ok(menu)
    }
}
