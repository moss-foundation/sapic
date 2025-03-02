pub mod common;
pub mod github;
pub mod gitlab;

use url::Url;

pub trait GitHostingProvider {
    fn name(&self) -> String;
    fn base_url(&self) -> Url;
}
