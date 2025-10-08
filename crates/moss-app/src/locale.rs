use joinerror::{OptionExt, ResultExt};
use moss_applib::AppRuntime;
use moss_fs::FileSystem;
use moss_language::{
    defaults::TranslationDefaults, loader::LocaleLoader, models::primitives::LanguageId,
    registry::LanguageRegistry,
};
use serde_json::Value as JsonValue;
use std::{collections::HashMap, sync::Arc};

use crate::models::types::LocaleInfo;

const DEFAULT_LANGUAGE: &str = "en";

pub struct LocaleService {
    defaults: TranslationDefaults,
    loader: LocaleLoader,
    registry: Arc<dyn LanguageRegistry>,
}

impl LocaleService {
    pub async fn new<R: AppRuntime>(
        fs: Arc<dyn FileSystem>,
        registry: Arc<dyn LanguageRegistry>,
    ) -> joinerror::Result<Self> {
        Ok(Self {
            defaults: TranslationDefaults::new()?,
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
                        order: None,      // DEPRECATED: remove before merging
                        is_default: None, // DEPRECATED: remove before merging
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
            order: None,      // DEPRECATED: remove before merging
            is_default: None, // DEPRECATED: remove before merging
        })
    }

    // TODO: Should we maintain a separate map based on language code?
    pub async fn get_namespace(&self, code: &str, ns: &str) -> joinerror::Result<JsonValue> {
        let default_namespace_value = self.defaults.namespace(ns).unwrap_or_default();
        dbg!(&default_namespace_value);

        if code == DEFAULT_LANGUAGE {
            return Ok((*default_namespace_value).clone());
        }

        let (_, locale) = self
            .registry
            .list()
            .await
            .into_iter()
            .find(|(_id, item)| item.code == code)
            .ok_or_join_err::<()>(format!("Locale for language code `{}` not found", code))?;

        let namespace_object = self
            .loader
            .load_namespace(&locale.path, ns)
            .await
            .join_err_with::<()>(|| {
                format!("failed to load namespace `{}` for locale `{}`", ns, code)
            })?
            .as_object()
            .ok_or_join_err::<()>(format!(
                "namespace `{}` for locale `{}` is not an object",
                ns, code
            ))?
            .clone();

        let mut merged = default_namespace_value
            .as_object()
            .cloned()
            .unwrap_or_default();

        merged.extend(namespace_object);

        Ok(JsonValue::Object(merged))
    }
}
