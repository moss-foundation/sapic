use moss_common::api::OperationResult;
use uuid::Uuid;
use validator::Validate;

use crate::{
    collection::Collection,
    models::{
        operations::{CreateEntryInput, CreateEntryOutput},
        types::configuration::{
            CompositeDirConfigurationModel, CompositeItemConfigurationModel, ConfigurationMetadata,
            ConfigurationModel,
        },
    },
};

impl Collection {
    pub async fn create_entry(
        &mut self,
        input: CreateEntryInput,
    ) -> OperationResult<CreateEntryOutput> {
        input.validate()?;

        let id = Uuid::new_v4();
        let is_dir = matches!(input, CreateEntryInput::Dir(_));
        let path = input.path().clone();
        let name = input.name().to_owned();
        let configuration = match input {
            CreateEntryInput::Item(item) => {
                ConfigurationModel::Item(CompositeItemConfigurationModel {
                    metadata: ConfigurationMetadata { id },
                    inner: item.configuration,
                })
            }
            CreateEntryInput::Dir(dir) => ConfigurationModel::Dir(CompositeDirConfigurationModel {
                metadata: ConfigurationMetadata { id },
                inner: dir.configuration,
            }),
        };

        // self.worktree()
        //     .create_entry(
        //         &path,
        //         &name,
        //         is_dir,
        //         toml::to_string(&configuration)?.as_bytes(),
        //     )
        //     .await?;

        // TODO: db operations

        Ok(CreateEntryOutput { id })
    }
}
