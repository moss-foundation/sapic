mod builder;
mod pool;

pub mod prelude {
    pub use super::builder::{Instantiation, ServicePoolBuilder};
    pub use super::pool::{AppService, ServiceKey, ServicePool, ServicePoolError};
}
