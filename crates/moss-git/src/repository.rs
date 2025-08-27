use derive_more::Deref;
use git2::{RemoteCallbacks, build::RepoBuilder};
use std::path::Path;

#[derive(Deref)]
pub struct Repository {
    #[deref]
    inner: git2::Repository,
}

impl Repository {
    pub fn clone<'a>(url: &str, into: &Path, cb: RemoteCallbacks<'a>) -> anyhow::Result<Self> {
        let mut opts = git2::FetchOptions::new();
        opts.remote_callbacks(cb);

        let mut builder = RepoBuilder::new();
        builder.fetch_options(opts);

        Ok(Repository {
            inner: builder.clone(url, into)?,
        })
    }
}
