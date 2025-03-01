// TODO: Implement other git functionalities
// e.g. branching, merging, conflict resolution
use anyhow::Result;
use git2::RemoteCallbacks;
use std::sync::Arc;

pub mod repo;
// TODO: We will use more secure method of storing the AuthAgent info
// For easy testing, we will use environment variables for now
pub trait TestStorage {
    fn write_to_file(&self) -> Result<()>;
    fn read_from_file() -> Result<Arc<Self>>;
}

pub trait GitAuthAgent {
    fn generate_callback<'a>(&'a self, cb: &mut RemoteCallbacks<'a>) -> Result<()>;
}
