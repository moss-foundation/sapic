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

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::TypeId;

    #[derive(Debug, Clone, PartialEq)]
    struct TestService {
        pub name: String,
        pub value: u32,
    }

    impl ServiceMarker for TestService {}

    #[derive(Debug, Clone, PartialEq)]
    struct AnotherTestService {
        pub data: Vec<String>,
    }

    impl ServiceMarker for AnotherTestService {}

    #[test]
    fn test_service_provider_new() {
        let provider = ServiceProvider::new();
        assert_eq!(provider.services.len(), 0);
    }

    #[test]
    fn test_service_provider_from_service_map() {
        let mut services = ServiceMap::default();
        let test_service = TestService {
            name: "test".to_string(),
            value: 42,
        };
        services.insert(TypeId::of::<TestService>(), Arc::new(test_service));

        let provider = ServiceProvider::from(services);
        assert_eq!(provider.services.len(), 1);
    }

    #[test]
    fn test_get_service_success() {
        let mut services = ServiceMap::default();
        let test_service = TestService {
            name: "test".to_string(),
            value: 42,
        };
        services.insert(TypeId::of::<TestService>(), Arc::new(test_service.clone()));

        let provider = ServiceProvider::from(services);
        let retrieved_service = provider.get::<TestService>();

        assert_eq!(retrieved_service.name, "test");
        assert_eq!(retrieved_service.value, 42);
        assert_eq!(retrieved_service, &test_service);
    }

    #[test]
    fn test_multiple_services() {
        let mut services = ServiceMap::default();

        let test_service = TestService {
            name: "service1".to_string(),
            value: 100,
        };
        let another_service = AnotherTestService {
            data: vec!["hello".to_string(), "world".to_string()],
        };

        services.insert(TypeId::of::<TestService>(), Arc::new(test_service.clone()));
        services.insert(
            TypeId::of::<AnotherTestService>(),
            Arc::new(another_service.clone()),
        );

        let provider = ServiceProvider::from(services);

        let retrieved_test = provider.get::<TestService>();
        let retrieved_another = provider.get::<AnotherTestService>();

        assert_eq!(retrieved_test, &test_service);
        assert_eq!(retrieved_another, &another_service);
    }

    #[test]
    fn test_arc_and_ref_same_service() {
        let mut services = ServiceMap::default();
        let test_service = TestService {
            name: "same_service".to_string(),
            value: 999,
        };
        services.insert(TypeId::of::<TestService>(), Arc::new(test_service.clone()));

        let provider = ServiceProvider::from(services);
        let service_ref = provider.get::<TestService>();
        let service = provider.get::<TestService>();

        assert_eq!(service_ref, &*service);
        assert_eq!(service_ref, &test_service);
        assert_eq!(*service, test_service);
    }

    #[test]
    #[should_panic(
        expected = "Service moss_applib::providers::tests::TestService must be registered before it can be used"
    )]
    fn test_get_unregistered_service_panics() {
        let provider = ServiceProvider::new();
        let _ = provider.get::<TestService>();
    }
}
