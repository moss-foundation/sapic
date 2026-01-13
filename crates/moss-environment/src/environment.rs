use derive_more::Deref;
use joinerror::{OptionExt, ResultExt};
use json_patch::{
    AddOperation, PatchOperation, RemoveOperation, ReplaceOperation, jsonptr::PointerBuf,
};
use moss_bindingutils::primitives::{ChangeJsonValue, ChangeString};
use moss_common::continue_if_err;
use moss_edit::json::EditOptions;
use moss_fs::FileSystem;
use moss_hcl::{HclResultExt, hcl_to_json, json_to_hcl};
use moss_logging::session;
use moss_storage2::{KvStorage, models::primitives::StorageScope};
use sapic_base::environment::types::{
    VariableInfo,
    primitives::{EnvironmentId, VariableId},
};
use sapic_core::context::AnyAsyncContext;
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
    storage::{key_variable, key_variable_local_value},
    utils,
};

pub struct Environment {
    pub(super) id: EnvironmentId,
    pub(super) fs: Arc<dyn FileSystem>,
    pub(super) storage: Arc<dyn KvStorage>,
    pub(super) abs_path: PathBuf,
    // FIXME: Should project environments be stored in the project database?
    // Environment variables are stored in workspace database
    // We use Arc<String> instead of WorkspaceId to avoid circular dependency
    pub(super) workspace_id: Arc<String>,
}

unsafe impl Send for Environment {}
unsafe impl Sync for Environment {}

impl AnyEnvironment for Environment {
    async fn abs_path(&self) -> &Path {
        &self.abs_path
    }

    // TODO: add variables()

    // TODO: rename to details
    // FIXME: Should this be handled by the environment service? I'll keep it for now
    // async fn describe(&self, ctx: &dyn AnyAsyncContext) -> joinerror::Result<DescribeEnvironment> {
    //     let abs_path = self.abs_path().await;
    //     let rdr = self
    //         .fs
    //         .open_file(ctx, abs_path)
    //         .await
    //         .join_err_with::<()>(|| format!("failed to open file: {}", abs_path.display()))?;
    //
    //     let parsed: SourceFile = hcl::from_reader(rdr)
    //         .join_err_with::<()>(|| format!("failed to parse hcl: {}", abs_path.display()))?;
    //
    //     let mut variables =
    //         HashMap::with_capacity(parsed.variables.as_ref().map_or(0, |v| v.len()));
    //
    //     if let Some(vars) = parsed.variables.as_ref() {
    //         let storage_scope = StorageScope::Workspace(self.workspace_id.clone());
    //
    //         for (var_id, var) in vars.iter() {
    //             let global_value = continue_if_err!(hcl_to_json(&var.value), |err| {
    //                 println!("failed to convert global value expression: {}", err); // TODO: log error
    //             });
    //
    //             let local_value: Option<JsonValue> = self
    //                 .storage
    //                 .get(
    //                     ctx,
    //                     storage_scope.clone(),
    //                     &key_variable_local_value(&self.id, var_id),
    //                 )
    //                 .await
    //                 .unwrap_or_else(|e| {
    //                     session::warn!(format!(
    //                         "failed to get variable localValue from the database: {}",
    //                         e
    //                     ));
    //                     None
    //                 });
    //
    //             // FIXME: Should the variables be cached?
    //             variables.insert(
    //                 var_id.clone(),
    //                 VariableInfo {
    //                     id: var_id.clone(),
    //                     name: var.name.clone(),
    //                     global_value: Some(global_value),
    //                     local_value,
    //                     disabled: var.options.disabled,
    //                     order: None, // TODO: REMOVE
    //                     desc: var.description.clone(),
    //                 },
    //             );
    //         }
    //     }
    //
    //     Ok(DescribeEnvironment {
    //         id: self.id.clone(),
    //         abs_path: abs_path.into(),
    //         color: parsed.metadata.color.clone(),
    //         name: parsed.metadata.name.clone(),
    //         variables,
    //     })
    // }

