use derive_more::Deref;
use joinerror::{OptionExt, ResultExt};
use json_patch::{
    AddOperation, PatchOperation, RemoveOperation, ReplaceOperation, jsonptr::PointerBuf,
};
use moss_applib::AppRuntime;
use moss_bindingutils::primitives::{ChangeJsonValue, ChangeString};
use moss_common::continue_if_err;
use moss_db::{DatabaseError, primitives::AnyValue};
use moss_edit::json::EditOptions;
use moss_fs::{FileSystem, FsResultExt};
use moss_hcl::{HclResultExt, hcl_to_json, json_to_hcl};
use moss_storage::{
    common::VariableStore,
    primitives::segkey::SegKeyBuf,
    storage::operations::{GetItem, TransactionalPutItem, TransactionalRemoveItem},
};
use serde_json::Value as JsonValue;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::watch;

use crate::{
    AnyEnvironment, DescribeEnvironment, ModifyEnvironmentParams,
    configuration::{SourceFile, VariableDecl},
    edit::EnvironmentEditing,
    models::{primitives::VariableId, types::VariableInfo},
    segments::{SEGKEY_VARIABLE_LOCALVALUE, SEGKEY_VARIABLE_ORDER},
    utils,
};

#[derive(Debug, Deref, Clone)]
pub(super) struct EnvironmentPath {
    filename: String,
    pub parent: PathBuf,

    #[deref]
    pub full_path: Arc<Path>,
}

impl EnvironmentPath {
    pub fn new(abs_path: PathBuf) -> joinerror::Result<Self> {
        debug_assert!(abs_path.is_absolute());

        let parent = abs_path
            .parent()
            .ok_or_join_err::<()>("environment path must have a parent")?;

        let name = abs_path
            .file_name()
            .ok_or_join_err::<()>("environment path must have a name")?;

        Ok(Self {
            parent: parent.to_path_buf(),
            filename: name.to_string_lossy().to_string(),
            full_path: abs_path.into(),
        })
    }
}

pub struct Environment<R: AppRuntime> {
    pub(super) fs: Arc<dyn FileSystem>,
    pub(super) abs_path_rx: watch::Receiver<EnvironmentPath>,
    pub(super) edit: EnvironmentEditing,
    pub(super) variable_store: Arc<dyn VariableStore<R::AsyncContext>>,
}

unsafe impl<R: AppRuntime> Send for Environment<R> {}
unsafe impl<R: AppRuntime> Sync for Environment<R> {}

impl<R: AppRuntime> AnyEnvironment<R> for Environment<R> {
    async fn abs_path(&self) -> Arc<Path> {
        self.abs_path_rx.borrow().full_path.clone()
    }

    async fn name(&self) -> joinerror::Result<String> {
        utils::parse_file_name(&self.abs_path_rx.borrow().filename)
            .join_err::<()>("failed to parse environment file name")
    }

    // TODO: add variables()

    // TODO: rename to details
    async fn describe(&self, ctx: &R::AsyncContext) -> joinerror::Result<DescribeEnvironment> {
        let abs_path = self.abs_path().await;
        let rdr = self
            .fs
            .open_file(&abs_path)
            .await
            .join_err_with::<()>(|| format!("failed to open file: {}", abs_path.display()))?;

        let parsed: SourceFile = hcl::from_reader(rdr)
            .join_err_with::<()>(|| format!("failed to parse hcl: {}", abs_path.display()))?;

        let mut variables =
            HashMap::with_capacity(parsed.variables.as_ref().map_or(0, |v| v.len()));

        if let Some(vars) = parsed.variables.as_ref() {
            for (id, var) in vars.iter() {
                let global_value = continue_if_err!(hcl_to_json(&var.value), |err| {
                    println!("failed to convert global value expression: {}", err); // TODO: log error
                });

                // TODO: log error when failed to fetch from the database
                let local_value: Option<JsonValue> = {
                    let segkey = SegKeyBuf::from(id.as_str()).join(SEGKEY_VARIABLE_LOCALVALUE);

                    match GetItem::get(self.variable_store.as_ref(), ctx, segkey)
                        .await
                        .and_then(|v| {
                            v.deserialize::<JsonValue>()
                                .map_err(|e| DatabaseError::Serialization(e.to_string()))
                        }) {
                        Ok(value) => Some(value),
                        Err(e) => {
                            // TODO: log error
                            println!("failed to fetch local_value from the database: {}", e);
                            None
                        }
                    }
                };

                let order: Option<isize> = {
                    let segkey = SegKeyBuf::from(id.as_str()).join(SEGKEY_VARIABLE_ORDER);
                    match GetItem::get(self.variable_store.as_ref(), ctx, segkey)
                        .await
                        .and_then(|v| {
                            v.deserialize::<isize>()
                                .map_err(|e| DatabaseError::Serialization(e.to_string()))
                        }) {
                        Ok(order) => Some(order),
                        Err(e) => {
                            // TODO: log error
                            println!("failed to fetch order from the database: {}", e);
                            None
                        }
                    }
                };

                variables.insert(
                    id.clone(),
                    VariableInfo {
                        id: id.clone(),
                        name: var.name.clone(),
                        global_value: Some(global_value),
                        local_value,
                        disabled: var.options.disabled,
                        order,
                        desc: var.description.clone(),
                    },
                );
            }
        }

        Ok(DescribeEnvironment {
            id: parsed.metadata.id.clone(),
            abs_path,
            color: parsed.metadata.color.clone(),
            name: self.name().await?,
            variables,
        })
    }

