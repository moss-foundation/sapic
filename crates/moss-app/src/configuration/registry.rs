use joinerror::ResultExt;
use moss_contrib::include::IncludeConfiguration;
use moss_logging::session;
use moss_text::ReadOnlyStr;
use serde::Deserialize;
use serde_json::Value as JsonValue;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use crate::models::{
    primitives::ConfigurationParameterType as ParameterType,
    types::{ConfigurationNodeSchema, ConfigurationParameterItemSchema},
};

#[derive(Debug, Deserialize)]
struct ParameterDecl {
    pub id: ReadOnlyStr,
    pub default: Option<JsonValue>,
    #[serde(rename = "type")]
    pub typ: ParameterType,
    pub description: Option<String>,
    pub maximum: Option<u64>,
    pub minimum: Option<u64>,
    pub excluded: bool,
    pub protected: bool,
    pub order: Option<i64>,
    #[serde(default)]
    pub tags: Vec<ReadOnlyStr>,
}

#[derive(Clone)]
pub(crate) struct ParameterItem {
    pub id: ReadOnlyStr,
    pub default: Option<JsonValue>,
    pub typ: ParameterType,
    pub description: Option<String>,
    pub maximum: Option<u64>,
    pub minimum: Option<u64>,
    pub protected: bool,
    pub order: Option<i64>,
    pub tags: Vec<ReadOnlyStr>,
}

impl From<ParameterDecl> for ParameterItem {
    fn from(decl: ParameterDecl) -> Self {
        Self {
            id: decl.id.clone(),
            default: decl.default.map(|d| d.into()),
            typ: decl.typ,
            description: decl.description,
            maximum: decl.maximum,
            minimum: decl.minimum,
            protected: decl.protected,
            order: decl.order,
            tags: decl.tags.iter().map(|tag| tag.clone()).collect(),
        }
    }
}

