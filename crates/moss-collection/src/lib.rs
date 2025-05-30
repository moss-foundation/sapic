pub mod api;
pub mod collection;
pub mod config;
pub mod manifest;
pub mod models;
pub mod tokens;
pub mod worktree;

pub use collection::Collection;

pub mod constants {
    pub(crate) const FILENAME_FOLDER_SPECIFICATION: &'static str = "folder.sapic";

    // pub(crate) const GET_SPECFILE: &'static str = "get.sapic";
    // pub(crate) const PUT_SPECFILE: &'static str = "put.sapic";
    // pub(crate) const DELETE_SPECFILE: &'static str = "del.sapic";
    // pub(crate) const POST_SPECFILE: &'static str = "post.sapic";
    // pub(crate) const FOLDER_SPECFILE: &'static str = "folder.sapic";
}

mod defaults {
    pub(crate) const DEFAULT_COLLECTION_NAME: &str = "New Collection";
}

pub mod dirs {
    pub const COLLECTIONS_DIR: &str = "requests";
    pub const ENVIRONMENTS_DIR: &str = "environments";
}
