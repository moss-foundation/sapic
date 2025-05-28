pub mod unit_store;
pub mod variable_store;

pub trait CollectionVariableStore: Send + Sync {}

pub trait CollectionUnitStore: Send + Sync {}
