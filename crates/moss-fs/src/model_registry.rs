use moss_patch::Model;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[derive(Default)]
struct RegistryState {
    models: HashMap<String, Model>,
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

    pub async fn get(&self, uri: &str) -> Option<Model> {
        let state = self.state.read().await;
        state.models.get(uri).cloned()
    }

    pub async fn add(&self, uri: String, model: Model) {
        let mut state = self.state.write().await;
        state.models.insert(uri, model);
    }

    pub async fn with_model<T>(&self, uri: &str, f: impl FnOnce(&Model) -> T) -> Option<T> {
        let state = self.state.read().await;
        let model = state.models.get(uri)?;
        Some(f(model))
    }

    pub async fn with_model_mut<T>(&self, uri: &str, f: impl FnOnce(&mut Model) -> T) -> Option<T> {
        let mut state = self.state.write().await;
        let model = state.models.get_mut(uri)?;
        Some(f(model))
    }

    pub async fn remove(&self, uri: &str) {
        let mut state = self.state.write().await;
        state.models.remove(uri);
    }
}
