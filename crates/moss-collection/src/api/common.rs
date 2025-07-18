use moss_applib::AppRuntime;
use moss_common::api::OperationResult;
use validator::Validate;

use crate::{
    Collection,
    models::{
        operations::{CreateDirEntryInput, CreateEntryOutput, CreateItemEntryInput},
        primitives::{EntryId, EntryPath},
        types::{
            AfterUpdateDirEntryDescription, AfterUpdateItemEntryDescription, UpdateDirEntryParams,
            UpdateItemEntryParams,
            configuration::{
                CompositeDirConfigurationModel, CompositeItemConfigurationModel,
                ConfigurationMetadata,
            },
        },
    },
    services::{
        DynWorktreeService,
        worktree_service::{EntryMetadata, ModifyParams},
    },
};

impl<R: AppRuntime> Collection<R> {
    pub(super) async fn create_dir_entry(
        &self,
        ctx: &R::AsyncContext,
        input: CreateDirEntryInput,
    ) -> OperationResult<CreateEntryOutput> {
        input.validate()?;

        let worktree_service = self.service::<DynWorktreeService<R>>();

        let id = EntryId::new();
        let model = CompositeDirConfigurationModel {
            metadata: ConfigurationMetadata { id: id.to_string() },
            inner: input.configuration,
        };

        worktree_service
            .create_dir_entry(
                ctx,
                &id,
                &input.name,
                &input.path,
                model.into(),
                EntryMetadata {
                    order: input.order,
                    expanded: true, // Directories are automatically marked as expanded
                },
            )
            .await?;

        Ok(CreateEntryOutput { id: id })
    }

    pub(super) async fn create_item_entry(
        &self,
        ctx: &R::AsyncContext,
        input: CreateItemEntryInput,
    ) -> OperationResult<CreateEntryOutput> {
        input.validate()?;

        let worktree_service = self.service::<DynWorktreeService<R>>();

        let id = EntryId::new();
        let model = CompositeItemConfigurationModel {
            metadata: ConfigurationMetadata { id: id.to_string() },
            inner: input.configuration,
        };

        worktree_service
            .create_item_entry(
                ctx,
                &id,
                &input.name,
                &input.path,
                model.into(),
                EntryMetadata {
                    order: input.order,
                    expanded: false,
                },
            )
            .await?;

        Ok(CreateEntryOutput { id: id })
    }

    pub(super) async fn update_item_entry(
        &self,
        ctx: &R::AsyncContext,
        input: UpdateItemEntryParams,
    ) -> OperationResult<AfterUpdateItemEntryDescription> {
        input.validate()?;
        let worktree_service = self.service::<DynWorktreeService<R>>();

        let (path, configuration) = worktree_service
            .update_item_entry(
                ctx,
                &input.id,
                ModifyParams {
                    name: input.name,
                    protocol: input.protocol,
                    expanded: input.expanded,
                    order: input.order,
                    path: input.path,
                },
            )
            .await?;

        let path = EntryPath::new(path.to_path_buf());
        let model = CompositeItemConfigurationModel::from(configuration);

        Ok(AfterUpdateItemEntryDescription {
            id: input.id,
            path,
            configuration: model,
        })
    }

    pub(super) async fn update_dir_entry(
        &self,
        ctx: &R::AsyncContext,
        input: UpdateDirEntryParams,
    ) -> OperationResult<AfterUpdateDirEntryDescription> {
        input.validate()?;
        let worktree_service = self.service::<DynWorktreeService<R>>();

        let (path, configuration) = worktree_service
            .update_dir_entry(
                ctx,
                &input.id,
                ModifyParams {
                    name: input.name,
                    order: input.order,
                    expanded: input.expanded,
                    protocol: None,
                    path: input.path,
                },
            )
            .await?;

        let path = EntryPath::new(path.to_path_buf());
        let model = CompositeDirConfigurationModel::from(configuration);

        Ok(AfterUpdateDirEntryDescription {
            id: input.id,
            path,
            configuration: model,
        })
    }
}
