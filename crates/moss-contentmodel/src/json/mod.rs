use json_patch::{AddOperation, PatchOperation, RemoveOperation, ReplaceOperation, patch};
use jsonptr::PointerBuf;
use serde_json::Value;

#[derive(Debug, Clone)]
pub enum Action {
    Add {
        path: PointerBuf,
        new_value: Value,
    },
    Remove {
        path: PointerBuf,
        old_value: Value,
    },
    Replace {
        path: PointerBuf,
        old_value: Value,
        new_value: Value,
    },
}

#[derive(Debug, Clone)]
pub struct JsonModel {
    value: Value,
    applied: Vec<Action>,
    undone: Vec<Action>,
}

impl JsonModel {
    pub fn new(initial: Value) -> Self {
        Self {
            value: initial,
            applied: vec![],
            undone: vec![],
        }
    }

    pub fn apply(&mut self, patches: &[PatchOperation]) -> Result<(), String> {
        let mut actions = Vec::with_capacity(patches.len());

        for op in patches {
            match op {
                PatchOperation::Add(AddOperation { path, value }) => {
                    actions.push(Action::Add {
                        path: path.clone(),
                        new_value: value.clone(),
                    });
                }
                PatchOperation::Remove(RemoveOperation { path }) => {
                    let old = path
                        .resolve(&self.value)
                        .map_err(|e| format!("resolve error: {}", e))?
                        .clone();
                    actions.push(Action::Remove {
                        path: path.clone(),
                        old_value: old,
                    });
                }
                PatchOperation::Replace(ReplaceOperation { path, value }) => {
                    let old = path
                        .resolve(&self.value)
                        .map_err(|e| format!("resolve error: {}", e))?
                        .clone();
                    actions.push(Action::Replace {
                        path: path.clone(),
                        old_value: old,
                        new_value: value.clone(),
                    });
                }
                _ => unimplemented!(),
            }
        }

        patch(&mut self.value, patches).map_err(|e| format!("apply error: {}", e))?;
        self.applied.extend(actions);
        self.undone.clear();
        Ok(())
    }

    pub fn undo(&mut self) -> Result<(), String> {
        if let Some(action) = self.applied.pop() {
            let inverse_patch = match &action {
                Action::Add { path, .. } => {
                    PatchOperation::Remove(RemoveOperation { path: path.clone() })
                }
                Action::Remove {
                    path,
                    old_value: old,
                } => PatchOperation::Add(AddOperation {
                    path: path.clone(),
                    value: old.clone(),
                }),
                Action::Replace {
                    path,
                    old_value: old,
                    ..
                } => PatchOperation::Replace(ReplaceOperation {
                    path: path.clone(),
                    value: old.clone(),
                }),
            };

            patch(&mut self.value, &[inverse_patch]).map_err(|e| format!("undo error: {}", e))?;
            self.undone.push(action);
        }
        Ok(())
    }

    pub fn redo(&mut self) -> Result<(), String> {
        if let Some(action) = self.undone.pop() {
            let redo_patch = match &action {
                Action::Add {
                    path,
                    new_value: value,
                } => PatchOperation::Add(AddOperation {
                    path: path.clone(),
                    value: value.clone(),
                }),
                Action::Remove { path, .. } => {
                    PatchOperation::Remove(RemoveOperation { path: path.clone() })
                }
                Action::Replace {
                    path,
                    new_value: new,
                    ..
                } => PatchOperation::Replace(ReplaceOperation {
                    path: path.clone(),
                    value: new.clone(),
                }),
            };

            patch(&mut self.value, &[redo_patch]).map_err(|e| format!("redo error: {}", e))?;
            self.applied.push(action);
        }
        Ok(())
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn json_model_undo_redo() {
        let initial = json!({"age": 30, "city": "New York"});

        let mut model = JsonModel::new(initial);

        model
            .apply(&[PatchOperation::Add(AddOperation {
                path: PointerBuf::parse("/name").unwrap(),
                value: json!("Jane"),
            })])
            .unwrap();

        assert_eq!(model.value()["name"], json!("Jane"));

        model.undo().unwrap();
        assert!(model.value().get("name").is_none());

        model.redo().unwrap();
        assert_eq!(model.value()["name"], json!("Jane"));
    }
}
