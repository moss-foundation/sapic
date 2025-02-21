use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Clone, Serialize, Deserialize)]
pub struct OAuthCred {
    access_token: String,
    // FIXME: Neither `SystemTime` nor `Instant` seems ideal
    // `SystemTime` is prone to drifting, while `Instant` is opaque
    // An alternative option is to only store refresh_token when storing
    // Forcing the agent to generate a new access_token when a new session starts
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
