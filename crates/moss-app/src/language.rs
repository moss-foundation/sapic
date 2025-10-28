use joinerror::{OptionExt, ResultExt};
use moss_applib::AppRuntime;
use moss_fs::FileSystem;
use moss_language::{
    defaults::TranslationDefaults,
    loader::LanguageLoader,
    models::{primitives::LanguageCode, types::LanguageInfo},
    registry::LanguageRegistry,
};
use serde_json::Value as JsonValue;
use std::{collections::HashMap, sync::Arc};

const DEFAULT_LANGUAGE_CODE: &str = "en";

pub struct LanguageService {
    defaults: TranslationDefaults,
    loader: LanguageLoader,
    registry: Arc<dyn LanguageRegistry>,
}

impl LanguageService {
    pub async fn new<R: AppRuntime>(
        fs: Arc<dyn FileSystem>,
        registry: Arc<dyn LanguageRegistry>,
    ) -> joinerror::Result<Self> {
        Ok(Self {
            defaults: TranslationDefaults::new()?,
            registry,
            loader: LanguageLoader::new(fs),
        })
    }

    pub async fn languages(&self) -> HashMap<String, LanguageInfo> {
        let languages = self.registry.list().await;
        languages
            .into_iter()
            .map(|(id, item)| {
                (
                    id.clone(),
                    LanguageInfo {
                        display_name: item.display_name,
                        code: item.code.clone(),
                        direction: item.direction,
                    },
                )
            })
            .collect()
    }

    // TODO: Should we maintain a separate map based on language code?
    pub async fn get_namespace(
        &self,
        code: &LanguageCode,
        ns: &str,
    ) -> joinerror::Result<JsonValue> {
        let default_namespace_value = self.defaults.namespace(ns).unwrap_or_default();

        if code == DEFAULT_LANGUAGE_CODE {
            return Ok((*default_namespace_value).clone());
        }

        let (_, language) = self
            .registry
            .list()
            .await
            .into_iter()
            .find(|(_id, item)| item.code == *code)
            .ok_or_join_err::<()>(format!("language for language code `{}` not found", code))?;

        let namespace_object = self
            .loader
            .load_namespace(&language.path, ns)
            .await
            .join_err_with::<()>(|| {
                format!("failed to load namespace `{}` for language `{}`", ns, code)
            })?
            .as_object()
            .ok_or_join_err::<()>(format!(
                "namespace `{}` for language `{}` is not an object",
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
