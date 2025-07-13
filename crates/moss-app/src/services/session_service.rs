use moss_applib::{ServiceMarker, context_old::ContextValue};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, sync::Arc};

#[derive(Clone, Debug, PartialEq, Hash, Eq, Deref, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SessionId(Arc<String>);
impl SessionId {
    pub fn new() -> Self {
        Self(Arc::new(nanoid!(10)))
    }
}

impl From<String> for SessionId {
    fn from(s: String) -> Self {
        Self(Arc::new(s))
    }
}

impl AsRef<str> for SessionId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ContextValue for SessionId {}

pub struct SessionService {
    session_id: SessionId,
}

impl ServiceMarker for SessionService {}

impl SessionService {
    pub fn new() -> Self {
        Self {
            session_id: SessionId::new(),
        }
    }

    pub fn session_id(&self) -> &SessionId {
        &self.session_id
    }
}
