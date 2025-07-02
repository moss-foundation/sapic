use moss_applib::ServiceMarker;
use moss_common::nanoid::new_nanoid;

pub struct SessionService {
    session_id: String,
}

impl ServiceMarker for SessionService {}

impl SessionService {
    pub fn new() -> Self {
        Self {
            session_id: new_nanoid(),
        }
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }
}
