pub mod api;
pub mod id_registry;
pub mod models;

pub trait Merge<T> {
    fn merge(&mut self, other: T) -> &mut Self;
}
