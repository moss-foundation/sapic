use hcl::ser::LabeledBlock;
use indexmap::IndexMap;
use moss_applib::{AppRuntime, errors::ValidationResultExt};
use moss_common::continue_if_err;
use moss_hcl::{Block, json_to_hcl};
use moss_logging::session;
use std::collections::HashMap;
use validator::Validate;

use crate::{
    Project,
    models::{
        operations::CreateEntryOutput,
        primitives::{
            EntryClass, EntryId, EntryProtocol, FrontendEntryPath, HeaderId, PathParamId,
            QueryParamId,
        },
        types::{
            AfterUpdateDirEntryDescription, AfterUpdateItemEntryDescription, CreateDirEntryParams,
            CreateItemEntryParams, UpdateDirEntryParams, UpdateItemEntryParams,
        },
    },
    worktree::{
        ModifyParams,
        entry::model::{
            EntryMetadataSpec, EntryModel, HeaderParamSpec, HeaderParamSpecOptions, PathParamSpec,
            PathParamSpecOptions, QueryParamSpec, QueryParamSpecOptions, UrlDetails,
        },
    },
};

impl<R: AppRuntime> Project<R> {
    pub(super) async fn create_dir_entry(
        &self,
        ctx: &R::AsyncContext,
        input: CreateDirEntryParams,
    ) -> joinerror::Result<CreateEntryOutput> {
        input.validate().join_err_bare()?;

        let id = EntryId::new();
        let model = EntryModel::from((id.clone(), input.class));

        self.worktree()
            .await
            .create_dir_entry(
                ctx,
                &input.name,
                &input.path,
                model,
                input.order,
                true, // Directories are automatically marked as expanded
            )
            .await?;

        Ok(CreateEntryOutput { id: id })
    }

    pub(super) async fn create_item_entry(
        &self,
        ctx: &R::AsyncContext,
        input: CreateItemEntryParams,
    ) -> joinerror::Result<CreateEntryOutput> {
        input.validate().join_err_bare()?;

        let id = EntryId::new();

        match &input.class {
            EntryClass::Endpoint => self.create_endpoint_entry(id, ctx, input).await,
            _ => {
                let model = EntryModel {
                    metadata: Block::new(EntryMetadataSpec {
                        id: id.clone(),
                        class: input.class,
                    }),
                    url: None,
                    headers: None, // Hardcoded for now
                    path_params: None,
                    query_params: None,
                    body: None,
                };
                self.worktree()
                    .await
                    .create_item_entry(ctx, &input.name, &input.path, model, input.order, false)
                    .await?;
                Ok(CreateEntryOutput { id: id })
            }
        }
    }

    pub(super) async fn update_item_entry(
        &self,
        ctx: &R::AsyncContext,
        input: UpdateItemEntryParams,
    ) -> joinerror::Result<AfterUpdateItemEntryDescription> {
        input.validate().join_err_bare()?;

        let path = self
            .worktree()
            .await
            .update_item_entry(
                ctx,
                &input.id,
                ModifyParams {
                    name: input.name,
                    protocol: input.protocol,
                    expanded: input.expanded,
                    order: input.order,
                    path: input.path,

                    query_params_to_add: input.query_params_to_add,
                    query_params_to_update: input.query_params_to_update,
                    query_params_to_remove: input.query_params_to_remove,

                    path_params_to_add: input.path_params_to_add,
                    path_params_to_update: input.path_params_to_update,
                    path_params_to_remove: input.path_params_to_remove,

                    headers_to_add: input.headers_to_add,
                    headers_to_update: input.headers_to_update,
                    headers_to_remove: input.headers_to_remove,
                },
            )
            .await?;

        let path = FrontendEntryPath::new(path.to_path_buf());

        Ok(AfterUpdateItemEntryDescription { id: input.id, path })
    }

    pub(super) async fn update_dir_entry(
        &self,
        ctx: &R::AsyncContext,
        input: UpdateDirEntryParams,
    ) -> joinerror::Result<AfterUpdateDirEntryDescription> {
        input.validate().join_err_bare()?;

        let path = self
            .worktree()
            .await
            .update_dir_entry(
                ctx,
                &input.id,
                ModifyParams {
                    name: input.name,
                    order: input.order,
                    expanded: input.expanded,
                    protocol: None,
                    path: input.path,

                    headers_to_add: vec![],
                    headers_to_update: vec![],
                    headers_to_remove: vec![],

                    path_params_to_add: vec![],
                    path_params_to_update: vec![],
                    path_params_to_remove: vec![],

                    query_params_to_add: vec![],
                    query_params_to_update: vec![],
                    query_params_to_remove: vec![],
                },
            )
            .await?;

        let path = FrontendEntryPath::new(path.to_path_buf());

        Ok(AfterUpdateDirEntryDescription { id: input.id, path })
    }
}

