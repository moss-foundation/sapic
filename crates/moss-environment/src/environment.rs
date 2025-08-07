use derive_more::Deref;
use joinerror::{OptionExt, ResultExt};
use json_patch::{
    AddOperation, PatchOperation, RemoveOperation, ReplaceOperation, jsonptr::PointerBuf,
};
use moss_applib::AppRuntime;
use moss_bindingutils::primitives::{ChangeJsonValue, ChangeString};
use moss_common::continue_if_err;
use moss_db::primitives::AnyValue;
use moss_fs::{FileSystem, FsResultExt};
use moss_hcl::{HclResultExt, hcl_to_json, json_to_hcl};
use moss_storage::{
    common::VariableStore,
    storage::operations::{GetItem, PutItem, RemoveItem},
};
use serde_json::Value as JsonValue;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::watch;

use crate::{
    AnyEnvironment, DescribeEnvironment, ModifyEnvironmentParams,
    configuration::{SourceFile, VariableDecl},
    edit::EnvironmentEditing,
    models::{primitives::VariableId, types::VariableInfo},
    utils,
};

use crate::segments::{SEGKEY_VARIABLE_LOCALVALUE, SEGKEY_VARIABLE_ORDER};

#[derive(Debug, Deref, Clone)]
pub(super) struct EnvironmentPath {
    filename: String,
    pub parent: PathBuf,

    #[deref]
    pub full_path: PathBuf,
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
            full_path: abs_path,
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
    async fn abs_path(&self) -> PathBuf {
        self.abs_path_rx.borrow().full_path.clone()
    }

    async fn name(&self) -> joinerror::Result<String> {
        utils::parse_file_name(&self.abs_path_rx.borrow().filename)
            .join_err::<()>("failed to parse environment file name")
    }

