use joinerror::Error;
use json_patch::{AddOperation, PatchOperation, RemoveOperation, ReplaceOperation, patch};
use jsonptr::PointerBuf;
use serde_json::{Value as JsonValue, json};

#[derive(Debug, Clone)]
pub enum JsonEditAction {
    Add {
        path: PointerBuf,
        new_value: JsonValue,
    },
    Remove {
        path: PointerBuf,
        old_value: JsonValue,
    },
    Replace {
        path: PointerBuf,
        old_value: JsonValue,
        new_value: JsonValue,
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
        root: &mut JsonValue,
        patches: &[(PatchOperation, EditOptions)],
    ) -> joinerror::Result<()> {
        let mut actions = Vec::with_capacity(patches.len());
        let mut applied_patches = Vec::with_capacity(patches.len());

        for (op, options) in patches {
            match op {
                PatchOperation::Add(AddOperation { path, value }) => {
                    if options.create_missing_segments {
                        actions.extend(ensure_path_exists(root, path)?);
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
                        actions.extend(ensure_path_exists(root, path)?);
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
                            // If `create_missing_segments` is true, it will
                            if options.create_missing_segments {
                                actions.push(JsonEditAction::Add {
                                    path: path.clone(),
                                    new_value: value.clone(),
                                });
                                applied_patches.push(PatchOperation::Add(AddOperation {
                                    path: path.clone(),
                                    value: value.clone(),
                                }));
                            } else if options.ignore_if_not_exists {
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

    pub fn undo(&mut self, root: &mut JsonValue) -> joinerror::Result<()> {
        while let Some(action) = self.applied.pop() {
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

    pub fn redo(&mut self, root: &mut JsonValue) -> joinerror::Result<()> {
        while let Some(action) = self.undone.pop() {
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

/// Return a list of actions that automatically created the missing segments
fn ensure_path_exists(
    root: &mut JsonValue,
    path: &PointerBuf,
) -> joinerror::Result<Vec<JsonEditAction>> {
    let segments = path
        .tokens()
        .map(|t| t.decoded().to_string())
        .collect::<Vec<_>>();

    if segments.is_empty() {
        return Ok(Vec::new()); // Root path, nothing to ensure
    }

    let mut actions = Vec::with_capacity(segments.len());

    let mut current = root;
    let mut current_path = PointerBuf::new();

    for segment in &segments[..segments.len() - 1] {
        current_path.push_back(segment);
        if current.is_object() {
            let obj = current.as_object_mut().unwrap();

            if !obj.contains_key(segment) {
                obj.insert(segment.clone(), json!({}));
                actions.push(JsonEditAction::Add {
                    path: current_path.clone(),
                    new_value: JsonValue::Object(serde_json::Map::new()),
                });
            }

            current = obj.get_mut(segment).unwrap();
        } else {
            return Err(joinerror::Error::new::<()>(format!(
                "segment '{}' expected to be an object",
                segment
            )));
        }
    }

    Ok(actions)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_add_action() {
        let mut root = JsonValue::Object(serde_json::Map::new());

        let mut edit = JsonEdit::new();
        unsafe {
            edit.apply(
                &mut root,
                &[(
                    PatchOperation::Add(AddOperation {
                        path: PointerBuf::new_unchecked("/foo"),
                        value: JsonValue::Number(42.into()),
                    }),
                    EditOptions {
                        ignore_if_not_exists: false,
                        create_missing_segments: false,
                    },
                )],
            )
            .unwrap();
        }

        assert!(root.get("foo").unwrap().as_number().unwrap().eq(&42.into()));
    }

    #[test]
    fn test_add_action_create_missing_segments() {
        let mut root = JsonValue::Object(serde_json::Map::new());

        let mut edit = JsonEdit::new();
        unsafe {
            edit.apply(
                &mut root,
                &[(
                    PatchOperation::Add(AddOperation {
                        path: PointerBuf::new_unchecked("/foo/bar"),
                        value: JsonValue::Number(42.into()),
                    }),
                    EditOptions {
                        ignore_if_not_exists: false,
                        create_missing_segments: true,
                    },
                )],
            )
            .unwrap();
        }

        assert_eq!(
            root.get("foo").unwrap().get("bar").unwrap(),
            &JsonValue::Number(42.into())
        );
    }

    #[test]
    fn test_add_action_undo() {
        let mut root = JsonValue::Object(serde_json::Map::new());

        let mut edit = JsonEdit::new();
        unsafe {
            edit.apply(
                &mut root,
                &[(
                    PatchOperation::Add(AddOperation {
                        path: PointerBuf::new_unchecked("/foo"),
                        value: JsonValue::Number(42.into()),
                    }),
                    EditOptions {
                        ignore_if_not_exists: false,
                        create_missing_segments: false,
                    },
                )],
            )
            .unwrap();
        }

        edit.undo(&mut root).unwrap();

        assert_eq!(root, JsonValue::Object(serde_json::Map::new()));
    }

    #[test]
    fn test_add_action_undo_create_missing_segments() {
        let mut root = JsonValue::Object(serde_json::Map::new());

        let mut edit = JsonEdit::new();
        unsafe {
            edit.apply(
                &mut root,
                &[(
                    PatchOperation::Add(AddOperation {
                        path: PointerBuf::new_unchecked("/foo/bar"),
                        value: JsonValue::Number(42.into()),
                    }),
                    EditOptions {
                        ignore_if_not_exists: false,
                        create_missing_segments: true,
                    },
                )],
            )
            .unwrap();
        }

        edit.undo(&mut root).unwrap();
        assert_eq!(root, JsonValue::Object(serde_json::Map::new()));
    }

    #[test]
    fn test_remove_action() {
        let mut root = json!({
            "foo": 42
        });

        let mut edit = JsonEdit::new();

        unsafe {
            edit.apply(
                &mut root,
                &[(
                    PatchOperation::Remove(RemoveOperation {
                        path: PointerBuf::new_unchecked("/foo"),
                    }),
                    EditOptions {
                        ignore_if_not_exists: false,
                        create_missing_segments: false,
                    },
                )],
            )
            .unwrap();
        }

        assert_eq!(root, JsonValue::Object(serde_json::Map::new()));
    }

    #[test]
    fn test_remove_action_nonexistent() {
        // When `ignore_if_not_exist` is false, remove will fail if the path does not exist
        let mut root = json!({});

        let mut edit = JsonEdit::new();

        let result = unsafe {
            edit.apply(
                &mut root,
                &[(
                    PatchOperation::Remove(RemoveOperation {
                        path: PointerBuf::new_unchecked("/foo"),
                    }),
                    EditOptions {
                        ignore_if_not_exists: false,
                        create_missing_segments: false,
                    },
                )],
            )
        };

        assert!(result.is_err());
    }

    #[test]
    fn test_remove_action_ignore_if_not_exists() {
        // When `ignore_if_not_exist` is true, the operation will be skipped if the path does not exist
        let mut root = json!({});

        let mut edit = JsonEdit::new();

        unsafe {
            edit.apply(
                &mut root,
                &[(
                    PatchOperation::Remove(RemoveOperation {
                        path: PointerBuf::new_unchecked("/foo"),
                    }),
                    EditOptions {
                        ignore_if_not_exists: true,
                        create_missing_segments: false,
                    },
                )],
            )
            .unwrap();
        }

        assert_eq!(root, JsonValue::Object(serde_json::Map::new()));
    }

    #[test]
    fn test_remove_action_undo() {
        let mut root = json!({
            "foo": 42
        });

        let mut edit = JsonEdit::new();

        unsafe {
            edit.apply(
                &mut root,
                &[(
                    PatchOperation::Remove(RemoveOperation {
                        path: PointerBuf::new_unchecked("/foo"),
                    }),
                    EditOptions {
                        ignore_if_not_exists: false,
                        create_missing_segments: false,
                    },
                )],
            )
            .unwrap();
        }

        edit.undo(&mut root).unwrap();

        assert_eq!(root, json!({"foo": 42}));
    }

    #[test]
    fn test_replace_action() {
        let mut root = json!({
            "foo": 42
        });
        let mut edit = JsonEdit::new();

        unsafe {
            edit.apply(
                &mut root,
                &[(
                    PatchOperation::Replace(ReplaceOperation {
                        path: PointerBuf::new_unchecked("/foo"),
                        value: JsonValue::String("New".to_string()),
                    }),
                    EditOptions {
                        ignore_if_not_exists: false,
                        create_missing_segments: false,
                    },
                )],
            )
            .unwrap();
        }

        assert_eq!(
            root.get("foo").unwrap(),
            &JsonValue::String("New".to_string())
        );
    }

    #[test]
    fn test_replace_action_nonexistent() {
        // If neither of the options is true, replace action will fail if the path does not exist
        let mut root = json!({});
        let mut edit = JsonEdit::new();

        let result = unsafe {
            edit.apply(
                &mut root,
                &[(
                    PatchOperation::Replace(ReplaceOperation {
                        path: PointerBuf::new_unchecked("/foo"),
                        value: JsonValue::String("New".to_string()),
                    }),
                    EditOptions {
                        ignore_if_not_exists: false,
                        create_missing_segments: false,
                    },
                )],
            )
        };

        assert!(result.is_err());
    }

    #[test]
    fn test_replace_action_create_missing_segments() {
        // If `create_missing_segments` is true, replace action will recursively create all missing segments
        let mut root = json!({});
        let mut edit = JsonEdit::new();

        unsafe {
            edit.apply(
                &mut root,
                &[(
                    PatchOperation::Replace(ReplaceOperation {
                        path: PointerBuf::new_unchecked("/foo/bar"),
                        value: JsonValue::Number(42.into()),
                    }),
                    EditOptions {
                        ignore_if_not_exists: false,
                        create_missing_segments: true,
                    },
                )],
            )
            .unwrap();
        }

        assert_eq!(
            root.get("foo").unwrap().get("bar").unwrap(),
            &JsonValue::Number(42.into())
        );
    }

    #[test]
    fn test_replace_action_ignore_if_not_exists() {
        // If `ignore_if_not_exists` is true, the action will be skipped when the path does not exist
        let mut root = json!({});
        let mut edit = JsonEdit::new();

        unsafe {
            edit.apply(
                &mut root,
                &[(
                    PatchOperation::Replace(ReplaceOperation {
                        path: PointerBuf::new_unchecked("/foo/bar"),
                        value: JsonValue::Number(42.into()),
                    }),
                    EditOptions {
                        ignore_if_not_exists: true,
                        create_missing_segments: false,
                    },
                )],
            )
            .unwrap();
        }

        assert_eq!(root, JsonValue::Object(serde_json::Map::new()));
    }

    #[test]
    fn test_replace_action_undo() {
        let mut root = json!({
            "foo": 42
        });
        let mut edit = JsonEdit::new();

        unsafe {
            edit.apply(
                &mut root,
                &[(
                    PatchOperation::Replace(ReplaceOperation {
                        path: PointerBuf::new_unchecked("/foo"),
                        value: JsonValue::String("New".to_string()),
                    }),
                    EditOptions {
                        ignore_if_not_exists: false,
                        create_missing_segments: false,
                    },
                )],
            )
            .unwrap();
        }

        edit.undo(&mut root).unwrap();

        assert_eq!(root, json!({"foo": 42}));
    }

    #[test]
    fn test_replace_action_undo_create_missing_segments() {
        let mut root = json!({});
        let mut edit = JsonEdit::new();

        unsafe {
            edit.apply(
                &mut root,
                &[(
                    PatchOperation::Replace(ReplaceOperation {
                        path: PointerBuf::new_unchecked("/foo"),
                        value: JsonValue::String("New".to_string()),
                    }),
                    EditOptions {
                        ignore_if_not_exists: false,
                        create_missing_segments: true,
                    },
                )],
            )
            .unwrap();
        }

        edit.undo(&mut root).unwrap();

        assert_eq!(root, json!({}));
    }
}
