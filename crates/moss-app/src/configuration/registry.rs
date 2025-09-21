use moss_configuration::{ConfigurationDecl, ParameterDecl, ParameterType};
use moss_logging::session;
use moss_text::ReadOnlyStr;
use serde_json::Value as JsonValue;
use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    sync::Arc,
};

#[allow(unused)] // All fields of the structure will be used later
pub struct ParameterValue {
    pub id: ReadOnlyStr,
    pub default: Option<JsonValue>,
    pub typ: ParameterType,
    pub description: Option<Cow<'static, str>>,
    pub maximum: Option<u64>,
    pub minimum: Option<u64>,
    pub protected: bool,
    pub order: Option<i64>,
    pub tags: Vec<Cow<'static, str>>,
}

impl From<&ParameterDecl> for ParameterValue {
    fn from(decl: &ParameterDecl) -> Self {
        Self {
            id: decl.id.clone(),
            default: decl.default.map(|d| d.into()),
            typ: decl.typ,
            description: decl.description.map(|s| Cow::Borrowed(s)),
            maximum: decl.maximum,
            minimum: decl.minimum,
            protected: decl.protected,
            order: decl.order,
            tags: decl.tags.iter().map(|s| Cow::Borrowed(*s)).collect(),
        }
    }
}

#[allow(unused)] // All fields of the structure will be used later
pub struct ConfigurationValue {
    pub id: ReadOnlyStr,
    pub parent_id: Option<ReadOnlyStr>,
    pub order: Option<i64>,
    pub name: Option<Cow<'static, str>>,
    pub description: Option<Cow<'static, str>>,
    pub parameters: Vec<Arc<ParameterValue>>,
}

impl ConfigurationValue {
    fn extend(&mut self, params: Vec<Arc<ParameterValue>>) {
        self.parameters.extend(params);
    }
}

#[allow(unused)] // All fields of the structure will be used later
pub struct ConfigurationRegistry {
    nodes: HashMap<ReadOnlyStr, ConfigurationValue>,
    parameters: HashMap<ReadOnlyStr, Arc<ParameterValue>>,

    // Excluded parameters are hidden from the UI but can still be registered.
    excluded: HashMap<ReadOnlyStr, Arc<ParameterValue>>,
    keys: HashSet<ReadOnlyStr>,
}

impl ConfigurationRegistry {
    pub fn new<'a>(decls: impl Iterator<Item = &'a ConfigurationDecl>) -> Self {
        let mut nodes = HashMap::new();
        let mut parameters = HashMap::new();
        let mut excluded = HashMap::new();
        let mut keys = HashSet::new();

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
            let id = if let Some(id) = &decl.id {
                id.clone()
            } else {
                session::warn!(format!("configuration declaration has no id:\n{:?}", decl));
                continue;
            };

            let mut params: Vec<Arc<ParameterValue>> = Vec::with_capacity(decl.parameters.len());
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
                ConfigurationValue {
                    id,
                    parent_id: decl.parent_id.clone(),
                    order: decl.order,
                    name: decl.name.map(|s| Cow::Borrowed(s)),
                    description: decl.description.map(|s| Cow::Borrowed(s)),
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

            let mut params: Vec<Arc<ParameterValue>> = Vec::with_capacity(decl.parameters.len());
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

        Self {
            nodes,
            parameters,
            excluded,
            keys,
        }
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
