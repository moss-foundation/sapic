use std::path::{Path, PathBuf};

use moss_api::ext::ValidationResultExt;
use moss_applib::AppRuntime;
use validator::Validate;

use crate::{
    Collection, dirs,
    models::{
        operations::CreateEntryOutput,
        primitives::{EntryClass, EntryId, FrontendEntryPath},
        types::{
            AfterUpdateDirEntryDescription, AfterUpdateItemEntryDescription, CreateDirEntryParams,
            CreateItemEntryParams, UpdateDirEntryParams, UpdateItemEntryParams,
        },
    },
    spec::EntryModel,
    worktree::ModifyParams,
};

impl<R: AppRuntime> Collection<R> {
    pub(super) async fn create_dir_entry(
        &self,
        ctx: &R::AsyncContext,
        input: CreateDirEntryParams,
    ) -> joinerror::Result<CreateEntryOutput> {
        input.validate().join_err_bare()?;

        let id = EntryId::new();

        // let model = CompositeDirConfigurationModel {
        //     metadata: ConfigurationMetadata { id: id.to_string() },
        //     inner: input.configuration,
        // };

        let model = EntryModel::from((id.clone(), class_from_path(&input.path)));

        self.worktree
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
        // let model = CompositeItemConfigurationModel {
        //     metadata: ConfigurationMetadata { id: id.to_string() },
        //     inner: input.configuration,
        // };

        let model = EntryModel::from((id.clone(), class_from_path(&input.path)));

        self.worktree
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
            .worktree
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
        // let model = CompositeItemConfigurationModel::from(configuration);

        Ok(AfterUpdateItemEntryDescription {
            id: input.id,
            path,
            // configuration: model,
        })
    }

    pub(super) async fn update_dir_entry(
        &self,
        ctx: &R::AsyncContext,
        input: UpdateDirEntryParams,
    ) -> joinerror::Result<AfterUpdateDirEntryDescription> {
        input.validate().join_err_bare()?;

        let path = self
            .worktree
            .update_dir_entry(
                ctx,
                &input.id,
                ModifyParams {
                    name: input.name,
                    order: input.order,
                    expanded: input.expanded,
                    protocol: None,
                    path: input.path,

                    query_params_to_add: vec![],
                    query_params_to_update: vec![],
                    query_params_to_remove: vec![],
                    //
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
        // let model = CompositeDirConfigurationModel::from(configuration);

        Ok(AfterUpdateDirEntryDescription {
            id: input.id,
            path,
            // configuration: model,
        })
    }
}

fn class_from_path(path: &Path) -> EntryClass {
    match path.extension().and_then(|s| s.to_str()) {
        Some(dirs::REQUESTS_DIR) => EntryClass::Request,
        Some(dirs::ENDPOINTS_DIR) => EntryClass::Endpoint,
        Some(dirs::COMPONENTS_DIR) => EntryClass::Component,
        Some(dirs::SCHEMAS_DIR) => EntryClass::Schema,
        _ => unreachable!(),
    }
}
