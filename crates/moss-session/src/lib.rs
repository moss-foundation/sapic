pub struct SessionService {
    uuid: String,
}

impl SessionService {
    pub fn init() -> SessionService {
        SessionService {
            uuid: uuid::Uuid::new_v4().to_string(),
        }
    }

    pub fn get_session_uuid(&self) -> String {
        self.uuid.clone()
    }
}
