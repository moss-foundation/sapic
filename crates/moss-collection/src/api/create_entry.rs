use moss_common::api::OperationResult;
use uuid::Uuid;
use validator::Validate;

use crate::{
    collection::Collection,
    configuration::{
        ConfigurationModel, DirConfigurationModel, ItemConfigurationModel, SpecificationMetadata,
    },
    models::operations::{CreateEntryInput, CreateEntryOutput},
};

impl Collection {
    pub async fn create_entry(
        &mut self,
        input: CreateEntryInput,
    ) -> OperationResult<CreateEntryOutput> {
        input.validate()?;

        let id = Uuid::new_v4();
        let config = if input.is_dir {
            ConfigurationModel::Dir(DirConfigurationModel {
                metadata: SpecificationMetadata { id },
            })
        } else {
            ConfigurationModel::Item(ItemConfigurationModel {
                metadata: SpecificationMetadata { id },
            })
        };

        let mut worktree = self.worktree().await?.write().await;
        let changes = worktree.create_entry(&input.destination, config).await?;

        Ok(CreateEntryOutput { changes })
    }
}
