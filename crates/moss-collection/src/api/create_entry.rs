use moss_common::api::OperationResult;
use validator::Validate;

use crate::{
    collection::Collection,
    constants::ID_LENGTH,
    models::{
        operations::{CreateEntryInput, CreateEntryOutput},
        types::configuration::{
            CompositeDirConfigurationModel, CompositeItemConfigurationModel, ConfigurationMetadata,
            docschema::{RawDirConfiguration, RawItemConfiguration},
        },
    },
};

impl Collection {
    pub async fn create_entry(
        &mut self,
        input: CreateEntryInput,
    ) -> OperationResult<CreateEntryOutput> {
        input.validate()?;

        let id = nanoid::nanoid!(ID_LENGTH);
        let is_dir = matches!(input, CreateEntryInput::Dir(_));
        let path = input.path().clone();
        let name = input.name().to_owned();
        let content = match input {
            CreateEntryInput::Item(item) => {
                let model = CompositeItemConfigurationModel {
                    metadata: ConfigurationMetadata { id: id.clone() },
                    inner: item.configuration,
                };

                let configuration: RawItemConfiguration = model.into();
                hcl::ser::to_string(&configuration)?
            }
            CreateEntryInput::Dir(dir) => {
                let model = CompositeDirConfigurationModel {
                    metadata: ConfigurationMetadata { id: id.clone() },
                    inner: dir.configuration,
                };

                let configuration: RawDirConfiguration = model.into();
                hcl::ser::to_string(&configuration)?
            }
        };

        self.worktree()
            .create_entry(&path, &name, is_dir, content.as_bytes())
            .await?;

        // TODO: db operations

        Ok(CreateEntryOutput { id })
    }
}
