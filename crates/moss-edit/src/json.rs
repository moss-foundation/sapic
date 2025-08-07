use joinerror::Error;
use json_patch::{AddOperation, PatchOperation, RemoveOperation, ReplaceOperation, patch};
use jsonptr::PointerBuf;
use serde_json::{Value, json};

#[derive(Debug, Clone)]
pub enum JsonEditAction {
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
pub struct EditOptions {
    /// If true, `Remove` and `Replace` operations will be skipped if the path does not exist
    pub ignore_if_not_exists: bool,

    /// If true, `Replace` operation will automatically create missing segments
    /// Otherwise it will raise an error
    pub create_missing_segments: bool,
}

struct ResolveError;
impl ResolveError {
    fn from(e: jsonptr::resolve::Error) -> Error {
        Error::new::<()>(format!("resolve error: {}", e))
    }
}

pub struct JsonEdit {
    applied: Vec<JsonEditAction>,
    undone: Vec<JsonEditAction>,
}

impl JsonEdit {
    pub fn new() -> Self {
        Self {
            applied: vec![],
            undone: vec![],
        }
    }

    pub fn apply(
        &mut self,
        root: &mut Value,
        patches: &[(PatchOperation, EditOptions)],
    ) -> joinerror::Result<()> {
        let mut actions = Vec::with_capacity(patches.len());
        let mut applied_patches = Vec::with_capacity(patches.len());

        for (op, options) in patches {
            match op {
                PatchOperation::Add(AddOperation { path, value }) => {
                    if options.create_missing_segments {
                        ensure_path_exists(root, path)?;
                    }

                    actions.push(JsonEditAction::Add {
                        path: path.clone(),
                        new_value: value.clone(),
                    });
                    applied_patches.push(op.clone());
                }
                PatchOperation::Remove(RemoveOperation { path }) => {
                    match path.resolve(root).map_err(ResolveError::from) {
                        Ok(old) => {
                            actions.push(JsonEditAction::Remove {
                                path: path.clone(),
                                old_value: old.clone(),
                            });
                            applied_patches.push(op.clone());
                        }
                        Err(e) => {
                            if options.ignore_if_not_exists {
                                continue;
                            } else {
                                return Err(e);
                            }
                        }
                    }
                }
                PatchOperation::Replace(ReplaceOperation { path, value }) => {
                    if options.create_missing_segments {
                        ensure_path_exists(root, path)?;
                    }

                    match path.resolve(root).map_err(ResolveError::from) {
                        Ok(old) => {
                            actions.push(JsonEditAction::Replace {
                                path: path.clone(),
                                old_value: old.clone(),
                                new_value: value.clone(),
                            });
                            applied_patches.push(op.clone());
                        }
                        Err(e) => {
                            if options.ignore_if_not_exists {
                                continue;
                            } else {
                                return Err(e);
                            }
                        }
                    }
                }
                _ => unimplemented!(),
            }
        }

        patch(root, &applied_patches)
            .map_err(|e| Error::new::<()>(format!("apply error: {}", e)))?;
        self.applied.extend(actions);
        self.undone.clear();
        Ok(())
    }

    pub fn undo(&mut self, root: &mut Value) -> joinerror::Result<()> {
        if let Some(action) = self.applied.pop() {
            let inverse_patch = match &action {
                JsonEditAction::Add { path, .. } => {
                    PatchOperation::Remove(RemoveOperation { path: path.clone() })
                }
                JsonEditAction::Remove {
                    path,
                    old_value: old,
                } => PatchOperation::Add(AddOperation {
                    path: path.clone(),
                    value: old.clone(),
                }),
                JsonEditAction::Replace {
                    path,
                    old_value: old,
                    ..
                } => PatchOperation::Replace(ReplaceOperation {
                    path: path.clone(),
                    value: old.clone(),
                }),
            };

            patch(root, &[inverse_patch])
                .map_err(|e| Error::new::<()>(format!("undo error: {}", e)))?;
            self.undone.push(action);
        }
        Ok(())
    }

    pub fn redo(&mut self, root: &mut Value) -> joinerror::Result<()> {
        if let Some(action) = self.undone.pop() {
            let redo_patch = match &action {
                JsonEditAction::Add {
                    path,
                    new_value: value,
                } => PatchOperation::Add(AddOperation {
                    path: path.clone(),
                    value: value.clone(),
                }),
                JsonEditAction::Remove { path, .. } => {
                    PatchOperation::Remove(RemoveOperation { path: path.clone() })
                }
                JsonEditAction::Replace {
                    path,
                    new_value: new,
                    ..
                } => PatchOperation::Replace(ReplaceOperation {
                    path: path.clone(),
                    value: new.clone(),
                }),
            };

            patch(root, &[redo_patch])
                .map_err(|e| Error::new::<()>(format!("redo error: {}", e)))?;
            self.applied.push(action);
        }
        Ok(())
    }
}

fn ensure_path_exists(root: &mut Value, path: &PointerBuf) -> joinerror::Result<()> {
    let segments = path
        .tokens()
        .map(|t| t.decoded().to_string())
        .collect::<Vec<_>>();

    if segments.is_empty() {
        return Ok(()); // Root path, nothing to ensure
    }

    let mut current = root;

    for segment in &segments[..segments.len() - 1] {
        if current.is_object() {
            let obj = current.as_object_mut().unwrap();

            if !obj.contains_key(segment) {
                obj.insert(segment.clone(), json!({}));
            }

            current = obj.get_mut(segment).unwrap();
        } else {
            return Err(joinerror::Error::new::<()>(format!(
                "segment '{}' expected to be an object",
                segment
            )));
        }
    }

    Ok(())
}
