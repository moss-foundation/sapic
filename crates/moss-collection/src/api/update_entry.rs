use moss_common::{NanoId, api::OperationResult};
use validator::Validate;

use crate::{
    collection::Collection,
    models::{
        operations::{UpdateEntryInput, UpdateEntryOutput},
        primitives::EntryPath,
        types::{
            AfterUpdateDirEntryDescription, AfterUpdateItemEntryDescription, UpdateDirEntryParams,
            UpdateItemEntryParams,
            configuration::{CompositeDirConfigurationModel, CompositeItemConfigurationModel},
        },
    },
    services::worktree_service::{ModifyParams, WorktreeService},
};

impl Collection {
    pub async fn update_entry(
        &self,
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
        &self,
        input: UpdateItemEntryParams,
    ) -> OperationResult<AfterUpdateItemEntryDescription> {
        input.validate()?;
        let id: NanoId = input.id.clone().into();
        let worktree_service = self.service::<WorktreeService>();

        let (path, configuration) = worktree_service
            .update_item_entry(
                &id,
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

        Ok(AfterUpdateItemEntryDescription {
            id: input.id,
            path,
            configuration: model,
        })
    }

    pub(super) async fn update_dir_entry(
        &self,
        input: UpdateDirEntryParams,
    ) -> OperationResult<AfterUpdateDirEntryDescription> {
        input.validate()?;
        let id: NanoId = input.id.clone().into();
        let worktree_service = self.service::<WorktreeService>();

        let (path, configuration) = worktree_service
            .update_dir_entry(
                &id,
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

        Ok(AfterUpdateDirEntryDescription {
            id: input.id,
            path,
            configuration: model,
        })
    }
}
