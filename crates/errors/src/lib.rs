// //! Global application error types
// //!
// //! This module contains only universal error types that are relevant across
// //! the entire application. Domain-specific errors or errors specific to
// //! particular crates should be declared directly in those crates where
// //! they are used, not in this global module.
// //!
// //! Implementing *ResultExt traits in this module is only acceptable for types
// //! from external libraries. All other internal, application-level extensions
// //! should be implemented in their own dedicated crates.

// joinerror::errors! {
//     /// The entity that a client attempted to access (e.g., file or directory) does not exist.
//     NotFound => "not_found",

//     /// The entity that a client attempted to create (e.g., file or directory) already exists.
//     AlreadyExists => "already_exists",

//     /// This means that some invariants expected by the underlying system have been broken.
//     /// This error is reserved for serious errors.
//     Internal => "internal",

//     /// This error is reserved for errors that are not covered by the other error codes.
//     Unknown => "unknown",
// }
