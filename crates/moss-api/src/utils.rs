use crate::{constants::DEFAULT_OPERATION_TIMEOUT, models::types::Options};
use std::time::Duration;
use tokio::time::Timeout;

pub fn with_timeout<F>(options: Option<Options>, future: F) -> Timeout<F::IntoFuture>
where
    F: IntoFuture,
{
    let duration = if let Some(opts) = options {
        opts.timeout.map_or(DEFAULT_OPERATION_TIMEOUT, |timeout| {
            Duration::from_secs(timeout)
        })
    } else {
        DEFAULT_OPERATION_TIMEOUT
    };

    tokio::time::timeout(duration, future)
}
