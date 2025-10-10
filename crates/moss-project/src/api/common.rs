use hcl::ser::LabeledBlock;
use indexmap::{IndexMap, indexmap};
use moss_app_delegate::AppDelegate;
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
            EntryClass, EntryId, EntryProtocol, FormDataParamId, FrontendEntryPath, HeaderId,
            PathParamId, QueryParamId, UrlencodedParamId,
        },
        types::{
            AfterUpdateDirEntryDescription, AfterUpdateItemEntryDescription, CreateDirEntryParams,
            CreateItemEntryParams, UpdateDirEntryParams, UpdateItemEntryParams,
            http::AddBodyParams,
        },
    },
    worktree::{
        ModifyParams,
        entry::model::{
            BodyKind, BodySpec, EntryMetadataSpec, EntryModel, FormDataParamSpec,
            FormDataParamSpecOptions, HeaderParamSpec, HeaderParamSpecOptions, PathParamSpec,
            PathParamSpecOptions, QueryParamSpec, QueryParamSpecOptions, UrlDetails,
            UrlencodedParamSpec, UrlencodedParamSpecOptions,
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
        app_delegate: &AppDelegate<R>,
        input: UpdateItemEntryParams,
    ) -> joinerror::Result<AfterUpdateItemEntryDescription> {
        input.validate().join_err_bare()?;

        let path = self
            .worktree()
            .await
            .update_item_entry(
                ctx,
                app_delegate,
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

                    body: input.body,
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

                    body: None,
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
        let mut urlencoded_param_orders = HashMap::new();
        let mut formdata_param_orders = HashMap::new();

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
                    description: param.description.clone(),
                    options: HeaderParamSpecOptions {
                        disabled: param.options.disabled,
                        propagate: param.options.propagate,
                    },
                },
            );
            header_orders.insert(id, param.order);
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
                    description: param.description.clone(),
                    options: PathParamSpecOptions {
                        disabled: param.options.disabled,
                        propagate: param.options.propagate,
                    },
                },
            );
            path_param_orders.insert(id.clone(), param.order.clone());
        }

        let body = if let Some(body_params) = input.body {
            Some(
                create_body_block(
                    body_params,
                    &mut urlencoded_param_orders,
                    &mut formdata_param_orders,
                )
                .await,
            )
        } else {
            None
        };

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
                    description: param.description.clone(),
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
            headers: if header_map.is_empty() {
                None
            } else {
                Some(LabeledBlock::new(header_map))
            },
            path_params: Some(LabeledBlock::new(path_param_map)),
            query_params: Some(LabeledBlock::new(query_param_map)),
            body,
        };

        self.worktree()
            .await
            .create_item_entry(ctx, &input.name, &input.path, model, input.order, false)
            .await?;

        let output = CreateEntryOutput { id: id.clone() };

        if header_orders.is_empty()
            && path_param_orders.is_empty()
            && query_param_orders.is_empty()
            && urlencoded_param_orders.is_empty()
            && formdata_param_orders.is_empty()
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

        for (urlencoded_param_id, order) in urlencoded_param_orders {
            continue_if_err!(
                self.storage_service
                    .put_entry_body_urlencoded_param_order_txn(
                        ctx,
                        &mut txn,
                        &id,
                        &urlencoded_param_id,
                        order,
                    )
                    .await,
                |err| {
                    session::error!(format!(
                        "failed to put entry body urlencoded param order: {}",
                        err
                    ));
                }
            )
        }

        for (formdata_param_id, order) in formdata_param_orders {
            continue_if_err!(
                self.storage_service
                    .put_entry_body_formdata_param_order_txn(
                        ctx,
                        &mut txn,
                        &id,
                        &formdata_param_id,
                        order,
                    )
                    .await,
                |err| {
                    session::error!(format!("failed to put entry formdata param order: {}", err));
                }
            )
        }

        if let Err(e) = txn.commit() {
            session::error!(format!("failed to commit write transaction: {}", e));
        }

        Ok(output)
    }
}

async fn create_body_block(
    params: AddBodyParams,
    urlencoded_param_orders: &mut HashMap<UrlencodedParamId, isize>,
    formdata_param_orders: &mut HashMap<FormDataParamId, isize>,
) -> LabeledBlock<IndexMap<BodyKind, BodySpec>> {
    let (kind, spec) = match params {
        AddBodyParams::Text(text) => (
            BodyKind::Text,
            BodySpec {
                text: Some(text),
                ..Default::default()
            },
        ),
        AddBodyParams::Json(json) => (
            BodyKind::Json,
            BodySpec {
                json: Some(json),
                ..Default::default()
            },
        ),
        AddBodyParams::Xml(xml) => (
            BodyKind::Xml,
            BodySpec {
                xml: Some(xml),
                ..Default::default()
            },
        ),
        AddBodyParams::Binary(path) => (
            BodyKind::Binary,
            BodySpec {
                binary: Some(path),
                ..Default::default()
            },
        ),
        AddBodyParams::Urlencoded(urlencoded) => {
            let mut urlencoded_map = IndexMap::with_capacity(urlencoded.len());
            for param in urlencoded {
                let id = param.id.unwrap_or(UrlencodedParamId::new());
                let value = continue_if_err!(json_to_hcl(&param.value), |err| {
                    session::error!("failed to convert value expression: {}", err)
                });

                urlencoded_map.insert(
                    id.clone(),
                    UrlencodedParamSpec {
                        name: param.name,
                        value,
                        description: param.description,
                        options: UrlencodedParamSpecOptions {
                            disabled: param.options.disabled,
                            propagate: param.options.propagate,
                        },
                    },
                );
                urlencoded_param_orders.insert(id.clone(), param.order);
            }
            let spec = BodySpec {
                urlencoded: Some(LabeledBlock::new(urlencoded_map)),
                ..Default::default()
            };
            (BodyKind::Urlencoded, spec)
        }
        AddBodyParams::FormData(form_data) => {
            let mut formdata_map = IndexMap::with_capacity(form_data.len());
            for param in form_data {
                let id = param.id.unwrap_or(FormDataParamId::new());
                let value = continue_if_err!(json_to_hcl(&param.value), |err| {
                    session::error!("failed to convert value expression: {}", err)
                });

                formdata_map.insert(
                    id.clone(),
                    FormDataParamSpec {
                        name: param.name,
                        value,
                        description: param.description,
                        options: FormDataParamSpecOptions {
                            disabled: param.options.disabled,
                            propagate: param.options.propagate,
                        },
                    },
                );
                formdata_param_orders.insert(id.clone(), param.order);
            }
            let spec = BodySpec {
                form_data: Some(LabeledBlock::new(formdata_map)),
                ..Default::default()
            };
            (BodyKind::FormData, spec)
        }
    };
    LabeledBlock::new(indexmap! {
        kind => spec
    })
}
