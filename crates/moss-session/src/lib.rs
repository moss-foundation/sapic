use moss_app::service_pool::AppService;
use std::any::Any;
use uuid::Uuid;

pub struct SessionService {
    session_id: Uuid,
}

impl SessionService {
    pub fn new() -> Self {
        Self {
            session_id: Uuid::new_v4(),
        }
    }

    pub fn get_session_uuid(&self) -> &Uuid {
        &self.session_id
    }
}

impl AppService for SessionService {}
