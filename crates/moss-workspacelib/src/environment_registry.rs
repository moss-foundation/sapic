use std::{collections::HashMap, marker::PhantomData, sync::Arc};

use derive_more::Deref;
use moss_applib::AppRuntime;
use tokio::sync::{RwLock, watch};

use moss_environment::{AnyEnvironment, models::primitives::EnvironmentId};

#[derive(Deref)]
pub struct EnvironmentModel<R, Environment>
where
    R: AppRuntime,
    Environment: AnyEnvironment<R>,
{
    pub id: EnvironmentId,
    pub collection_id: Option<Arc<String>>,

    #[deref]
    pub handle: Arc<Environment>,
    pub _runtime: PhantomData<R>,
}

impl<R, Environment> Clone for EnvironmentModel<R, Environment>
where
    R: AppRuntime,
    Environment: AnyEnvironment<R>,
{
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            collection_id: self.collection_id.clone(),
            handle: self.handle.clone(),
            _runtime: PhantomData,
        }
    }
}

unsafe impl<R, Environment> Send for EnvironmentModel<R, Environment>
where
    R: AppRuntime,
    Environment: AnyEnvironment<R>,
{
}

unsafe impl<R, Environment> Sync for EnvironmentModel<R, Environment>
where
    R: AppRuntime,
    Environment: AnyEnvironment<R>,
{
}

type EnvironmentMap<R, Environment> = HashMap<EnvironmentId, EnvironmentModel<R, Environment>>;

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

    pub async fn get(&self, id: &EnvironmentId) -> Option<EnvironmentModel<R, Environment>> {
        let state = self.state.read().await;
        state.get(id).cloned()
    }

    pub async fn insert(&self, item: EnvironmentModel<R, Environment>) {
        let mut state = self.state.write().await;
        state.insert(item.id.clone(), item);

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
