use moss_contentmodel::ContentModel;
use std::{collections::HashMap, path::Path, sync::Arc};
use tokio::sync::RwLock;

pub struct GlobalModelRegistry {
    state: RwLock<HashMap<Arc<Path>, ContentModel>>,
}

impl GlobalModelRegistry {
    pub fn new() -> Self {
        Self {
            state: RwLock::new(HashMap::new()),
        }
    }

    pub async fn get(&self, path: &Path) -> Option<ContentModel> {
        let state = self.state.read().await;
        state.get(path).cloned()
    }

    pub async fn insert(&self, path: Arc<Path>, model: ContentModel) {
        let mut state = self.state.write().await;
        state.insert(path.clone(), model);
    }

    pub async fn rekey(&self, old_path: &Path, new_path: Arc<Path>) -> Option<()> {
        let mut state = self.state.write().await;
        let model = state.remove(old_path)?;
        state.insert(new_path, model);

        Some(())
    }

    pub async fn with_model<T>(
        &self,
        path: &Path,
        f: impl FnOnce(&ContentModel) -> T,
    ) -> Option<T> {
        let state = self.state.read().await;
        let model = state.get(path)?;
        Some(f(model))
    }

    pub async fn with_model_mut<T>(
        &self,
        path: &Path,
        f: impl FnOnce(&mut ContentModel) -> T,
    ) -> Option<T> {
        let mut state = self.state.write().await;
        let model = state.get_mut(path)?;
        Some(f(model))
    }

    pub async fn remove(&self, path: &Path) {
        let mut state = self.state.write().await;
        state.remove(path);
    }
}
