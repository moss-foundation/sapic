pub mod github_session;
pub mod gitlab_session;
pub mod session;

use chrono::{DateTime, Utc};
use sapic_base::user::types::{
    AccountInfo, AccountMetadata,
    primitives::{AccountId, AccountKind},
};
use sapic_core::context::AnyAsyncContext;
use std::time::{Duration, Instant};

use super::account::session::AccountSession;

pub fn make_secret_key(prefix: &str, host: &str, account_id: &AccountId) -> String {
    format!("{prefix}:{host}:{account_id}")
}

pub fn calc_expires_at(duration: Duration) -> Instant {
    Instant::now()
        .checked_add(duration)
        .unwrap()
        .checked_sub(Duration::from_secs(30 * 60))
        .unwrap()
}

#[derive(Clone)]
pub(crate) struct Metadata {
    pub(crate) expires_at: Option<DateTime<Utc>>,
}

pub struct Account {
    pub(crate) id: AccountId,
    pub(crate) username: String,
    pub(crate) host: String,
    pub(crate) session: AccountSession,
    pub(crate) kind: AccountKind,
    pub(crate) metadata: Metadata,
}

impl Clone for Account {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            username: self.username.clone(),
            host: self.host.clone(),
            session: self.session.clone(),
            kind: self.kind.clone(),
            metadata: self.metadata.clone(),
        }
    }
}

impl Account {
    pub fn new(
        id: AccountId,
        username: String,
        host: String,
        session: AccountSession,
        kind: AccountKind,
        expires_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            username,
            host,
            session,
            kind,
            metadata: Metadata { expires_at },
        }
    }

    pub fn id(&self) -> AccountId {
        self.id.clone()
    }

    pub fn session(&self) -> &AccountSession {
        &self.session
    }

    pub fn username(&self) -> String {
        self.username.clone()
    }

    pub fn host(&self) -> String {
        self.host.clone()
    }

    pub fn kind(&self) -> AccountKind {
        self.kind.clone()
    }

    pub fn info(&self) -> AccountInfo {
        AccountInfo {
            id: self.id.clone(),
            username: self.username.clone(),
            host: self.host.clone(),
            kind: self.kind.clone(),
            method: self.session.session_kind().into(),
            metadata: AccountMetadata {
                pat_expires_at: self.metadata.expires_at,
            },
        }
    }

    pub async fn revoke(&self, ctx: &dyn AnyAsyncContext) -> joinerror::Result<()> {
        self.session.revoke(ctx).await
    }

    // Update PAT and returns the old PAT
    // If the new PAT belongs to a different account or does not exist, revert the change
    pub async fn update_pat(
        &self,
        ctx: &dyn AnyAsyncContext,
        pat: &str,
    ) -> joinerror::Result<String> {
        let old_pat = self.session.token(ctx).await?;
        self.session.update_pat(pat).await?;
        Ok(old_pat)
    }
}
