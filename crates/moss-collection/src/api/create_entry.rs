use moss_common::api::OperationResult;
use validator::Validate;

use crate::{
    collection::Collection,
    models::{
        operations::{
            CreateDirEntryInput, CreateEntryInput, CreateEntryOutput, CreateItemEntryInput,
        },
        primitives::EntryId,
        types::configuration::{
            CompositeDirConfigurationModel, CompositeItemConfigurationModel, ConfigurationMetadata,
        },
    },
    services::worktree_service::{EntryMetadata, WorktreeService},
};

impl Collection {
    pub async fn create_entry(
        &self,
        input: CreateEntryInput,
    ) -> OperationResult<CreateEntryOutput> {
        match input {
            CreateEntryInput::Item(input) => self.create_item_entry(input).await,
            CreateEntryInput::Dir(input) => self.create_dir_entry(input).await,
        }
    }

    async fn create_dir_entry(
        &self,
        input: CreateDirEntryInput,
    ) -> OperationResult<CreateEntryOutput> {
        input.validate()?;

        let worktree_service = self.service::<WorktreeService>();

        let id = EntryId::new();
        let model = CompositeDirConfigurationModel {
            metadata: ConfigurationMetadata { id: id.to_string() },
            inner: input.configuration,
        };

        worktree_service
            .create_dir_entry(
                &id,
                &input.name,
                input.path,
                model.into(),
                EntryMetadata {
                    order: input.order,
                    expanded: true, // Directories are automatically marked as expanded
                },
            )
            .await?;

        Ok(CreateEntryOutput { id: id })
    }

    async fn create_item_entry(
        &self,
        input: CreateItemEntryInput,
    ) -> OperationResult<CreateEntryOutput> {
        input.validate()?;

        let worktree_service = self.service::<WorktreeService>();

        let id = EntryId::new();
        let model = CompositeItemConfigurationModel {
            metadata: ConfigurationMetadata { id: id.to_string() },
            inner: input.configuration,
        };

        worktree_service
            .create_item_entry(
                &id,
                &input.name,
                input.path,
                model.into(),
                EntryMetadata {
                    order: input.order,
                    expanded: false,
                },
            )
            .await?;

        Ok(CreateEntryOutput { id: id })
    }
}
