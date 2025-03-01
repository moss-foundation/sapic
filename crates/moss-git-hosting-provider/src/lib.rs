pub mod github;
pub mod gitlab;
pub mod headless;

use url::Url;

// TODO: Create a `Sensitive` type for storing passwords securely?
// TODO: Preserving the auth info for repos
pub trait GitHostingProvider {
    fn name(&self) -> Option<String>;
    fn base_url(&self) -> Option<Url>;
}
