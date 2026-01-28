use derive_more::Deref;
use moss_environment::Environment;
use moss_fs::FileSystem;
use moss_storage2::KvStorage;
use rustc_hash::{FxHashMap, FxHashSet};
use sapic_base::{
    environment::types::primitives::EnvironmentId, workspace::types::primitives::WorkspaceId,
};
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::RwLock;

use crate::dirs;

#[derive(Clone, Deref)]
struct EnvironmentItem {
    pub _id: EnvironmentId,
    pub _project_id: Option<Arc<String>>,
    pub _order: Option<isize>,

    #[deref]
    pub handle: Arc<Environment>,
}

type EnvironmentMap = HashMap<EnvironmentId, EnvironmentItem>;

struct ServiceState {
    environments: EnvironmentMap,
    _active_environments: HashMap<Arc<String>, EnvironmentId>,
    _groups: FxHashSet<Arc<String>>,
    _expanded_groups: HashSet<Arc<String>>,
    _sources: FxHashMap<Arc<String>, PathBuf>,
}

// DEPRECATED
// This will be removed alongside with old sapic window
pub struct EnvironmentService {
    _abs_path: PathBuf,
    _fs: Arc<dyn FileSystem>,
    state: Arc<RwLock<ServiceState>>,
    _storage: Arc<dyn KvStorage>,
    _workspace_id: WorkspaceId,
}

impl EnvironmentService {
    /// `abs_path` is the absolute path to the workspace directory
    pub async fn new(
        _abs_path: &Path,
        _fs: Arc<dyn FileSystem>,
        _storage: Arc<dyn KvStorage>,
        _workspace_id: WorkspaceId,
        _sources: FxHashMap<Arc<String>, PathBuf>,
    ) -> joinerror::Result<Self> {
        let _abs_path = _abs_path.join(dirs::ENVIRONMENTS_DIR);
        let state = Arc::new(RwLock::new(ServiceState {
            environments: HashMap::new(),
            _active_environments: HashMap::new(),
            _groups: FxHashSet::default(),
            _expanded_groups: HashSet::new(),
            _sources,
        }));

        Ok(Self {
            _fs,
            _abs_path,
            state,
            _storage,
            _workspace_id,
        })
    }

    pub async fn environment(&self, id: &EnvironmentId) -> Option<Arc<Environment>> {
        let state = self.state.read().await;
        state.environments.get(id).map(|item| item.handle.clone())
    }
}