    async fn describe(&self, ctx: &R::AsyncContext) -> joinerror::Result<DescribeEnvironment> {
        let abs_path = self.abs_path().await;
        let rdr = self
            .fs
            .open_file(&abs_path)
            .await
            .join_err_with::<()>(|| format!("failed to open file: {}", abs_path.display()))?;

        let parsed: SourceFile = hcl::from_reader(rdr).join_err::<()>("failed to parse hcl")?;

        let mut variables = Vec::with_capacity(parsed.variables.as_ref().map_or(0, |v| v.len()));

        if let Some(vars) = parsed.variables.as_ref() {
            for (id, var) in vars.iter() {
                let global_value = continue_if_err!(hcl_to_json(&var.value), |err| {
                    println!("failed to convert global value expression: {}", err); // TODO: log error
                });

                // TODO: log error when failed to fetch from the database
                let local_value: Option<JsonValue> = {
                    let segkey = SEGKEY_VARIABLE_LOCALVALUE.join(id.as_str());
                    GetItem::get(self.variable_store.as_ref(), ctx, segkey)
                        .await
                        .ok()
                        .and_then(|v| v.deserialize().ok())
                };

                let order: Option<isize> = {
                    let segkey = SEGKEY_VARIABLE_ORDER.join(id.as_str());
                    GetItem::get(self.variable_store.as_ref(), ctx, segkey)
                        .await
                        .ok()
                        .and_then(|v| v.deserialize().ok())
                };

                variables.push(VariableInfo {
                    id: id.clone(),
                    name: var.name.clone(),
                    global_value: Some(global_value),
                    local_value,
                    disabled: var.options.disabled,
                    order,
                    desc: var.description.clone(),
                });
            }
        }

        Ok(DescribeEnvironment {
            id: parsed.metadata.id.clone(),
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
                patches.push(PatchOperation::Add(AddOperation {
                    path: unsafe { PointerBuf::new_unchecked("/metadata/color") },
                    value: JsonValue::String(color),
                }));
            }
            Some(ChangeString::Remove) => {
                patches.push(PatchOperation::Remove(RemoveOperation {
                    path: unsafe { PointerBuf::new_unchecked("/metadata/color") },
                }));
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

            patches.push(PatchOperation::Add(AddOperation {
                path: unsafe { PointerBuf::new_unchecked(format!("/variable/{}", id_str)) },
                value,
            }));

            let order = var_to_add.order;

            let segkey_localvalue = SEGKEY_VARIABLE_LOCALVALUE.join(id.as_str());
            if let Err(e) = PutItem::put(
                self.variable_store.as_ref(),
                ctx,
                segkey_localvalue,
                AnyValue::serialize(&var_to_add.local_value)?,
            )
            .await
            {
                // TODO: log error
                println!("failed to put local_value in the db: {}", e);
            }

            let segkey_order = SEGKEY_VARIABLE_ORDER.join(id.as_str());
            if let Err(e) = PutItem::put(
                self.variable_store.as_ref(),
                ctx,
                segkey_order,
                AnyValue::serialize(&order)?,
            )
            .await
            {
                // TODO: log error
                println!("failed to put order in the db: {}", e);
            }
        }

        for var_to_update in params.vars_to_update {
            if let Some(new_name) = var_to_update.name {
                patches.push(PatchOperation::Replace(ReplaceOperation {
                    path: unsafe {
                        PointerBuf::new_unchecked(format!("/variable/{}/name", var_to_update.id))
                    },
                    value: JsonValue::String(new_name),
                }));
            }

            match var_to_update.global_value {
                Some(ChangeJsonValue::Update(value)) => {
                    patches.push(PatchOperation::Replace(ReplaceOperation {
                        path: unsafe {
                            PointerBuf::new_unchecked(format!(
                                "/variable/{}/value",
                                var_to_update.id
                            ))
                        },
                        value,
                    }));
                }
                Some(ChangeJsonValue::Remove) => {
                    patches.push(PatchOperation::Remove(RemoveOperation {
                        path: unsafe {
                            PointerBuf::new_unchecked(format!(
                                "/variable/{}/value",
                                var_to_update.id
                            ))
                        },
                    }));
                }
                _ => {}
            }

            let segkey_localvalue = SEGKEY_VARIABLE_LOCALVALUE.join(var_to_update.id.as_str());
            match var_to_update.local_value {
                Some(ChangeJsonValue::Update(value)) => {
                    if let Err(e) = PutItem::put(
                        self.variable_store.as_ref(),
                        ctx,
                        segkey_localvalue,
                        AnyValue::serialize(&value)?,
                    )
                    .await
                    {
                        // TODO: log error
                        println!("failed to put local_value in the db: {}", e);
                    }
                }
                Some(ChangeJsonValue::Remove) => {
                    if let Err(e) =
                        RemoveItem::remove(self.variable_store.as_ref(), ctx, segkey_localvalue)
                            .await
                    {
                        // TODO: log error
                        println!("failed to remove local_value from the db: {}", e);
                    }
                }
                _ => {}
            }

            let segkey_order = SEGKEY_VARIABLE_ORDER.join(var_to_update.id.as_str());
            match var_to_update.order {
                Some(order) => {
                    if let Err(e) = PutItem::put(
                        self.variable_store.as_ref(),
                        ctx,
                        segkey_order,
                        AnyValue::serialize(&order)?,
                    )
                    .await
                    {
                        // TODO: log error
                        println!("failed to put local_value in the db: {}", e);
                    }
                }
                None => {}
            }

            match var_to_update.desc {
                Some(ChangeString::Update(value)) => {
                    patches.push(PatchOperation::Replace(ReplaceOperation {
                        path: unsafe {
                            PointerBuf::new_unchecked(format!(
                                "/variable/{}/description",
                                var_to_update.id
                            ))
                        },
                        value: JsonValue::String(value),
                    }));
                }
                Some(ChangeString::Remove) => {
                    patches.push(PatchOperation::Remove(RemoveOperation {
                        path: unsafe {
                            PointerBuf::new_unchecked(format!(
                                "/variable/{}/description",
                                var_to_update.id
                            ))
                        },
                    }));
                }
                _ => {}
            }
        }

        for id in params.vars_to_delete {
            patches.push(PatchOperation::Remove(RemoveOperation {
                path: unsafe { PointerBuf::new_unchecked(format!("/variable/{}", id)) },
            }));

            let segkey_localvalue = SEGKEY_VARIABLE_LOCALVALUE.join(id.as_str());
            if let Err(e) =
                RemoveItem::remove(self.variable_store.as_ref(), ctx, segkey_localvalue).await
            {
                // TODO: log error
                println!("failed to remove local_value from the db: {}", e);
            }

            let segkey_order = SEGKEY_VARIABLE_ORDER.join(id.as_str());
            if let Err(e) =
                RemoveItem::remove(self.variable_store.as_ref(), ctx, segkey_order).await
            {
                // TODO: log error
                println!("failed to remove order from the db: {}", e);
            }
        }

        self.edit
            .edit(&patches)
            .await
            .join_err::<()>("failed to edit environment")?;

        Ok(())
    }
}
