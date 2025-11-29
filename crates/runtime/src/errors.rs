joinerror::errors! {
    /// The operation was rejected because the resource is not available.
    /// Note that it is not always safe to retry non-idempotent operations.
    Unavailable => "unavailable",

    /// The operation was rejected because the system is not in a state required
    /// for the operation's execution. For example, the workspace to be described or
    /// deleted does not opened yet, etc.
    FailedPrecondition => "failed_precondition",
}