impl From<&ParameterItem> for ConfigurationParameterItemSchema {
    fn from(param: &ParameterItem) -> Self {
        Self {
            id: param.id.to_string(),
            default: param.default.clone(),
            typ: param.typ,
            description: param.description.as_ref().map(|s| s.to_string()),
            maximum: param.maximum,
            minimum: param.minimum,
            protected: param.protected,
            order: param.order,
            tags: param.tags.iter().map(|s| s.to_string()).collect(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct ConfigurationDecl {
    pub id: Option<ReadOnlyStr>,
    pub parent_id: Option<ReadOnlyStr>,
    pub order: Option<i64>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub parameters: Vec<ParameterDecl>,
}

pub(crate) struct ConfigurationNode {
    pub id: ReadOnlyStr,
    pub parent_id: Option<ReadOnlyStr>,
    pub order: Option<i64>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub parameters: Vec<Arc<ParameterItem>>,
}

impl From<&ConfigurationNode> for ConfigurationNodeSchema {
    fn from(value: &ConfigurationNode) -> Self {
        Self {
            id: value.id.to_string(),
            parent_id: value.parent_id.as_ref().map(|s| s.to_string()),
            order: value.order,
            name: value.name.as_ref().map(|s| s.to_string()),
            description: value.description.as_ref().map(|s| s.to_string()),
            parameters: value.parameters.iter().map(|p| (&**p).into()).collect(),
        }
    }
}

impl ConfigurationNode {
    fn extend(&mut self, params: Vec<Arc<ParameterItem>>) {
        self.parameters.extend(params);
    }
}

pub(super) struct ConfigurationRegistry {
    nodes: HashMap<ReadOnlyStr, Arc<ConfigurationNode>>,
    parameters: HashMap<ReadOnlyStr, Arc<ParameterItem>>,

    // Excluded parameters are hidden from the UI but can still be registered.
    excluded: HashMap<ReadOnlyStr, Arc<ParameterItem>>,

    #[allow(unused)]
    keys: HashSet<ReadOnlyStr>,
}

impl ConfigurationRegistry {
    pub fn new<'a>(
        includes: impl Iterator<Item = &'a IncludeConfiguration>,
    ) -> joinerror::Result<Self> {
        let mut nodes = HashMap::new();
        let mut parameters = HashMap::new();
        let mut excluded = HashMap::new();
        let mut keys = HashSet::new();

        let mut decls = Vec::new();
        for include in includes {
            let decl: Vec<ConfigurationDecl> = serde_json::from_str(include.0)
                .join_err_with::<()>(|| {
                    format!("failed to parse included configuration: {}", include.0)
                })?;
            decls.extend(decl);
        }

        let mut extensions = Vec::new();
        let mut bases = Vec::new();
        for decl in decls {
            if decl.parent_id.is_some() {
                extensions.push(decl);
            } else {
                bases.push(decl);
            }
        }

        for decl in bases {
            let id = if let Some(id) = decl.id {
                id.clone()
            } else {
                session::warn!(format!("configuration declaration has no id:\n{:?}", decl));
                continue;
            };

            let mut params: Vec<Arc<ParameterItem>> = Vec::with_capacity(decl.parameters.len());
            for param_decl in decl.parameters {
                if param_decl.excluded {
                    excluded.insert(param_decl.id.clone(), Arc::new(param_decl.into()));
                } else {
                    params.push(Arc::new(param_decl.into()));
                }
            }

            parameters.extend(params.iter().map(|p| (p.id.clone(), p.clone())));
            keys.extend(params.iter().map(|p| p.id.clone()));
            nodes.insert(
                id.clone(),
                ConfigurationNode {
                    id,
                    parent_id: decl.parent_id.clone(),
                    order: decl.order,
                    name: decl.name,
                    description: decl.description,
                    parameters: params,
                },
            );
        }

        for decl in extensions {
            let parent_id = if let Some(id) = &decl.parent_id {
                id.clone()
            } else {
                session::warn!(format!("configuration declaration has no id:\n{:?}", decl));
                continue;
            };

            let parent = match nodes.get_mut(&parent_id) {
                Some(parent) => parent,
                None => {
                    session::warn!(format!(
                        "configuration declaration has no parent node:\n{:?}",
                        decl
                    ));
                    continue;
                }
            };

            let mut params: Vec<Arc<ParameterItem>> = Vec::with_capacity(decl.parameters.len());
            for param_decl in decl.parameters {
                if param_decl.excluded {
                    excluded.insert(param_decl.id.clone(), Arc::new(param_decl.into()));
                } else {
                    params.push(Arc::new(param_decl.into()));
                }
            }

            parameters.extend(params.iter().map(|p| (p.id.clone(), p.clone())));
            keys.extend(params.iter().map(|p| p.id.clone()));
            parent.extend(params);
        }

        Ok(Self {
            nodes: nodes.into_iter().map(|(k, v)| (k, Arc::new(v))).collect(),
            parameters,
            excluded,
            keys,
        })
    }

    pub fn defaults(&self) -> HashMap<ReadOnlyStr, JsonValue> {
        let mut defaults = HashMap::new();
        for (id, param) in &self.parameters {
            if let Some(default) = &param.default {
                defaults.insert(id.clone(), default.clone());
            }
        }

        for (id, param) in &self.excluded {
            if let Some(default) = &param.default {
                defaults.insert(id.clone(), default.clone());
            }
        }

        defaults
    }

    pub fn nodes(&self) -> HashMap<ReadOnlyStr, Arc<ConfigurationNode>> {
        self.nodes.clone()
    }

    pub fn is_parameter_known(&self, id: &str) -> bool {
        self.parameters.contains_key(id)
    }

    pub fn validate_parameter(&self, id: &str, value: &JsonValue) -> joinerror::Result<()> {
        let param = match self.parameters.get(id) {
            Some(param) => param,
            None => {
                return Ok(());
            }
        };

        // TODO: Implement more complete validation logic based on the schema.

        match param.typ {
            ParameterType::String => {
                if !value.is_string() {
                    return Err(joinerror::Error::new::<()>("value is not a string"));
                }
            }

            ParameterType::Number => {
                if !value.is_number() {
                    return Err(joinerror::Error::new::<()>("value is not a number"));
                }
            }

            ParameterType::Boolean => {
                if !value.is_boolean() {
                    return Err(joinerror::Error::new::<()>("value is not a boolean"));
                }
            }

            ParameterType::Object => {
                if !value.is_object() {
                    return Err(joinerror::Error::new::<()>("value is not an object"));
                }
            }

            ParameterType::Array => {
                if !value.is_array() {
                    return Err(joinerror::Error::new::<()>("value is not an array"));
                }
            }
        }

        Ok(())
    }
}
