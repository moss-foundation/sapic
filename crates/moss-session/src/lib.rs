use moss_app::{service::AppService, service_pool::AppService_2};
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

impl AppService for SessionService {
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    fn dispose(&self) {}

    fn as_any(&self) -> &(dyn Any + Send) {
        self
    }
}

impl AppService_2 for SessionService {}
