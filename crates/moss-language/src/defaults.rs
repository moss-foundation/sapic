use joinerror::ResultExt;
use rustc_hash::FxHashMap;
use serde_json::Value as JsonValue;
use std::sync::Arc;

use crate::contribution::RegisterTranslationContribution;

type Namespace = String;

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
