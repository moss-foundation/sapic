pub mod contribution;
pub mod types;

use serde_json::{Map, Value as JsonValue};
use std::collections::HashSet;

#[derive(Clone)]
pub struct ConfigurationModel {
    /// A set of all keys present in this object.
    pub keys: HashSet<String>,
    /// A JSON object with string keys, where the values are specific settings.
    pub contents: Map<String, JsonValue>,
}

impl ConfigurationModel {
    pub fn merge_old(&self, other: &Self) -> Self {
        let mut all_keys: HashSet<String> = self.keys.iter().cloned().collect();
        all_keys.extend(other.keys.iter().cloned());

        let mut merged_contents = self.contents.clone();
        for (k, v) in &other.contents {
            merged_contents.insert(k.clone(), v.clone());
        }

        Self {
            keys: all_keys.into_iter().collect(),
            contents: merged_contents,
        }
    }

    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        self.contents.get(key)
    }

    pub fn merge(&self, values: &Map<String, JsonValue>) -> Self {
        let mut all_keys: HashSet<String> = self.keys.iter().cloned().collect();
        all_keys.extend(values.keys().cloned());

        let mut merged_contents = self.contents.clone();
        for (k, v) in values {
            merged_contents.insert(k.clone(), v.clone());
        }

        Self {
            keys: all_keys.into_iter().collect(),
            contents: merged_contents,
        }
    }

    pub fn values(&self) -> Vec<(String, JsonValue)> {
        self.contents
            .clone()
            .into_iter()
            .map(|(key, value)| (key, value))
            .collect()
    }

    pub fn raw(&self) -> JsonValue {
        JsonValue::Object(self.contents.clone())
    }
}
