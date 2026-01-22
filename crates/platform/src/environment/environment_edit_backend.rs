use async_trait::async_trait;
use joinerror::ResultExt;
use json_patch::{AddOperation, PatchOperation, RemoveOperation, ReplaceOperation};
use jsonptr::PointerBuf;
use moss_bindingutils::primitives::{ChangeJsonValue, ChangeString};
use moss_common::continue_if_err;
use moss_edit::json::{EditOptions, JsonEdit};
use moss_environment::configuration::VariableDecl;
use moss_fs::{CreateOptions, FileSystem};
use moss_hcl::{HclResultExt, json_to_hcl};
use sapic_core::context::AnyAsyncContext;
use sapic_system::environment::{EnvironmentEditBackend, EnvironmentEditParams};
use serde_json::Value as JsonValue;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::RwLock;

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

        if let Some(new_name) = params.name {
            patches.push((
                PatchOperation::Add(AddOperation {
                    path: unsafe { PointerBuf::new_unchecked("/metadata/name") },
                    value: JsonValue::String(new_name),
                }),
                EditOptions {
                    create_missing_segments: true,
                    ignore_if_not_exists: false,
                },
            ));
        }

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
            hcl::from_reader(rdr).join_err::<()>("failed to parse environment json")?;

        let mut edits_lock = self.edits.write().await;
        edits_lock
            .apply(&mut value, &patches)
            .join_err::<()>("failed to apply patches")?;

        let content =
            hcl::to_string(&value).join_err::<()>("failed to serialize environment json")?;

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

#[cfg(test)]
mod tests {
    use hcl::Expression;
    use indexmap::indexmap;
    use moss_environment::models::types::{
        AddVariableParams, UpdateVariableParams, VariableOptions,
    };
    use moss_fs::RealFileSystem;
    use moss_testutils::random_name::random_string;
    use sapic_base::environment::types::primitives::{EnvironmentId, VariableId};
    use sapic_core::context::ArcContext;
    use sapic_system::environment::{
        CreateEnvironmentFsParams, EnvironmentServiceFs as EnvironmentServiceFsPort,
    };

    use crate::environment::environment_service_fs::EnvironmentServiceFs;

    use super::*;

    async fn setup_env_edit_backend() -> (
        ArcContext,
        EnvironmentId,
        Arc<EnvironmentServiceFs>,
        Arc<EnvironmentFsEditBackend>,
        PathBuf,
    ) {
        let ctx = ArcContext::background();
        let test_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("tests")
            .join("data")
            .join(random_string(10));

        let tmp_path = test_path.join("tmp");
        let environments_path = test_path.join("environments");

        tokio::fs::create_dir_all(&environments_path).await.unwrap();
        tokio::fs::create_dir_all(&tmp_path).await.unwrap();

        let fs = Arc::new(RealFileSystem::new(&tmp_path));
        let env_fs = Arc::new(EnvironmentServiceFs::new(
            environments_path.clone(),
            fs.clone(),
        ));

        let id = EnvironmentId::new();
        let create_params = CreateEnvironmentFsParams {
            name: "Test".to_string(),
            color: Some("#ff0000".to_string()),
            variables: Default::default(),
        };

        let path = env_fs
            .create_environment(&ctx, &id, &create_params)
            .await
            .unwrap();

        let edit = EnvironmentFsEditBackend::new(&path, fs);

        (ctx, id, env_fs, edit, test_path)
    }