    async fn modify(
        &self,
        ctx: &R::AsyncContext,
        params: ModifyEnvironmentParams,
    ) -> joinerror::Result<()> {
        if let Some(new_name) = params.name {
            self.edit.rename(&new_name).await?;
        }

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

        for var_to_add in params.vars_to_add {
            let id = VariableId::new();
            let id_str = id.to_string();

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
                    path: unsafe { PointerBuf::new_unchecked(format!("/variable/{}", id_str)) },
                    value,
                }),
                EditOptions {
                    create_missing_segments: true,
                    ignore_if_not_exists: false,
                },
            ));

            // We don't want database failure to stop the function
            let mut txn = continue_if_err!(self.variable_store.begin_write(&ctx).await, |err| {
                println!("failed to start a write transaction: {}", err);
            });

            let local_value =
                continue_if_err!(AnyValue::serialize(&var_to_add.local_value), |err| {
                    println!("failed to serialize localvalue: {}", err);
                });

            continue_if_err!(
                TransactionalPutItem::put_with_context(
                    self.variable_store.as_ref(),
                    ctx,
                    &mut txn,
                    SegKeyBuf::from(id.as_str()).join(SEGKEY_VARIABLE_LOCALVALUE),
                    local_value,
                )
                .await,
                |err| {
                    println!("failed to put local_value in the database: {}", err);
                }
            );

            let order = continue_if_err!(AnyValue::serialize(&var_to_add.order), |err| {
                println!("failed to serialize order: {}", err);
            });

            continue_if_err!(
                TransactionalPutItem::put_with_context(
                    self.variable_store.as_ref(),
                    ctx,
                    &mut txn,
                    SegKeyBuf::from(id.as_str()).join(SEGKEY_VARIABLE_ORDER),
                    order,
                )
                .await,
                |err| {
                    println!("failed to put local_value in the database: {}", err);
                }
            );

            continue_if_err!(txn.commit(), |err| {
                println!("failed to commit transaction: {}", err);
            });
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

            let mut transaction =
                continue_if_err!(self.variable_store.begin_write(&ctx).await, |err| {
                    println!("failed to start a write transaction: {}", err);
                });

            match var_to_update.local_value {
                Some(ChangeJsonValue::Update(value)) => {
                    let local_value = continue_if_err!(AnyValue::serialize(&value), |err| {
                        println!("failed to serialize local_value: {}", err);
                    });

                    continue_if_err!(
                        TransactionalPutItem::put_with_context(
                            self.variable_store.as_ref(),
                            ctx,
                            &mut transaction,
                            SegKeyBuf::from(var_to_update.id.as_str())
                                .join(SEGKEY_VARIABLE_LOCALVALUE),
                            local_value,
                        )
                        .await,
                        |err| {
                            println!("failed to put local_value in the database: {}", err);
                        }
                    );
                }
                Some(ChangeJsonValue::Remove) => {
                    continue_if_err!(
                        TransactionalRemoveItem::remove(
                            self.variable_store.as_ref(),
                            ctx,
                            &mut transaction,
                            SegKeyBuf::from(var_to_update.id.as_str())
                                .join(SEGKEY_VARIABLE_LOCALVALUE),
                        )
                        .await,
                        |err| {
                            println!("failed to remove local_value in the database: {}", err);
                        }
                    );
                }
                _ => {}
            }

            match var_to_update.order {
                Some(order) => {
                    let order = continue_if_err!(AnyValue::serialize(&order), |err| {
                        println!("failed to serialize order: {}", err);
                    });

                    continue_if_err!(
                        TransactionalPutItem::put_with_context(
                            self.variable_store.as_ref(),
                            ctx,
                            &mut transaction,
                            SegKeyBuf::from(var_to_update.id.as_str()).join(SEGKEY_VARIABLE_ORDER),
                            order,
                        )
                        .await,
                        |err| {
                            println!("failed to put order in the database: {}", err);
                        }
                    )
                }
                None => {}
            }

            continue_if_err!(transaction.commit(), |err| {
                println!("failed to commit transaction: {}", err);
            });
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

            let mut transaction =
                continue_if_err!(self.variable_store.begin_write(&ctx).await, |err| {
                    println!("failed to start a write transaction: {}", err);
                });

            continue_if_err!(
                TransactionalRemoveItem::remove(
                    self.variable_store.as_ref(),
                    ctx,
                    &mut transaction,
                    SegKeyBuf::from(id.as_str()).join(SEGKEY_VARIABLE_LOCALVALUE)
                )
                .await,
                |err| {
                    println!("failed to remove local_value in the database: {}", err);
                }
            );

            continue_if_err!(
                TransactionalRemoveItem::remove(
                    self.variable_store.as_ref(),
                    ctx,
                    &mut transaction,
                    SegKeyBuf::from(id.as_str()).join(SEGKEY_VARIABLE_ORDER)
                )
                .await,
                |err| {
                    println!("failed to remove order in the database: {}", err);
                }
            );

            continue_if_err!(transaction.commit(), |err| {
                println!("failed to commit transaction: {}", err);
            })
        }

        self.edit
            .edit(&patches)
            .await
            .join_err::<()>("failed to edit environment")?;

        Ok(())
    }
}
