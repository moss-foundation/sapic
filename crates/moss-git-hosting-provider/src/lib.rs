pub mod common;
pub mod github;
pub mod gitlab;

use url::Url;

// TODO: Create a `Sensitive` type for storing passwords securely?
// TODO: Preserving the auth info for repos
pub trait GitHostingProvider {
    fn name(&self) -> String;
    fn base_url(&self) -> Url;
}
