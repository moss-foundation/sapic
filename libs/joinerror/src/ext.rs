use crate::{Error, ErrorMarker, OptionExt, ResultExt};

impl<T> ResultExt<T> for Result<T, Error> {
    fn join_err<E: ErrorMarker>(self, details: impl Into<String>) -> Result<T, Error> {
        self.map_err(|e| e.join::<E>(details.into()))
    }

    fn join_err_with<E: ErrorMarker>(self, details: impl FnOnce() -> String) -> Result<T, Error> {
        self.map_err(|e| e.join::<E>(details()))
    }
}

impl<T> ResultExt<T> for anyhow::Result<T> {
    fn join_err<E: ErrorMarker>(self, details: impl Into<String>) -> Result<T, Error> {
        self.map_err(|e| Error::new::<()>(e.to_string()).join::<E>(details))
    }

    fn join_err_with<E: ErrorMarker>(self, details: impl FnOnce() -> String) -> Result<T, Error> {
        self.map_err(|e| Error::new::<()>(e.to_string()).join::<E>(details()))
    }
}

impl<T> ResultExt<T> for Result<T, serde_json::Error> {
    fn join_err<E: ErrorMarker>(self, details: impl Into<String>) -> Result<T, Error> {
        self.map_err(|e| Error::new::<()>(e.to_string()).join::<E>(details))
    }

    fn join_err_with<E: ErrorMarker>(self, details: impl FnOnce() -> String) -> Result<T, Error> {
        self.map_err(|e| Error::new::<()>(e.to_string()).join::<E>(details()))
    }
}

// FIXME: Not sure if this is the best place to implement this trait
// Maybe it would be better to be feature-gated
impl<T> ResultExt<T> for tokio::io::Result<T> {
    fn join_err<E: ErrorMarker>(self, details: impl Into<String>) -> Result<T, Error> {
        self.map_err(|e| Error::new::<()>(e.to_string()).join::<E>(details))
    }
    fn join_err_with<E: ErrorMarker>(self, details: impl FnOnce() -> String) -> Result<T, Error> {
        self.map_err(|e| Error::new::<()>(e.to_string()).join::<E>(details()))
    }
}

impl<T> OptionExt<T> for Option<T> {
    fn ok_or_join_err<E: ErrorMarker>(self, details: impl Into<String>) -> crate::Result<T> {
        self.ok_or(Error::new::<E>(details.into()))
    }

    fn ok_or_join_err_with<E: ErrorMarker>(
        self,
        details: impl FnOnce() -> String,
    ) -> crate::Result<T> {
        self.ok_or_else(|| Error::new::<E>(details()))
    }
}
