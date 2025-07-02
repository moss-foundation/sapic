use moss_applib::ServiceMarker;
use uuid::Uuid;

pub struct SessionService {
    session_id: Uuid,
}

impl ServiceMarker for SessionService {}

impl SessionService {
    pub fn new() -> Self {
        Self {
            session_id: Uuid::new_v4(),
        }
    }

    pub fn session_id(&self) -> &Uuid {
        &self.session_id
    }
}
