// TODO: Implement other git functionalities
// e.g. branching, merging, conflict resolution
use anyhow::Result;
use std::sync::Arc;
use git2::RemoteCallbacks;

pub mod repo;
// TODO: We will use more secure method of storing the AuthAgent info
// For easy testing, we will use environment variables for now
pub trait TestStorage {

    fn write_to_file(&self) -> Result<()>;
    fn read_from_file() -> Result<Arc<Self>>;
}


// TODO: This `AuthAgent` is exclusively used for git operations
// We might have auth agent handling provider API, for example
pub trait GitAuthAgent {
    fn generate_callback<'a>(&'a self, cb: &mut RemoteCallbacks<'a>) -> Result<()>;
}
