use hcl::ser::LabeledBlock;
use indexmap::{IndexMap, indexmap};
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use moss_common::continue_if_err;
use moss_hcl::{Block, json_to_hcl};
use moss_logging::session;
use moss_storage2::{Storage, models::primitives::StorageScope};
use sapic_ipc::ValidationResultExt;
use std::collections::HashMap;
use validator::Validate;

use crate::{
    Project,
    models::{
        operations::CreateResourceOutput,
        primitives::{
            FormDataParamId, FrontendResourcePath, HeaderId, PathParamId, QueryParamId,
            ResourceClass, ResourceId, ResourceProtocol, UrlencodedParamId,
        },
        types::{
            AfterUpdateDirResourceDescription, AfterUpdateItemResourceDescription,
            CreateDirResourceParams, CreateItemResourceParams, UpdateDirResourceParams,
            UpdateItemResourceParams, http::AddBodyParams,
        },
    },
    storage::{
        key_resource_body_formdata_param_order, key_resource_body_urlencoded_param_order,
        key_resource_header_order, key_resource_path_param_order, key_resource_query_param_order,
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
    pub(super) async fn create_dir_resource(
        &self,
        ctx: &R::AsyncContext,
        input: CreateDirResourceParams,
    ) -> joinerror::Result<CreateResourceOutput> {
        input.validate().join_err_bare()?;

        let id = ResourceId::new();
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

        Ok(CreateResourceOutput { id: id })
    }

    pub(super) async fn create_item_resource(
        &self,
        ctx: &R::AsyncContext,
        input: CreateItemResourceParams,
    ) -> joinerror::Result<CreateResourceOutput> {
        input.validate().join_err_bare()?;

        let id = ResourceId::new();

        match &input.class {
            ResourceClass::Endpoint => self.create_endpoint_resource(id, ctx, input).await,
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
                Ok(CreateResourceOutput { id: id })
            }
        }
    }

    pub(super) async fn update_item_resource(
        &self,
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        input: UpdateItemResourceParams,
    ) -> joinerror::Result<AfterUpdateItemResourceDescription> {
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

        let path = FrontendResourcePath::new(path.to_path_buf());

        Ok(AfterUpdateItemResourceDescription { id: input.id, path })
    }

    pub(super) async fn update_dir_resource(
        &self,
        ctx: &R::AsyncContext,
        input: UpdateDirResourceParams,
    ) -> joinerror::Result<AfterUpdateDirResourceDescription> {
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

        let path = FrontendResourcePath::new(path.to_path_buf());

        Ok(AfterUpdateDirResourceDescription { id: input.id, path })
    }
}

impl<R: AppRuntime> Project<R> {
    async fn create_endpoint_resource(
        &self,
        id: ResourceId,
        ctx: &R::AsyncContext,
        input: CreateItemResourceParams,
    ) -> joinerror::Result<CreateResourceOutput> {
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
                protocol: input.protocol.clone().unwrap_or(ResourceProtocol::Get),
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

        let output = CreateResourceOutput { id: id.clone() };

        if header_orders.is_empty()
            && path_param_orders.is_empty()
            && query_param_orders.is_empty()
            && urlencoded_param_orders.is_empty()
            && formdata_param_orders.is_empty()
        {
            return Ok(output);
        }

        // FIXME: Find a better way to convert to &[(&str, JsonValue)]

        let mut key_values = vec![];
        for (header_id, order) in header_orders {
            key_values.push((
                key_resource_header_order(&id, &header_id),
                serde_json::to_value(&order)?,
            ));
        }

        for (path_param_id, order) in path_param_orders {
            key_values.push((
                key_resource_path_param_order(&id, &path_param_id),
                serde_json::to_value(&order)?,
            ));
        }

        for (query_param_id, order) in query_param_orders {
            key_values.push((
                key_resource_query_param_order(&id, &query_param_id),
                serde_json::to_value(&order)?,
            ));
        }

        for (urlencoded_param_id, order) in urlencoded_param_orders {
            key_values.push((
                key_resource_body_urlencoded_param_order(&id, &urlencoded_param_id),
                serde_json::to_value(&order)?,
            ));
        }

        for (formdata_param_id, order) in formdata_param_orders {
            key_values.push((
                key_resource_body_formdata_param_order(&id, &formdata_param_id),
                serde_json::to_value(&order)?,
            ));
        }

        let mut batch_input = Vec::new();
        for (key, value) in key_values.iter() {
            batch_input.push((key.as_str(), value.clone()));
        }

        let storage = <dyn Storage>::global(&self.app_delegate);
        let storage_scope = StorageScope::Project(self.id.inner());

        if let Err(e) = storage.put_batch(storage_scope, &batch_input).await {
            session::warn!(format!(
                "failed to update database after creating endpoint resource: {}",
                e
            ));
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
