use crate::{Error, ErrorMarker, ResultExt};

impl<T> ResultExt<T> for Result<T, Error> {
    fn join_err<E: ErrorMarker>(self, details: impl Into<String>) -> Result<T, Error> {
        self.map_err(|e| e.join::<E>(details.into()))
    }
}

impl<T> ResultExt<T> for anyhow::Result<T> {
    fn join_err<E: ErrorMarker>(self, details: impl Into<String>) -> Result<T, Error> {
        self.map_err(|e| Error::new::<()>(e.to_string()).join::<E>(details))
    }
}
