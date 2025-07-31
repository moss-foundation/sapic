use std::{collections::HashMap, marker::PhantomData, sync::Arc};

use moss_applib::AppRuntime;
use tokio::sync::{RwLock, watch};

use crate::{AnyEnvironment, models::primitives::EnvironmentId};

pub struct EnvironmentItem<R, Environment>
where
    R: AppRuntime,
    Environment: AnyEnvironment<R>,
{
    id: EnvironmentId,
    name: String,
    display_name: String,
    handle: Arc<Environment>,
    _runtime: PhantomData<R>,
}

impl<R, Environment> Clone for EnvironmentItem<R, Environment>
where
    R: AppRuntime,
    Environment: AnyEnvironment<R>,
{
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            name: self.name.clone(),
            display_name: self.display_name.clone(),
            handle: self.handle.clone(),
            _runtime: PhantomData,
        }
    }
}

unsafe impl<R, Environment> Send for EnvironmentItem<R, Environment>
where
    R: AppRuntime,
    Environment: AnyEnvironment<R>,
{
}

unsafe impl<R, Environment> Sync for EnvironmentItem<R, Environment>
where
    R: AppRuntime,
    Environment: AnyEnvironment<R>,
{
}

type EnvironmentMap<R, Environment> = HashMap<EnvironmentId, EnvironmentItem<R, Environment>>;

pub struct GlobalEnvironmentRegistry<R, Environment>
where
    R: AppRuntime,
    Environment: AnyEnvironment<R>,
{
    state: RwLock<EnvironmentMap<R, Environment>>,
    tx: watch::Sender<EnvironmentMap<R, Environment>>,
}

impl<R, Environment> GlobalEnvironmentRegistry<R, Environment>
where
    R: AppRuntime,
    Environment: AnyEnvironment<R>,
{
    pub fn new() -> Self {
        let (tx, _) = watch::channel(HashMap::new());
        Self {
            state: RwLock::new(HashMap::new()),
            tx,
        }
    }

    pub async fn get(&self, id: EnvironmentId) -> Option<EnvironmentItem<R, Environment>> {
        let state = self.state.read().await;
        state.get(&id).cloned()
    }

    pub async fn add(&self, id: EnvironmentId, item: EnvironmentItem<R, Environment>) {
        let mut state = self.state.write().await;
        state.insert(id, item);

        let _ = self.tx.send(state.clone());
    }

    pub async fn remove(&self, id: EnvironmentId) {
        let mut state = self.state.write().await;
        state.remove(&id);

        let _ = self.tx.send(state.clone());
    }

    pub async fn watch(&self) -> watch::Receiver<EnvironmentMap<R, Environment>> {
        self.tx.subscribe()
    }
}
