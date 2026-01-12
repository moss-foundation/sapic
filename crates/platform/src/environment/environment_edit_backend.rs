use async_trait::async_trait;
use joinerror::ResultExt;
use json_patch::{AddOperation, PatchOperation, RemoveOperation, ReplaceOperation};
use jsonptr::PointerBuf;
use moss_bindingutils::primitives::{ChangeJsonValue, ChangeString};
use moss_common::continue_if_err;
use moss_edit::json::{EditOptions, JsonEdit};
use moss_environment::configuration::VariableDecl;
use moss_fs::{CreateOptions, FileSystem};
use moss_hcl::json_to_hcl;
use sapic_core::context::AnyAsyncContext;
use sapic_system::environment::{EnvironmentEditBackend, EnvironmentEditParams};
use serde_json::Value as JsonValue;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::{RwLock, watch::Sender};

// I tie the environment edit backend to a particular environment
// Since different environments can have different locations
// Unlike, for example projects, which are all located under workspace/projects
pub struct EnvironmentFsEditBackend {
    // FIXME: Is updating the environment path still a thing after switching to ID based file?
    // This would only happen when we change a global environment to a project environment, vice versa
    // But in that case I assume the frontend will simply re-create the environment?
    environment_abs_path: RwLock<PathBuf>,
    fs: Arc<dyn FileSystem>,
    edits: RwLock<JsonEdit>,
}

impl EnvironmentFsEditBackend {
    pub fn new(environment_abs_path: &Path, fs: Arc<dyn FileSystem>) -> Arc<Self> {
        Arc::new(Self {
            environment_abs_path: RwLock::new(environment_abs_path.to_path_buf()),
            fs,
            edits: RwLock::new(JsonEdit::new()),
        })
    }
}

