// use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

// use crate::models::primitives::EnvironmentId;

// use super::models::types::{VariableKind, VariableName, VariableValue};

// #[derive(Debug)]
// pub struct VariableUpdate {
//     pub kind: Option<VariableKind>,
//     pub value: Option<VariableValue>,
//     pub desc: Option<String>,
// }

// #[derive(Clone, Debug, Deserialize, Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Variable {
//     pub kind: Option<VariableKind>,
//     pub value: Option<VariableValue>,
//     pub desc: Option<String>,
// }

// impl Variable {
//     pub fn update(&mut self, update: VariableUpdate) {
//         if let Some(kind) = update.kind {
//             self.kind = Some(kind);
//         }
//         if let Some(value) = update.value {
//             self.value = Some(value);
//         }
//         if let Some(desc) = update.desc {
//             self.desc = Some(desc);
//         }
//     }
// }

// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct FileModel {
//     pub id: EnvironmentId,
//     pub values: HashMap<VariableName, Variable>,
// }

// impl FileModel {
//     pub fn new() -> Self {
//         Self {
//             id: EnvironmentId::new(),
//             values: HashMap::new(),
//         }
//     }
// }
