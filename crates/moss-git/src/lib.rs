pub mod adapters;
pub mod models;
pub mod ports;

pub mod repo;

use anyhow::Result;
use std::sync::Arc;

// TODO: Create a `Sensitive` type for storing passwords securely?
// TODO: Preserving the auth info for repos

pub trait TestStorage {
    // TODO: We will use more secure method of storing the AuthAgent info
    // For easy testing, we will use environment variables for now
    fn write_to_file(&self) -> Result<()>;
    fn read_from_file() -> Result<Arc<Self>>;
}
