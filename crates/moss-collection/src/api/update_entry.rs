use moss_common::api::OperationResult;
use validator::Validate;

use crate::{
    collection::Collection,
    models::{
        operations::{
            UpdateDirEntryInput, UpdateDirEntryOutput, UpdateEntryInput, UpdateEntryOutput,
            UpdateItemEntryInput, UpdateItemEntryOutput,
        },
        primitives::EntryPath,
        types::configuration::{CompositeDirConfigurationModel, CompositeItemConfigurationModel},
    },
    services::worktree_service::{ModifyParams, WorktreeService},
};

impl Collection {
    pub async fn update_entry(
        &mut self,
        input: UpdateEntryInput,
    ) -> OperationResult<UpdateEntryOutput> {
        match input {
            UpdateEntryInput::Item(input) => {
                let output = self.update_item_entry(input).await?;
                Ok(UpdateEntryOutput::Item(output))
            }
            UpdateEntryInput::Dir(input) => {
                let output = self.update_dir_entry(input).await?;
                Ok(UpdateEntryOutput::Dir(output))
            }
        }
    }

    pub(super) async fn update_item_entry(
        &mut self,
        input: UpdateItemEntryInput,
    ) -> OperationResult<UpdateItemEntryOutput> {
        input.validate()?;

        let worktree_service = self.service::<WorktreeService>();

        let (path, configuration) = worktree_service
            .update_item_entry(
                input.id,
                ModifyParams {
                    name: input.name,
                    protocol: input.protocol,
                    expanded: input.expanded,
                    order: input.order,
                },
            )
            .await?;

        let path = EntryPath::new(path.to_path_buf());
        let model = CompositeItemConfigurationModel::from(configuration);

        Ok(UpdateItemEntryOutput {
            id: input.id,
            path,
            configuration: model,
        })
    }

    pub(super) async fn update_dir_entry(
        &mut self,
        input: UpdateDirEntryInput,
    ) -> OperationResult<UpdateDirEntryOutput> {
        input.validate()?;

        let worktree_service = self.service::<WorktreeService>();

        let (path, configuration) = worktree_service
            .update_dir_entry(
                input.id,
                ModifyParams {
                    name: input.name,
                    order: input.order,
                    expanded: input.expanded,
                    protocol: None,
                },
            )
            .await?;

        let path = EntryPath::new(path.to_path_buf());
        let model = CompositeDirConfigurationModel::from(configuration);

        Ok(UpdateDirEntryOutput {
            id: input.id,
            path,
            configuration: model,
        })
    }
}
