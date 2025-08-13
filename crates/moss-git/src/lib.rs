// TODO: Implement other git functionalities
// e.g. branching, merging, conflict resolution
use anyhow::Result;
use git2::RemoteCallbacks;

pub mod repo;
pub mod url;

pub trait GitAuthAgent {
    fn generate_callback<'a>(&'a self, cb: &mut RemoteCallbacks<'a>) -> Result<()>;
}

pub mod constants {
    pub const DEFAULT_REMOTE_NAME: &str = "origin";
}
