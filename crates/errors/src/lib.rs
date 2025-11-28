//! Global application error types
//!
//! This module contains only universal error types that are relevant across
//! the entire application. Domain-specific errors or errors specific to
//! particular crates should be declared directly in those crates where
//! they are used, not in this global module.
//!
//! Implementing *ResultExt traits in this module is only acceptable for types
//! from external libraries. All other internal, application-level extensions
//! should be implemented in their own dedicated crates.

joinerror::errors! {
    /// The operation was rejected because the system is not in a state required
    /// for the operation's execution. For example, the workspace to be described or
    /// deleted does not opened yet, etc.
    FailedPrecondition => "failed_precondition",

    /// The operation was rejected because the input was invalid.
    InvalidInput => "invalid_input",

    /// The entity that a client attempted to access (e.g., file or directory) does not exist.
    NotFound => "not_found",

    /// The entity that a client attempted to create (e.g., file or directory) already exists.
    AlreadyExists => "already_exists",

    /// The operation was rejected because the resource is not available.
    /// Note that it is not always safe to retry non-idempotent operations.
    Unavailable => "unavailable",

    /// This means that some invariants expected by the underlying system have been broken.
    /// This error is reserved for serious errors.
    Internal => "internal",

    /// This error is reserved for errors that are not covered by the other error codes.
    Unknown => "unknown",

    /// This error means that the operation did not finish within the specified deadline.
    Timeout => "timeout",

    /// The operation was cancelled by the user or system before completion.
    Cancelled => "cancelled",
}
