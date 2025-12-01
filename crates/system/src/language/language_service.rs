use joinerror::{OptionExt, ResultExt};
use rustc_hash::FxHashMap;
use sapic_base::language::types::{LanguageInfo, primitives::LanguageCode};
use serde_json::Value as JsonValue;
use std::{collections::HashMap, sync::Arc};

use crate::language::{LanguagePackLoader, LanguagePackRegistry};

type Namespace = String;

pub struct RegisterTranslationContribution(pub &'static str);
inventory::collect!(RegisterTranslationContribution);

const DEFAULT_LANGUAGE_CODE: &str = "en";

pub struct TranslationDefaults(FxHashMap<Namespace, Arc<JsonValue>>);

impl TranslationDefaults {
    pub fn new() -> joinerror::Result<Self> {
        let mut aggregated = FxHashMap::default();
        for contrib in inventory::iter::<RegisterTranslationContribution>() {
            let decl: FxHashMap<String, Arc<JsonValue>> = serde_json::from_str(contrib.0)
                .join_err_with::<()>(|| {
                    format!(
                        "failed to parse included translation defaults: {}",
                        contrib.0
                    )
                })?;

            aggregated.extend(decl);
        }

        Ok(Self(aggregated.into()))
    }

    pub fn namespace(&self, ns: &str) -> Option<Arc<JsonValue>> {
        self.0.get(ns).cloned()
    }
}

pub struct LanguageService {
    defaults: TranslationDefaults,
    loader: Arc<dyn LanguagePackLoader>,
    registry: Arc<dyn LanguagePackRegistry>,
}

impl LanguageService {
    pub fn new(
        registry: Arc<dyn LanguagePackRegistry>,
        loader: Arc<dyn LanguagePackLoader>,
    ) -> joinerror::Result<Self> {
        Ok(Self {
            defaults: TranslationDefaults::new()?,
            registry,
            loader,
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
