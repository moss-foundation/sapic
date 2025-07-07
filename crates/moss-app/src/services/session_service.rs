use moss_applib::ServiceMarker;
use moss_common::{NanoId, new_nanoid};

pub struct SessionService {
    session_id: NanoId,
}

impl ServiceMarker for SessionService {}

impl SessionService {
    pub fn new() -> Self {
        Self {
            session_id: new_nanoid(),
        }
    }

    pub fn session_id(&self) -> &NanoId {
        &self.session_id
    }
}
