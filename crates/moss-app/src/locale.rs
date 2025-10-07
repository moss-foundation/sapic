use joinerror::OptionExt;
use moss_applib::AppRuntime;
use moss_fs::FileSystem;
use moss_language::{
    loader::LocaleLoader, models::primitives::LanguageId, registry::LanguageRegistry,
};
use serde_json::Value as JsonValue;
use std::{collections::HashMap, sync::Arc};

use crate::models::types::LocaleInfo;

pub struct LocaleService {
    loader: LocaleLoader,
    registry: Arc<dyn LanguageRegistry>,
}

impl LocaleService {
    pub async fn new<R: AppRuntime>(
        fs: Arc<dyn FileSystem>,
        registry: Arc<dyn LanguageRegistry>,
    ) -> joinerror::Result<Self> {
        Ok(Self {
            registry,
            loader: LocaleLoader::new(fs),
        })
    }

    pub async fn locales(&self) -> HashMap<LanguageId, LocaleInfo> {
        let locales = self.registry.list().await;
        locales
            .into_iter()
            .map(|(id, item)| {
                (
                    id.clone(),
                    LocaleInfo {
                        identifier: item.identifier,
                        display_name: item.display_name,
                        code: item.code.clone(),
                        direction: item.direction,
                        order: None,      // FIXME
                        is_default: None, // FIXME
                    },
                )
            })
            .collect()
    }

    pub async fn get_locale(&self, id: &LanguageId) -> Option<LocaleInfo> {
        self.registry.get(id).await.map(|item| LocaleInfo {
            identifier: item.identifier,
            display_name: item.display_name,
            code: item.code,
            direction: item.direction,
            order: None,      // FIXME
            is_default: None, // FIXME
        })
    }

    // TODO: Should we maintain a separate map based on language code?
    pub async fn get_namespace(&self, code: &str, ns: &str) -> joinerror::Result<JsonValue> {
        let (_, locale) = self
            .registry
            .list()
            .await
            .into_iter()
            .find(|(_id, item)| item.code == code)
            .ok_or_join_err::<()>(format!("Locale for language code `{}` not found", code))?;
        let namespace = self.loader.load_namespace(&locale.path, ns).await?;

        Ok(namespace)
    }
}
