pub mod auth;
pub mod client;
mod response;

pub struct GitLabAuthProvider {
    // host: String,
}

impl GitLabAuthProvider {
    pub fn new() -> Self {
        Self {}
    }
}
