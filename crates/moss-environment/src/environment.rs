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
    common::{VariableEntity, VariableStore},
    primitives::segkey::SegKeyBuf,
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

                let (local_value, order) = {
                    let segkey = SegKeyBuf::from(id.as_ref());
                    // TODO: log error when failed to get variable entity from the database
                    let entity = GetItem::get(self.variable_store.as_ref(), ctx, segkey)
                        .await
                        .ok()
                        .and_then(|v| v.deserialize::<VariableEntity>().ok());

                    if let Some(entity) = entity {
                        (entity.local_value, entity.order)
                    } else {
                        (None, 0)
                    }
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
            let entity = VariableEntity {
                local_value: Some(var_to_add.local_value.clone()),
                order,
            };

            let segkey = SegKeyBuf::from(id.as_str());
            PutItem::put(
                self.variable_store.as_ref(),
                ctx,
                segkey,
                AnyValue::serialize(&entity)?,
            )
            .await?;
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

            let needs_update_db =
                var_to_update.local_value.is_some() || var_to_update.order.is_some();

            if needs_update_db {
                let segkey = SegKeyBuf::from(var_to_update.id.as_str());
                let mut new_entity: VariableEntity =
                    GetItem::get(self.variable_store.as_ref(), ctx, segkey.clone())
                        .await?
                        .deserialize()?;

                match var_to_update.local_value {
                    Some(ChangeJsonValue::Update(value)) => {
                        new_entity.local_value = Some(value);
                    }
                    Some(ChangeJsonValue::Remove) => {
                        new_entity.local_value = None;
                    }
                    _ => {}
                }

                match var_to_update.order {
                    Some(new_order) => {
                        new_entity.order = new_order;
                    }
                    None => {}
                }

                PutItem::put(
                    self.variable_store.as_ref(),
                    ctx,
                    segkey,
                    AnyValue::serialize(&new_entity)?,
                )
                .await?;
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

            let segkey = SegKeyBuf::from(id.as_str());

            RemoveItem::remove(self.variable_store.as_ref(), ctx, segkey).await?;
        }

        self.edit
            .edit(&patches)
            .await
            .join_err::<()>("failed to edit environment")?;

        Ok(())
    }
}
