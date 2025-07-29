use moss_applib::{AppRuntime, ServiceMarker};
use std::{collections::HashMap, sync::Arc};

use crate::{
    models::{
        primitives::VariableId,
        types::{VariableKind, VariableName, VariableValue},
    },
    services::storage_service::StorageService,
};

#[derive(Debug, Clone)]
pub struct VariableItemParams {
    pub disabled: bool,
}

#[derive(Debug, Clone)]
pub struct VariableItem {
    pub id: VariableId,
    pub kind: Option<VariableKind>,
    pub global_value: Option<VariableValue>,
    pub desc: Option<String>,
    pub params: VariableItemParams,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct ServiceState {
    variables: HashMap<VariableName, VariableItem>,
}

pub struct VariableService<R: AppRuntime> {
    #[allow(dead_code)]
    storage_service: Arc<StorageService<R>>,
}

impl<R: AppRuntime> ServiceMarker for VariableService<R> {}

impl<R: AppRuntime> VariableService<R> {
    pub fn new(storage_service: Arc<StorageService<R>>) -> Self {
        Self { storage_service }
    }
}
