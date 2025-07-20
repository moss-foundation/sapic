pub mod variable_store;

use moss_applib::context::AnyAsyncContext;

pub trait VariableStore<Context: AnyAsyncContext>: Send + Sync {}
