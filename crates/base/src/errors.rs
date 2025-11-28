joinerror::errors! {
    /// The entity that a client attempted to access (e.g., file or directory) does not exist.
    NotFound => "not_found",

    /// The entity that a client attempted to create (e.g., file or directory) already exists.
    AlreadyExists => "already_exists",

    /// This means that some invariants expected by the underlying system have been broken.
    /// This error is reserved for serious errors.
    Internal => "internal",

    /// This error is reserved for errors that are not covered by the other error codes.
    Unknown => "unknown",
}
