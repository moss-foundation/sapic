use std::time::{Duration, Instant};

use crate::models::primitives::AccountId;

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
