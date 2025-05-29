mod builder;
mod pool;

pub mod prelude {
    pub use super::{
        builder::{Instantiation, ServicePoolBuilder},
        pool::{AppService, ServiceKey, ServicePool, ServicePoolError},
    };
}
