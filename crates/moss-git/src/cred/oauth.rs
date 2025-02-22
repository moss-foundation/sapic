use serde::{Deserialize, Serialize};
use std::time::Instant;


// Since `Instant` is opaque and cannot be serialized
// We will only store the refresh_token when serializing OAuth Credential
// Forcing refreshing of tokens for new sessions
#[derive(Clone, Serialize, Deserialize)]
pub struct OAuthCred {
    #[serde(skip)]
    access_token: Option<String>,
    #[serde(skip)]
    time_to_refresh: Option<Instant>,
    refresh_token: String,
}

impl OAuthCred {
    pub fn new(access_token: Option<&str>, time_to_refresh: Option<Instant>, refresh_token: &str) -> OAuthCred {
        OAuthCred {
            access_token: access_token.map(String::from),
            time_to_refresh,
            refresh_token: refresh_token.to_string(),
        }
    }

    pub fn access_token(&self) -> Option<String> {
        self.access_token.clone()
    }

    pub fn time_to_refresh(&self) -> Option<Instant> {
        self.time_to_refresh.clone()
    }

    pub fn refresh_token(&self) -> String {
        self.refresh_token.clone()
    }
}
