use derive_more::Deref;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, sync::Arc};

#[derive(Clone, Debug, PartialEq, Hash, Eq, Deref, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LogEntryId(Arc<String>);
impl LogEntryId {
    pub fn new() -> Self {
        Self(Arc::new(nanoid!(10)))
    }
}

impl From<String> for LogEntryId {
    fn from(s: String) -> Self {
        Self(Arc::new(s))
    }
}

impl AsRef<str> for LogEntryId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Display for LogEntryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
