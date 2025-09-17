use hcl::ser::LabeledBlock;
use indexmap::IndexMap;
use joinerror::Error;
use moss_applib::{AppRuntime, errors::ValidationResultExt};
use moss_hcl::{Block, json_to_hcl};
use validator::Validate;

use crate::{
    Project,
    errors::ErrorInvalidInput,
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
            EntryMetadataSpec, EntryModel, HeaderParamOptions, HeaderParamSpec, PathParamOptions,
            PathParamSpec, QueryParamOptions, QueryParamSpec, UrlDetails,
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

        let model = match input.class {
            EntryClass::Endpoint => {
                let mut header_map = IndexMap::new();
                let mut path_param_map = IndexMap::new();
                let mut query_param_map = IndexMap::new();

                for param in input.headers {
                    header_map.insert(
                        HeaderId::new(),
                        HeaderParamSpec {
                            name: param.name,
                            value: json_to_hcl(&param.value).map_err(|e| {
                                Error::new::<ErrorInvalidInput>(format!(
                                    "failed to parse header value `{}`: {}",
                                    param.value.to_string(),
                                    e
                                ))
                            })?,
                            description: param.desc,
                            options: HeaderParamOptions {
                                disabled: param.options.disabled,
                                propagate: param.options.propagate,
                            },
                        },
                    );
                }

                for param in input.path_params {
                    path_param_map.insert(
                        PathParamId::new(),
                        PathParamSpec {
                            name: param.name,
                            value: json_to_hcl(&param.value).map_err(|e| {
                                Error::new::<ErrorInvalidInput>(format!(
                                    "failed to parse header value `{}`: {}",
                                    param.value.to_string(),
                                    e
                                ))
                            })?,
                            description: param.desc,
                            options: PathParamOptions {
                                disabled: param.options.disabled,
                                propagate: param.options.propagate,
                            },
                        },
                    );
                }

                for param in input.query_params {
                    query_param_map.insert(
                        QueryParamId::new(),
                        QueryParamSpec {
                            name: param.name,
                            value: json_to_hcl(&param.value).map_err(|e| {
                                Error::new::<ErrorInvalidInput>(format!(
                                    "failed to parse header value `{}`: {}",
                                    param.value.to_string(),
                                    e
                                ))
                            })?,
                            description: param.desc,
                            options: QueryParamOptions {
                                disabled: param.options.disabled,
                                propagate: param.options.propagate,
                            },
                        },
                    );
                }

                EntryModel {
                    metadata: Block::new(EntryMetadataSpec {
                        id: id.clone(),
                        class: input.class,
                    }),
                    url: Some(Block::new(UrlDetails {
                        protocol: input.protocol.clone().unwrap_or(EntryProtocol::Get),
                        raw: "Hardcoded Value".to_string(),
                    })),
                    headers: Some(LabeledBlock::new(header_map)),
                    path_params: Some(LabeledBlock::new(path_param_map)),
                    query_params: Some(LabeledBlock::new(query_param_map)),
                }
            }
            EntryClass::Component => {
                EntryModel {
                    metadata: Block::new(EntryMetadataSpec {
                        id: id.clone(),
                        class: input.class,
                    }),
                    url: None,
                    headers: None, // Hardcoded for now
                    path_params: None,
                    query_params: None,
                }
            }
            EntryClass::Schema => {
                EntryModel {
                    metadata: Block::new(EntryMetadataSpec {
                        id: id.clone(),
                        class: input.class,
                    }),
                    url: None,
                    headers: None, // Hardcoded for now
                    path_params: None,
                    query_params: None,
                }
            }
        };

        self.worktree()
            .await
            .create_item_entry(ctx, &input.name, &input.path, model, input.order, false)
            .await?;

        Ok(CreateEntryOutput { id: id })
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
                    //
                    // query_params_to_add: input.query_params_to_add,
                    // query_params_to_update: input.query_params_to_update,
                    // query_params_to_remove: input.query_params_to_remove,
                    //
                    // path_params_to_add: input.path_params_to_add,
                    // path_params_to_update: input.path_params_to_update,
                    // path_params_to_remove: input.path_params_to_remove,

                    // headers_to_add: input.headers_to_add,
                    // headers_to_update: input.headers_to_update,
                    // headers_to_remove: input.headers_to_remove,
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
                    //
                    // query_params_to_add: vec![],
                    // query_params_to_update: vec![],
                    // query_params_to_remove: vec![],

                    // path_params_to_add: vec![],
                    // path_params_to_update: vec![],
                    // path_params_to_remove: vec![],

                    // headers_to_add: vec![],
                    // headers_to_update: vec![],
                    // headers_to_remove: vec![],
                },
            )
            .await?;

        let path = FrontendEntryPath::new(path.to_path_buf());

        Ok(AfterUpdateDirEntryDescription { id: input.id, path })
    }
}

// /// A function for automatically determining the class
// /// based on the path passed from the frontend.
// fn class_from_path(path: &Path) -> EntryClass {
//     match path.iter().next().and_then(|s| s.to_str()) {
//         Some(dirs::REQUESTS_DIR) => EntryClass::Request,
//         Some(dirs::ENDPOINTS_DIR) => EntryClass::Endpoint,
//         Some(dirs::COMPONENTS_DIR) => EntryClass::Component,
//         Some(dirs::SCHEMAS_DIR) => EntryClass::Schema,
//         _ => unreachable!(),
//     }
// }
