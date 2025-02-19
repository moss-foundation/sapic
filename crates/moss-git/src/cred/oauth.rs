use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Clone, Serialize, Deserialize)]
pub struct OAuthCred {
    access_token: String,
    // TODO: Change this to `Instant` when not testing
    time_to_refresh: SystemTime,
    refresh_token: String,
}

impl OAuthCred {
    pub fn new(access_token: &str, time_to_refresh: SystemTime, refresh_token: &str) -> OAuthCred {
        OAuthCred {
            access_token: access_token.to_string(),
            time_to_refresh,
            refresh_token: refresh_token.to_string(),
        }
    }

    pub fn access_token(&self) -> String {
        self.access_token.clone()
    }

    pub fn time_to_refresh(&self) -> SystemTime {
        self.time_to_refresh.clone()
    }

    pub fn refresh_token(&self) -> String {
        self.refresh_token.clone()
    }
}
