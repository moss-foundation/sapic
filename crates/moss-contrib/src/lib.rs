use moss_fs::FileSystem;
use moss_logging::session;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

pub struct IncludeConfigurationDecl(pub &'static str);
inventory::collect!(IncludeConfigurationDecl);

pub struct IncludeContribution(pub &'static str);
inventory::collect!(IncludeContribution);

pub struct ExtensionPoint {
    key: &'static str,
    handler: fn(&mut HashMap<String, JsonValue>),
}
inventory::collect!(ExtensionPoint);

inventory::submit! {
    ExtensionPoint {
        key: "configuration",
        handler: |params| {
            params.insert("configuration".to_string(), JsonValue::Null);
        },
    }
}

// pub struct ExtensionService {}
