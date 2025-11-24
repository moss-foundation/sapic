use serde::Deserialize;
use serde_json::Value as JsonValue;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct AddonManifestFile {
    pub contributes: HashMap<String, JsonValue>,
}
