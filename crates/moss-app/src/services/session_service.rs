use crate::constants::ID_LENGTH;
use moss_applib::ServiceMarker;

pub struct SessionService {
    session_id: String,
}

impl ServiceMarker for SessionService {}

impl SessionService {
    pub fn new() -> Self {
        Self {
            session_id: nanoid::nanoid!(ID_LENGTH),
        }
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }
}