impl<R: AppRuntime> Project<R> {
    async fn create_endpoint_entry(
        &self,
        id: EntryId,
        ctx: &R::AsyncContext,
        input: CreateItemEntryParams,
    ) -> joinerror::Result<CreateEntryOutput> {
        let mut header_map = IndexMap::new();
        let mut header_orders = HashMap::new();
        let mut path_param_map = IndexMap::new();
        let mut path_param_orders = HashMap::new();
        let mut query_param_map = IndexMap::new();
        let mut query_param_orders = HashMap::new();

        for param in &input.headers {
            let id = HeaderId::new();
            let value = continue_if_err!(json_to_hcl(&param.value), |err| {
                session::error!("failed to convert value expression: {}", err)
            });

            header_map.insert(
                id.clone(),
                HeaderParamSpec {
                    name: param.name.clone(),
                    value,
                    description: param.desc.clone(),
                    options: HeaderParamSpecOptions {
                        disabled: param.options.disabled,
                        propagate: param.options.propagate,
                    },
                },
            );
            header_orders.insert(id.clone(), param.order.clone());
        }

        for param in &input.path_params {
            let id = PathParamId::new();
            let value = continue_if_err!(json_to_hcl(&param.value), |err| {
                session::error!("failed to convert value expression: {}", err)
            });

            path_param_map.insert(
                id.clone(),
                PathParamSpec {
                    name: param.name.clone(),
                    value,
                    description: param.desc.clone(),
                    options: PathParamSpecOptions {
                        disabled: param.options.disabled,
                        propagate: param.options.propagate,
                    },
                },
            );
            path_param_orders.insert(id.clone(), param.order.clone());
        }

        for param in &input.query_params {
            let id = QueryParamId::new();
            let value = continue_if_err!(json_to_hcl(&param.value), |err| {
                session::error!("failed to convert value expression: {}", err)
            });

            query_param_map.insert(
                id.clone(),
                QueryParamSpec {
                    name: param.name.clone(),
                    value,
                    description: param.desc.clone(),
                    options: QueryParamSpecOptions {
                        disabled: param.options.disabled,
                        propagate: param.options.propagate,
                    },
                },
            );
            query_param_orders.insert(id.clone(), param.order.clone());
        }

        let model = EntryModel {
            metadata: Block::new(EntryMetadataSpec {
                id: id.clone(),
                class: input.class.clone(),
            }),
            url: Some(Block::new(UrlDetails {
                protocol: input.protocol.clone().unwrap_or(EntryProtocol::Get),
                raw: "Hardcoded Value".to_string(),
            })),
            headers: Some(LabeledBlock::new(header_map)),
            path_params: Some(LabeledBlock::new(path_param_map)),
            query_params: Some(LabeledBlock::new(query_param_map)),
            body: None, // TODO
        };

        self.worktree()
            .await
            .create_item_entry(ctx, &input.name, &input.path, model, input.order, false)
            .await?;

        let output = CreateEntryOutput { id: id.clone() };

        if header_orders.is_empty() && path_param_orders.is_empty() && query_param_orders.is_empty()
        {
            return Ok(output);
        }

        // Storing param orders
        let mut txn = match self.storage_service.begin_write(ctx).await {
            Ok(txn) => txn,
            Err(e) => {
                session::error!(format!("failed to begin write transaction: {}", e));
                return Ok(output);
            }
        };

        for (header_id, order) in header_orders {
            continue_if_err!(
                self.storage_service
                    .put_entry_header_order_txn(ctx, &mut txn, &id, &header_id, order,)
                    .await,
                |err| {
                    session::error!(format!("failed to put header order: {}", err));
                }
            )
        }

        for (path_param_id, order) in path_param_orders {
            continue_if_err!(
                self.storage_service
                    .put_entry_path_param_order_txn(ctx, &mut txn, &id, &path_param_id, order,)
                    .await,
                |err| {
                    session::error!(format!("failed to put path param order: {}", err));
                }
            )
        }

        for (query_param_id, order) in query_param_orders {
            continue_if_err!(
                self.storage_service
                    .put_entry_query_param_order_txn(ctx, &mut txn, &id, &query_param_id, order,)
                    .await,
                |err| {
                    session::error!(format!("failed to put query param order: {}", err));
                }
            )
        }

        if let Err(e) = txn.commit() {
            session::error!(format!("failed to commit write transaction: {}", e));
        }

        Ok(output)
    }
}
