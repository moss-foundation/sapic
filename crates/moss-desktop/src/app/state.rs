use std::sync::Arc;
use std::time::Duration;

const STATE_CACHE_TTL: Duration = Duration::from_secs(60 * 3);
const STATE_MAX_CAPACITY: u64 = 100;

#[derive(Clone)]
pub struct Preferences {
    //TODO
}

#[derive(Clone)]
pub struct AppDefaults {
    //TODO
}

pub struct AppStateManager {
    pub preferences: Preferences,
    pub defaults: AppDefaults,
}

impl AppStateManager {
    pub fn new() -> Self {
        Self {
            preferences: Preferences {},
            defaults: AppDefaults {},
        }
    }
}
