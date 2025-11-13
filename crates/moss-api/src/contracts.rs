pub mod other;
pub mod theme;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Configuration options for API operations.
///
/// This struct provides configurable parameters that affect the behavior of API operations,
/// particularly timeout handling for asynchronous operations. It is commonly used with
/// the `with_timeout` utility function to wrap async operations with configurable timeouts.
///
/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct Options {
    /// Optional request ID for API operations.
    pub request_id: Option<String>,

    /// Optional timeout duration for API operations in seconds.
    ///
    /// When `Some(value)`, the operation will timeout after the specified number of seconds.
    /// When `None`, the operation will use the default timeout of 30 seconds
    /// (defined by `DEFAULT_OPERATION_TIMEOUT`).
    ///
    /// # Frontend Note
    ///
    /// The timeout value is converted to a `Duration` internally using `Duration::from_secs()`,
    /// so the minimum granularity is 1 second.
    pub timeout: Option<u64>,
}
