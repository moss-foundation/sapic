// TODO: temporary left here for now, should moved to window crate

use sapic_window::types::primitives::SessionId;

pub struct SessionService {
    session_id: SessionId,
}

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
