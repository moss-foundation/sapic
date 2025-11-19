use async_trait::async_trait;
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use moss_project::registries::{GlobalHttpHeaderRegistry, http_headers::HttpHeaderRegistryItem};
use sapic_base::extension::{contribution::ContributionKey, types::LoadedExtensionInfo};
use sapic_runtime::extension_point::ExtensionPoint;
use serde_json::Value as JsonValue;

const HTTP_HEADERS_KEY: ContributionKey = ContributionKey::new("http_headers");

pub struct HttpHeadersExtensionPoint;

impl HttpHeadersExtensionPoint {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

#[async_trait]
impl<R: AppRuntime> ExtensionPoint<R> for HttpHeadersExtensionPoint {
    fn key(&self) -> ContributionKey {
        HTTP_HEADERS_KEY
    }

    async fn handle(
        &self,
        app_delegate: &AppDelegate<R>,
        _: &LoadedExtensionInfo,
        data: JsonValue,
    ) -> joinerror::Result<()> {
        if !data.is_array() {
            joinerror::bail!("http headers contribution must be an array");
        }

        let headers: Vec<HttpHeaderRegistryItem> = serde_json::from_value(data)?;
        app_delegate
            .global::<GlobalHttpHeaderRegistry>()
            .register(headers)
            .await;

        Ok(())
    }
}
