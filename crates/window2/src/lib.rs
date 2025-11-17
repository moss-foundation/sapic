use async_trait::async_trait;
use moss_applib::context::Canceller;
use tauri::Runtime as TauriRuntime;

pub mod constants {
    pub const MIN_WINDOW_WIDTH: f64 = 800.0;
    pub const MIN_WINDOW_HEIGHT: f64 = 600.0;
}

pub mod defaults {
    pub const DEFAULT_WINDOW_POSITION_X: f64 = 100.0;
    pub const DEFAULT_WINDOW_POSITION_Y: f64 = 100.0;
}

#[async_trait]
pub trait AppWindowApi: Send + Sync + 'static {
    async fn track_cancellation(&self, request_id: &str, canceller: Canceller) -> ();
    async fn release_cancellation(&self, request_id: &str) -> ();
}

/// This is a wrapper around `tauri::WebviewWindow` to provide access to window
/// management functionality, but with restrictions on controlling its lifecycle.
/// The wrapper is meant to be used inside a custom application window controller
/// (e.g., WelcomeWindow, MainWindow, etc.). The idea is that this wrapper prevents
/// window controllers from, for example, closing the window themselves such actions
/// should only be performed by a higher-level window management system. All other
/// methods that are not related to lifecycle interruption and do not affect
/// other windows are okay to expose as public methods.
///
/// The `external` feature implies that this handle is used outside the window
/// controller crate, and in that case, we can allow any window operations that
///  Tauri provides.

#[cfg_attr(feature = "external", derive(derive_more::Deref))]
pub struct WindowHandle<R: TauriRuntime> {
    #[cfg(feature = "external")]
    #[deref]
    inner: tauri::WebviewWindow<R>,

    #[cfg(not(feature = "external"))]
    inner: tauri::WebviewWindow<R>,
}

impl<R: TauriRuntime> Clone for WindowHandle<R> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<R: TauriRuntime> WindowHandle<R> {
    pub fn new(window: tauri::WebviewWindow<R>) -> Self {
        Self { inner: window }
    }

    pub fn set_focus(&self) -> Result<(), tauri::Error> {
        self.inner.set_focus()
    }
}
