use joinerror::Error;
use moss_applib::{AppRuntime, ServiceMarker, providers::ServiceMap};
use std::{any::TypeId, path::PathBuf, sync::Arc};

use crate::{
    environment::Environment,
    errors::{ErrorEnvironmentAlreadyExists, ErrorEnvironmentNotFound},
};

pub struct EnvironmentCreateParams {
    pub name: String,
    pub dest_abs_path: PathBuf,
}

pub struct EnvironmentLoadParams {
    pub abs_path: PathBuf,
}

pub struct EnvironmentBuilder {
    services: ServiceMap,
}

impl EnvironmentBuilder {
    pub fn new() -> Self {
        Self {
            services: Default::default(),
        }
    }

    pub fn with_service<T: ServiceMarker + Send + Sync>(
        mut self,
        service: impl Into<Arc<T>>,
    ) -> Self {
        self.services.insert(TypeId::of::<T>(), service.into());
        self
    }

    pub async fn create<R: AppRuntime>(
        self,
        params: EnvironmentCreateParams,
    ) -> joinerror::Result<Environment<R>> {
        let abs_path = params.dest_abs_path.join(params.name);
        debug_assert!(abs_path.is_absolute());

        if abs_path.exists() {
            return Err(Error::new::<ErrorEnvironmentAlreadyExists>(
                abs_path.display().to_string(),
            ));
        }

        // TODO: create file

        Ok(Environment::new(abs_path.into(), self.services.into()))
    }

    pub async fn load<R: AppRuntime>(
        self,
        params: EnvironmentLoadParams,
    ) -> joinerror::Result<Environment<R>> {
        let abs_path = params.abs_path;
        debug_assert!(abs_path.is_absolute());

        if !abs_path.exists() {
            return Err(Error::new::<ErrorEnvironmentNotFound>(
                abs_path.display().to_string(),
            ));
        }

        // TODO: load file

        Ok(Environment::new(abs_path.into(), self.services.into()))
    }
}
