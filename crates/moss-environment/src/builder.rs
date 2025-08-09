use indexmap::IndexMap;
use joinerror::{Error, ResultExt};
use moss_applib::AppRuntime;
use moss_common::continue_if_err;
use moss_db::primitives::AnyValue;
use moss_fs::{CreateOptions, FileSystem, FsResultExt};
use moss_hcl::{Block, HclResultExt, LabeledBlock, json_to_hcl};
use moss_storage::{
    common::VariableStore, primitives::segkey::SegKeyBuf, storage::operations::TransactionalPutItem,
};
use serde_json::Value as JsonValue;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::watch;

use crate::{
    configuration::{MetadataDecl, SourceFile, VariableDecl},
    edit::EnvironmentEditing,
    environment::{Environment, EnvironmentPath},
    errors::{
        ErrorEnvironmentAlreadyExists, ErrorEnvironmentNotFound, ErrorFailedToEncode, ErrorIo,
    },
    models::{
        primitives::{EnvironmentId, VariableId},
        types::AddVariableParams,
    },
    segments::{SEGKEY_VARIABLE_LOCALVALUE, SEGKEY_VARIABLE_ORDER},
    utils,
};

pub struct CreateEnvironmentParams<'a> {
    pub name: String,
    pub abs_path: &'a Path,
    pub color: Option<String>,
    pub order: isize,
    pub variables: Vec<AddVariableParams>,
}

pub struct EnvironmentLoadParams {
    pub abs_path: PathBuf,
}

pub struct EnvironmentBuilder {
    fs: Arc<dyn FileSystem>,
    vars_to_store: HashMap<VariableId, (JsonValue, isize)>,
}

impl EnvironmentBuilder {
    pub fn new(fs: Arc<dyn FileSystem>) -> Self {
        Self {
            fs,
            vars_to_store: HashMap::new(),
        }
    }

    pub async fn initialize<'a>(
        &mut self,
        params: CreateEnvironmentParams<'a>,
    ) -> joinerror::Result<PathBuf> {
        debug_assert!(params.abs_path.is_absolute());

        let id = EnvironmentId::new();
        let file_name = utils::format_file_name(&params.name);
        let abs_path = params.abs_path.join(&file_name);
        if abs_path.exists() {
            return Err(Error::new::<ErrorEnvironmentAlreadyExists>(
                abs_path.display().to_string(),
            ));
        }

        let mut variables = IndexMap::with_capacity(params.variables.len());
        for v in params.variables {
            let global_value = continue_if_err!(json_to_hcl(&v.global_value), |err| {
                println!("failed to convert global value expression: {}", err); // TODO: log error
            });

            let id = VariableId::new();

            variables.insert(
                id.clone(),
                VariableDecl {
                    name: v.name,
                    value: global_value,
                    description: v.desc,
                    options: v.options,
                },
            );

            // We don't save data to the store here because we don't want to pass the store as a parameter to this function.
            // When the environment is simply being initialized, we might not yet have access to the store where variable data could be saved.
            self.vars_to_store.insert(id, (v.local_value, v.order));
        }

        let content = hcl::to_string(&SourceFile {
            metadata: Block::new(MetadataDecl {
                id: id.clone(),
                color: params.color,
            }),
            variables: Some(LabeledBlock::new(variables)),
        })
        .join_err_with::<ErrorFailedToEncode>(|| {
            format!("failed to encode environment file {}", abs_path.display())
        })?;

        self.fs
            .create_file_with(
                &abs_path,
                content.as_bytes(),
                CreateOptions {
                    overwrite: false,
                    ignore_if_exists: false,
                },
            )
            .await
            .join_err_with::<ErrorIo>(|| {
                format!("failed to create environment file {}", abs_path.display())
            })?;

        Ok(abs_path)
    }

    pub async fn create<'a, R: AppRuntime>(
        mut self,
        ctx: &R::AsyncContext,
        variable_store: Arc<dyn VariableStore<R::AsyncContext>>,
        params: CreateEnvironmentParams<'a>,
    ) -> joinerror::Result<Environment<R>> {
        debug_assert!(params.abs_path.is_absolute());

        let abs_path = self
            .initialize(params)
            .await
            .join_err::<()>("failed to initialize environment")?;

        for (id, (local_value, order)) in self.vars_to_store.drain() {
            let local_value = continue_if_err!(AnyValue::serialize(&local_value), |err| {
                println!("failed to serialize localvalue: {}", err);
            });

            let order = continue_if_err!(AnyValue::serialize(&order), |err| {
                println!("failed to serialize order: {}", err);
            });

            let mut txn = continue_if_err!(variable_store.begin_write(&ctx).await, |err| {
                println!("failed to start a write transaction: {}", err);
            });

            continue_if_err!(
                TransactionalPutItem::put_with_context(
                    variable_store.as_ref(),
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
            continue_if_err!(
                TransactionalPutItem::put_with_context(
                    variable_store.as_ref(),
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

        let (abs_path_tx, abs_path_rx) = watch::channel(EnvironmentPath::new(abs_path)?);

        Ok(Environment {
            fs: self.fs.clone(),
            abs_path_rx,
            edit: EnvironmentEditing::new(self.fs.clone(), abs_path_tx),
            variable_store,
        })
    }

    pub async fn load<R: AppRuntime>(
        self,
        variable_store: Arc<dyn VariableStore<R::AsyncContext>>,
        params: EnvironmentLoadParams,
    ) -> joinerror::Result<Environment<R>> {
        debug_assert!(params.abs_path.is_absolute());

        if !params.abs_path.exists() {
            return Err(Error::new::<ErrorEnvironmentNotFound>(
                params.abs_path.display().to_string(),
            ));
        }

        let (abs_path_tx, abs_path_rx) = watch::channel(EnvironmentPath::new(params.abs_path)?);

        Ok(Environment {
            fs: self.fs.clone(),
            abs_path_rx,
            edit: EnvironmentEditing::new(self.fs.clone(), abs_path_tx),
            variable_store,
        })
    }
}
