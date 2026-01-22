use moss_fs::FileSystem;
use moss_storage2::KvStorage;
use sapic_base::environment::types::primitives::EnvironmentId;
use std::sync::Arc;

use crate::AnyEnvironment;

// FIXME: I'm actually not sure what this structure is supposed to hold now
// Since we have extracted file system and storage logic into environment services
// It looks like fs, storage and abs_path can all be gone
// Maybe it can hold an in-memory cache of the variables? Not sure
pub struct Environment {
    pub(super) _id: EnvironmentId,
    pub(super) _fs: Arc<dyn FileSystem>,
    pub(super) _storage: Arc<dyn KvStorage>,
    pub(super) _workspace_id: Arc<String>,
}

unsafe impl Send for Environment {}
unsafe impl Sync for Environment {}

impl AnyEnvironment for Environment {}