#[async_trait]
impl EnvironmentEditBackend for EnvironmentFsEditBackend {
    async fn edit(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: EnvironmentEditParams,
    ) -> joinerror::Result<()> {
        // FIXME: tbh I think instead of recreating the file rename logic, we should switch to ID based filename

        let mut patches = Vec::new();

        match params.color {
            Some(ChangeString::Update(color)) => {
                patches.push((
                    PatchOperation::Add(AddOperation {
                        path: unsafe { PointerBuf::new_unchecked("/metadata/color") },
                        value: JsonValue::String(color),
                    }),
                    EditOptions {
                        create_missing_segments: true,
                        ignore_if_not_exists: false,
                    },
                ));
            }
            Some(ChangeString::Remove) => {
                patches.push((
                    PatchOperation::Remove(RemoveOperation {
                        path: unsafe { PointerBuf::new_unchecked("/metadata/color") },
                    }),
                    EditOptions {
                        create_missing_segments: false,
                        ignore_if_not_exists: false,
                    },
                ));
            }
            _ => {}
        };

        for (id, var_to_add) in params.vars_to_add {
            let global_value = continue_if_err!(json_to_hcl(&var_to_add.global_value), |err| {
                println!("failed to convert global value expression: {}", err); // TODO: log error
            });

            let decl = VariableDecl {
                name: var_to_add.name.clone(),
                value: global_value,
                description: var_to_add.desc.clone(),
                options: var_to_add.options.clone(),
            };

            let value = continue_if_err!(serde_json::to_value(decl), |err| {
                println!("failed to convert variable declaration to json: {}", err); // TODO: log error
            });

            patches.push((
                PatchOperation::Add(AddOperation {
                    path: unsafe {
                        PointerBuf::new_unchecked(format!("/variable/{}", id.to_string()))
                    },
                    value,
                }),
                EditOptions {
                    create_missing_segments: true,
                    ignore_if_not_exists: false,
                },
            ));
        }

        for var_to_update in params.vars_to_update {
            if let Some(new_name) = var_to_update.name {
                patches.push((
                    PatchOperation::Replace(ReplaceOperation {
                        path: unsafe {
                            PointerBuf::new_unchecked(format!(
                                "/variable/{}/name",
                                var_to_update.id
                            ))
                        },
                        value: JsonValue::String(new_name),
                    }),
                    EditOptions {
                        // Raise an error if the variable does not exist
                        create_missing_segments: false,
                        ignore_if_not_exists: false,
                    },
                ));
            }

            match var_to_update.global_value {
                Some(ChangeJsonValue::Update(value)) => {
                    patches.push((
                        PatchOperation::Replace(ReplaceOperation {
                            path: unsafe {
                                PointerBuf::new_unchecked(format!(
                                    "/variable/{}/value",
                                    var_to_update.id
                                ))
                            },
                            value,
                        }),
                        EditOptions {
                            // Raise an error if the variable does not exist
                            create_missing_segments: false,
                            ignore_if_not_exists: false,
                        },
                    ));
                }
                Some(ChangeJsonValue::Remove) => {
                    patches.push((
                        PatchOperation::Remove(RemoveOperation {
                            path: unsafe {
                                PointerBuf::new_unchecked(format!(
                                    "/variable/{}/value",
                                    var_to_update.id
                                ))
                            },
                        }),
                        EditOptions {
                            // Raise an error if the variable does not exist
                            create_missing_segments: false,
                            ignore_if_not_exists: false,
                        },
                    ));
                }
                _ => {}
            }

            match var_to_update.desc {
                Some(ChangeString::Update(value)) => {
                    patches.push((
                        PatchOperation::Replace(ReplaceOperation {
                            path: unsafe {
                                PointerBuf::new_unchecked(format!(
                                    "/variable/{}/description",
                                    var_to_update.id
                                ))
                            },
                            value: JsonValue::String(value),
                        }),
                        EditOptions {
                            // Raise an error if the variable does not exist
                            create_missing_segments: false,
                            ignore_if_not_exists: false,
                        },
                    ));
                }
                Some(ChangeString::Remove) => {
                    patches.push((
                        PatchOperation::Remove(RemoveOperation {
                            path: unsafe {
                                PointerBuf::new_unchecked(format!(
                                    "/variable/{}/description",
                                    var_to_update.id
                                ))
                            },
                        }),
                        EditOptions {
                            // Raise an error if the variable does not exist
                            create_missing_segments: false,
                            ignore_if_not_exists: false,
                        },
                    ));
                }
                _ => {}
            }

            if let Some(options) = var_to_update.options {
                let options_value = continue_if_err!(serde_json::to_value(options), |err| {
                    println!("failed to convert variable options to json: {}", err); // TODO: log error
                });

                patches.push((
                    PatchOperation::Replace(ReplaceOperation {
                        path: unsafe {
                            PointerBuf::new_unchecked(format!(
                                "/variable/{}/options",
                                var_to_update.id
                            ))
                        },
                        value: options_value,
                    }),
                    EditOptions {
                        // Raise an error if the variable does not exist
                        create_missing_segments: false,
                        ignore_if_not_exists: false,
                    },
                ));
            }
        }

        for id in params.vars_to_delete {
            patches.push((
                PatchOperation::Remove(RemoveOperation {
                    path: unsafe { PointerBuf::new_unchecked(format!("/variable/{}", id)) },
                }),
                EditOptions {
                    create_missing_segments: false,
                    // Skip the operation if the variable does not exist
                    // FIXME: How should we write logs when that happens?
                    ignore_if_not_exists: true,
                },
            ));
        }

        if patches.is_empty() {
            return Ok(());
        }

        let abs_path = self.environment_abs_path.read().await;

        let rdr = self
            .fs
            .open_file(ctx, abs_path.as_path())
            .await
            .join_err_with::<()>(|| format!("failed to open file: {}", abs_path.display()))?;

        let mut value: JsonValue =
            serde_json::from_reader(rdr).join_err::<()>("failed to parse environment json")?;

        let mut edits_lock = self.edits.write().await;
        edits_lock
            .apply(&mut value, &patches)
            .join_err::<()>("failed to apply patches")?;

        let content = serde_json::to_string(&value).join_err::<()>("failed to serialize json")?;

        self.fs
            .create_file_with(
                ctx,
                abs_path.as_path(),
                content.as_bytes(),
                CreateOptions {
                    overwrite: true,
                    ignore_if_exists: false,
                },
            )
            .await
            .join_err_with::<()>(|| {
                format!(
                    "failed to update environment manifest: {}",
                    abs_path.display()
                )
            })?;

        Ok(())
    }
}
