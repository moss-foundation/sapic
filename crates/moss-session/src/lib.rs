use moss_app::service::AppService;
use std::any::Any;
use uuid::Uuid;

pub struct SessionService {
    uuid: Uuid,
}

impl SessionService {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
        }
    }

    pub fn get_session_uuid(&self) -> String {
        self.uuid.to_string()
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
