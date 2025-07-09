pub mod defaults;
pub mod error;
pub mod ext;

pub use error::Error;

use crate::error::ErrorMarker;

pub type Result<T> = std::result::Result<T, Error>;

pub trait ResultExt<T> {
    fn join_err<E: ErrorMarker>(self, details: impl Into<String>) -> Result<T>;
}
