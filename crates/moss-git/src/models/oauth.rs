use crate::models::oauth::OAuthCred::{WithExpiration, WithoutExpiration};
use serde::{Deserialize, Serialize};
use std::time::Instant;
// Since `Instant` is opaque and cannot be serialized
// We will only store the refresh_token when serializing OAuth Credential
// Forcing refreshing of tokens for new sessions

#[derive(Clone, Serialize, Deserialize)]
pub enum OAuthCred {
    WithExpiration {
        #[serde(skip)]
        access_token: Option<String>,
        #[serde(skip)]
        time_to_refresh: Option<Instant>,
        refresh_token: String,
    },
    WithoutExpiration {
        access_token: String,
    },
}

impl OAuthCred {
    pub fn with_expiration(
        access_token: Option<&str>,
        time_to_refresh: Option<Instant>,
        refresh_token: &str,
    ) -> Self {
        WithExpiration {
            access_token: access_token.map(String::from),
            time_to_refresh,
            refresh_token: refresh_token.to_string(),
        }
    }

    pub fn without_expiration(access_token: &str) -> Self {
        WithoutExpiration {
            access_token: access_token.to_string(),
        }
    }

    pub fn refresh_token(&self) -> Option<String> {
        match self {
            OAuthCred::WithExpiration { refresh_token, .. } => Some(refresh_token.clone()),
            OAuthCred::WithoutExpiration { .. } => None,
        }
    }

    pub fn access_token(&self) -> Option<String> {
        match self {
            OAuthCred::WithExpiration { access_token, .. } => access_token.clone(),
            OAuthCred::WithoutExpiration { access_token, .. } => Some(access_token.clone()),
        }
    }

    pub fn time_to_refresh(&self) -> Option<Instant> {
        match self {
            OAuthCred::WithExpiration {
                time_to_refresh, ..
            } => time_to_refresh.clone(),
            OAuthCred::WithoutExpiration { .. } => None,
        }
    }
}
