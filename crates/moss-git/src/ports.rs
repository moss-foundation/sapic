use anyhow::Result;
use git2::RemoteCallbacks;

pub trait AuthAgent {
    fn generate_callback<'a>(&'a self, cb: &mut RemoteCallbacks<'a>) -> Result<()>;
}
