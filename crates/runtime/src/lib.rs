pub mod app;
pub mod app_delegate;
pub mod errors;
pub mod extension_point;
pub mod globals;
pub mod user_settings;

use joinerror::error::ErrorMarker;

// type TauriResult<T> = Result<T, tauri::Error>;

// pub trait TauriResultExt<T> {
//     fn join_err<E: ErrorMarker>(self, details: impl Into<String>) -> joinerror::Result<T>;
//     fn join_err_with<E: ErrorMarker>(
//         self,
//         details: impl FnOnce() -> String,
//     ) -> joinerror::Result<T>;
//     fn join_err_bare(self) -> joinerror::Result<T>;
// }

// impl<T> TauriResultExt<T> for TauriResult<T> {
//     fn join_err<E: ErrorMarker>(self, details: impl Into<String>) -> joinerror::Result<T> {
//         self.map_err(|e| joinerror::Error::new::<Internal>(e.to_string()).join::<E>(details))
//     }

//     fn join_err_with<E: ErrorMarker>(
//         self,
//         details: impl FnOnce() -> String,
//     ) -> joinerror::Result<T> {
//         self.map_err(|e| joinerror::Error::new::<Internal>(e.to_string()).join_with::<E>(details))
//     }

//     fn join_err_bare(self) -> joinerror::Result<T> {
//         self.map_err(|e| joinerror::Error::new::<Internal>(e.to_string()))
//     }
// }