    // async fn modify(
    //     &self,
    //     ctx: &dyn AnyAsyncContext,
    //     params: ModifyEnvironmentParams,
    // ) -> joinerror::Result<()> {
    //     TODO: Move file modify part to EnvironmentEdit
    //     We can keep the local value update logic here
    //     if let Some(new_name) = params.name {
    //         self.edit.rename(ctx, &new_name).await?;
    //     }
    //
    //     let mut patches = Vec::new();
    //
    //     match params.color {
    //         Some(ChangeString::Update(color)) => {
    //             patches.push((
    //                 PatchOperation::Add(AddOperation {
    //                     path: unsafe { PointerBuf::new_unchecked("/metadata/color") },
    //                     value: JsonValue::String(color),
    //                 }),
    //                 EditOptions {
    //                     create_missing_segments: true,
    //                     ignore_if_not_exists: false,
    //                 },
    //             ));
    //         }
    //         Some(ChangeString::Remove) => {
    //             patches.push((
    //                 PatchOperation::Remove(RemoveOperation {
    //                     path: unsafe { PointerBuf::new_unchecked("/metadata/color") },
    //                 }),
    //                 EditOptions {
    //                     create_missing_segments: false,
    //                     ignore_if_not_exists: false,
    //                 },
    //             ));
    //         }
    //         _ => {}
    //     };
    //
    //     let storage_scope = StorageScope::Workspace(self.workspace_id.clone());
    //     for var_to_add in params.vars_to_add {
    //         let id = VariableId::new();
    //         let id_str = id.to_string();
    //
    //         let global_value = continue_if_err!(json_to_hcl(&var_to_add.global_value), |err| {
    //             println!("failed to convert global value expression: {}", err); // TODO: log error
    //         });
    //
    //         let decl = VariableDecl {
    //             name: var_to_add.name.clone(),
    //             value: global_value,
    //             description: var_to_add.desc.clone(),
    //             options: var_to_add.options.clone(),
    //         };
    //
    //         let value = continue_if_err!(serde_json::to_value(decl), |err| {
    //             println!("failed to convert variable declaration to json: {}", err); // TODO: log error
    //         });
    //
    //         patches.push((
    //             PatchOperation::Add(AddOperation {
    //                 path: unsafe { PointerBuf::new_unchecked(format!("/variable/{}", id_str)) },
    //                 value,
    //             }),
    //             EditOptions {
    //                 create_missing_segments: true,
    //                 ignore_if_not_exists: false,
    //             },
    //         ));
    //
    //         let local_value_key = key_variable_local_value(&id);
    //         let order_key = key_variable_order(&id);
    //
    //         let batch_input = vec![
    //             (local_value_key.as_str(), var_to_add.local_value),
    //             (order_key.as_str(), serde_json::to_value(&var_to_add.order)?),
    //         ];
    //
    //         if let Err(e) = self
    //             .storage
    //             .put_batch(ctx, storage_scope.clone(), &batch_input)
    //             .await
    //         {
    //             session::warn!(format!(
    //                 "failed to add variable cache to the database: {}",
    //                 e
    //             ));
    //         }
    //     }
    //
    //     for var_to_update in params.vars_to_update {
    //         if let Some(new_name) = var_to_update.name {
    //             patches.push((
    //                 PatchOperation::Replace(ReplaceOperation {
    //                     path: unsafe {
    //                         PointerBuf::new_unchecked(format!(
    //                             "/variable/{}/name",
    //                             var_to_update.id
    //                         ))
    //                     },
    //                     value: JsonValue::String(new_name),
    //                 }),
    //                 EditOptions {
    //                     // Raise an error if the variable does not exist
    //                     create_missing_segments: false,
    //                     ignore_if_not_exists: false,
    //                 },
    //             ));
    //         }
    //
    //         match var_to_update.global_value {
    //             Some(ChangeJsonValue::Update(value)) => {
    //                 patches.push((
    //                     PatchOperation::Replace(ReplaceOperation {
    //                         path: unsafe {
    //                             PointerBuf::new_unchecked(format!(
    //                                 "/variable/{}/value",
    //                                 var_to_update.id
    //                             ))
    //                         },
    //                         value,
    //                     }),
    //                     EditOptions {
    //                         // Raise an error if the variable does not exist
    //                         create_missing_segments: false,
    //                         ignore_if_not_exists: false,
    //                     },
    //                 ));
    //             }
    //             Some(ChangeJsonValue::Remove) => {
    //                 patches.push((
    //                     PatchOperation::Remove(RemoveOperation {
    //                         path: unsafe {
    //                             PointerBuf::new_unchecked(format!(
    //                                 "/variable/{}/value",
    //                                 var_to_update.id
    //                             ))
    //                         },
    //                     }),
    //                     EditOptions {
    //                         // Raise an error if the variable does not exist
    //                         create_missing_segments: false,
    //                         ignore_if_not_exists: false,
    //                     },
    //                 ));
    //             }
    //             _ => {}
    //         }
    //
    //         match var_to_update.desc {
    //             Some(ChangeString::Update(value)) => {
    //                 patches.push((
    //                     PatchOperation::Replace(ReplaceOperation {
    //                         path: unsafe {
    //                             PointerBuf::new_unchecked(format!(
    //                                 "/variable/{}/description",
    //                                 var_to_update.id
    //                             ))
    //                         },
    //                         value: JsonValue::String(value),
    //                     }),
    //                     EditOptions {
    //                         // Raise an error if the variable does not exist
    //                         create_missing_segments: false,
    //                         ignore_if_not_exists: false,
    //                     },
    //                 ));
    //             }
    //             Some(ChangeString::Remove) => {
    //                 patches.push((
    //                     PatchOperation::Remove(RemoveOperation {
    //                         path: unsafe {
    //                             PointerBuf::new_unchecked(format!(
    //                                 "/variable/{}/description",
    //                                 var_to_update.id
    //                             ))
    //                         },
    //                     }),
    //                     EditOptions {
    //                         // Raise an error if the variable does not exist
    //                         create_missing_segments: false,
    //                         ignore_if_not_exists: false,
    //                     },
    //                 ));
    //             }
    //             _ => {}
    //         }
    //
    //         if let Some(options) = var_to_update.options {
    //             let options_value = continue_if_err!(serde_json::to_value(options), |err| {
    //                 println!("failed to convert variable options to json: {}", err); // TODO: log error
    //             });
    //
    //             patches.push((
    //                 PatchOperation::Replace(ReplaceOperation {
    //                     path: unsafe {
    //                         PointerBuf::new_unchecked(format!(
    //                             "/variable/{}/options",
    //                             var_to_update.id
    //                         ))
    //                     },
    //                     value: options_value,
    //                 }),
    //                 EditOptions {
    //                     // Raise an error if the variable does not exist
    //                     create_missing_segments: false,
    //                     ignore_if_not_exists: false,
    //                 },
    //             ));
    //         }
    //
    //         let local_value_key = key_variable_local_value(&var_to_update.id);
    //         match var_to_update.local_value {
    //             Some(ChangeJsonValue::Update(value)) => {
    //                 if let Err(e) = self
    //                     .storage
    //                     .put(ctx, storage_scope.clone(), &local_value_key, value)
    //                     .await
    //                 {
    //                     session::warn!(format!("failed to update variable localValue: {}", e));
    //                 }
    //             }
    //             Some(ChangeJsonValue::Remove) => {
    //                 if let Err(e) = self
    //                     .storage
    //                     .remove(ctx, storage_scope.clone(), &local_value_key)
    //                     .await
    //                 {
    //                     session::warn!(format!("failed to remove variable localValue: {}", e));
    //                 }
    //             }
    //             None => {}
    //         }
    //     }
    //
    //     for id in params.vars_to_delete {
    //         patches.push((
    //             PatchOperation::Remove(RemoveOperation {
    //                 path: unsafe { PointerBuf::new_unchecked(format!("/variable/{}", id)) },
    //             }),
    //             EditOptions {
    //                 create_missing_segments: false,
    //                 // Skip the operation if the variable does not exist
    //                 // FIXME: How should we write logs when that happens?
    //                 ignore_if_not_exists: true,
    //             },
    //         ));
    //
    //         if let Err(e) = self
    //             .storage
    //             .remove_batch_by_prefix(ctx, storage_scope.clone(), &key_variable(&id))
    //             .await
    //         {
    //             session::warn!(format!(
    //                 "failed to remove variable cache from database: {}",
    //                 e
    //             ));
    //         }
    //     }
    //
    //     self.edit
    //         .edit(ctx, &patches)
    //         .await
    //         .join_err::<()>("failed to edit environment")?;
    //
    //     Ok(())
    // }
}