    #[tokio::test]
    async fn test_edit_name() {
        let (ctx, id, env_fs, edit, test_path) = setup_env_edit_backend().await;

        let edit_params = EnvironmentEditParams {
            name: Some("New Name".to_string()),
            color: None,
            vars_to_add: vec![],
            vars_to_update: vec![],
            vars_to_delete: vec![],
        };

        edit.edit(&ctx, edit_params.clone()).await.unwrap();

        let source_file = env_fs.read_environment_sourcefile(&ctx, &id).await.unwrap();

        assert_eq!(source_file.metadata.name, edit_params.name.unwrap());
        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    async fn test_edit_color() {
        let (ctx, id, env_fs, edit, test_path) = setup_env_edit_backend().await;

        // Change color
        let new_color = "#000000".to_string();
        let edit_params = EnvironmentEditParams {
            name: None,
            color: Some(ChangeString::Update(new_color.clone())),
            vars_to_add: vec![],
            vars_to_update: vec![],
            vars_to_delete: vec![],
        };

        edit.edit(&ctx, edit_params.clone()).await.unwrap();

        let source_file = env_fs.read_environment_sourcefile(&ctx, &id).await.unwrap();

        assert_eq!(source_file.metadata.color, Some(new_color));

        // Remove color
        let edit_params = EnvironmentEditParams {
            name: None,
            color: Some(ChangeString::Remove),
            vars_to_add: vec![],
            vars_to_update: vec![],
            vars_to_delete: vec![],
        };

        edit.edit(&ctx, edit_params.clone()).await.unwrap();

        let source_file = env_fs.read_environment_sourcefile(&ctx, &id).await.unwrap();

        assert_eq!(source_file.metadata.color, None);
        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    async fn test_edit_vars() {
        let (ctx, id, env_fs, edit, test_path) = setup_env_edit_backend().await;

        let var_id = VariableId::new();
        let var_params = AddVariableParams {
            name: "Test".to_string(),
            global_value: JsonValue::String("Value 1".to_string()),
            local_value: JsonValue::Null,
            order: 0,
            desc: Some("Description".to_string()),
            options: VariableOptions { disabled: false },
        };
        // Add Variable
        edit.edit(
            &ctx,
            EnvironmentEditParams {
                name: None,
                color: None,
                vars_to_add: vec![(var_id.clone(), var_params.clone())],
                vars_to_update: vec![],
                vars_to_delete: vec![],
            },
        )
        .await
        .unwrap();

        let source_file = env_fs.read_environment_sourcefile(&ctx, &id).await.unwrap();

        assert_eq!(
            source_file.variables.unwrap().into_inner(),
            indexmap! {
                var_id.clone() => VariableDecl {
                    name: var_params.name.clone(),
                    value: Expression::String("Value 1".to_string()),
                    description: var_params.desc.clone(),
                    options: VariableOptions {
                        disabled: false
                    },
                }
            }
        );

        // Update Variable
        let update_params = UpdateVariableParams {
            id: var_id.clone(),
            name: Some("New Name".to_string()),
            global_value: Some(ChangeJsonValue::Update(JsonValue::Number(42.into()))),
            local_value: None,
            order: None,
            desc: Some(ChangeString::Remove),
            options: Some(VariableOptions { disabled: true }),
        };

        edit.edit(
            &ctx,
            EnvironmentEditParams {
                name: None,
                color: None,
                vars_to_add: vec![],
                vars_to_update: vec![update_params.clone()],
                vars_to_delete: vec![],
            },
        )
        .await
        .unwrap();

        let source_file = env_fs.read_environment_sourcefile(&ctx, &id).await.unwrap();

        assert_eq!(
            source_file.variables.unwrap().into_inner(),
            indexmap! {
                var_id.clone() => VariableDecl {
                    name: "New Name".to_string(),
                    value: Expression::Number(42.into()),
                    description: None,
                    options: VariableOptions {
                        disabled: true,
                    },
                }
            }
        );

        // Remove Variable

        edit.edit(
            &ctx,
            EnvironmentEditParams {
                name: None,
                color: None,
                vars_to_add: vec![],
                vars_to_update: vec![],
                vars_to_delete: vec![var_id.clone()],
            },
        )
        .await
        .unwrap();

        let source_file = env_fs.read_environment_sourcefile(&ctx, &id).await.unwrap();

        assert_eq!(source_file.variables.unwrap().into_inner(), indexmap! {});

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }
}
