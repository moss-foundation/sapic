use moss_common::api::OperationResult;
use uuid::Uuid;
use validator::Validate;

use crate::{
    collection::Collection,
    models::{
        operations::{
            CreateDirEntryInput, CreateEntryInput, CreateEntryOutput, CreateItemEntryInput,
        },
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

        let id = Uuid::new_v4();
        let model = CompositeDirConfigurationModel {
            metadata: ConfigurationMetadata { id },
            inner: input.configuration,
        };

        worktree_service
            .create_dir_entry(
                id,
                &input.name,
                input.path,
                model.into(),
                EntryMetadata {
                    order: input.order,
                    expanded: true, // Directories are automatically marked as expanded
                },
            )
            .await?;

        Ok(CreateEntryOutput { id })
    }

    async fn create_item_entry(
        &self,
        input: CreateItemEntryInput,
    ) -> OperationResult<CreateEntryOutput> {
        input.validate()?;

        let worktree_service = self.service::<WorktreeService>();

        let id = Uuid::new_v4();
        let model = CompositeItemConfigurationModel {
            metadata: ConfigurationMetadata { id },
            inner: input.configuration,
        };

        worktree_service
            .create_item_entry(
                id,
                &input.name,
                input.path,
                model.into(),
                EntryMetadata {
                    order: input.order,
                    expanded: false,
                },
            )
            .await?;

        Ok(CreateEntryOutput { id })
    }
}
