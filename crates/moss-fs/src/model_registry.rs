use moss_contentmodel::ContentModel;
use std::{collections::HashMap, path::Path, sync::Arc};
use tokio::sync::RwLock;

#[derive(Default)]
struct RegistryState {
    models: HashMap<Arc<Path>, ContentModel>,
}

pub struct GlobalModelRegistry {
    state: Arc<RwLock<RegistryState>>,
}

impl Clone for GlobalModelRegistry {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
        }
    }
}

impl GlobalModelRegistry {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(RegistryState::default())),
        }
    }

    pub async fn get(&self, path: &Path) -> Option<ContentModel> {
        let state = self.state.read().await;
        state.models.get(path).cloned()
    }

    pub async fn insert(&self, path: Arc<Path>, model: ContentModel) {
        let mut state = self.state.write().await;
        state.models.insert(path.clone(), model);
    }

    pub async fn rekey(&self, old_path: &Path, new_path: Arc<Path>) -> Option<()> {
        let mut state = self.state.write().await;
        let model = state.models.remove(old_path)?;
        state.models.insert(new_path, model);

        Some(())
    }

    pub async fn with_model<T>(
        &self,
        path: &Path,
        f: impl FnOnce(&ContentModel) -> T,
    ) -> Option<T> {
        let state = self.state.read().await;
        let model = state.models.get(path)?;
        Some(f(model))
    }

    pub async fn with_model_mut<T>(
        &self,
        path: &Path,
        f: impl FnOnce(&mut ContentModel) -> T,
    ) -> Option<T> {
        let mut state = self.state.write().await;
        let model = state.models.get_mut(path)?;
        Some(f(model))
    }

    pub async fn remove(&self, path: &Path) {
        let mut state = self.state.write().await;
        state.models.remove(path);
    }
}
