use moss_common::api::OperationResult;
use validator::Validate;

use crate::{
    collection::Collection,
    models::{
        operations::{
            UpdateDirEntryInput, UpdateEntryInput, UpdateEntryOutput, UpdateItemEntryInput,
        },
        types::configuration::ItemConfigurationModel,
    },
};

impl Collection {
    pub async fn update_entry(
        &mut self,
        input: UpdateEntryInput,
    ) -> OperationResult<UpdateEntryOutput> {
        match input {
            UpdateEntryInput::Item(input) => self.update_item_entry(input).await,
            UpdateEntryInput::Dir(input) => self.update_dir_entry(input).await,
        }
    }

    pub(super) async fn update_item_entry(
        &mut self,
        input: UpdateItemEntryInput,
    ) -> OperationResult<UpdateEntryOutput> {
        input.validate()?;

        let worktree = self.worktree();

        if let Some(name) = input.name {
            worktree.rename_entry(input.id, &name).await?;
        }

        let configuration = worktree
            .with_entry_item_mut(input.id, |entry| {
                if let Some(protocol) = input.protocol {
                    entry.set_protocol(protocol)?;
                }

                Ok(entry.configuration().clone())
            })
            .await?;

        // let model = CompositeItemConfigurationModel::from(entry.configuration().clone());

        // let model: ItemConfigurationModel = worktree
        //     .with_entry_mut(input.id, |entry| {
        //         if let Some(protocol) = input.protocol {
        //             entry.set_protocol(protocol)?;
        //         }

        //         let configuration = entry
        //             .configuration()
        //             .as_item()
        //             .expect("Entry expected to be an item");

        //         todo!()

        //         // Ok(configuration.into())
        //     })
        //     .await?;

        unimplemented!()
    }

    pub(super) async fn update_dir_entry(
        &mut self,
        _input: UpdateDirEntryInput,
    ) -> OperationResult<UpdateEntryOutput> {
        unimplemented!()
    }
}
