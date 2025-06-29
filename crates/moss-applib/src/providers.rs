use rustc_hash::FxHashMap;
use std::{
    any::{Any, TypeId},
    sync::Arc,
};

use crate::ServiceMarker;

pub type Service = Arc<dyn Any + Send + Sync>;
pub type ServiceMap = FxHashMap<TypeId, Service>;

pub struct ServiceProvider {
    services: ServiceMap,
}

impl From<ServiceMap> for ServiceProvider {
    fn from(services: ServiceMap) -> Self {
        Self { services }
    }
}

impl ServiceProvider {
    pub fn new() -> Self {
        Self {
            services: FxHashMap::default(),
        }
    }

    pub fn get<T: ServiceMarker>(&self) -> &T {
        let type_id = TypeId::of::<T>();
        let service = self.services.get(&type_id).expect(&format!(
            "Service {} must be registered before it can be used",
            std::any::type_name::<T>()
        ));

        service.downcast_ref::<T>().expect(&format!(
            "Service {} is registered with the wrong type type id",
            std::any::type_name::<T>()
        ))
    }
}
