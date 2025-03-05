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
