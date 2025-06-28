use moss_bindingutils::primitives::ChangeUsize;
use moss_common::api::OperationResult;
use moss_db::primitives::AnyValue;
use moss_storage::storage::operations::{
    PutItem, RemoveItem, TransactionalPutItem, TransactionalRemoveItem,
};
use std::{path::Path, sync::Arc};
use validator::Validate;

use crate::{
    collection::Collection,
    models::{
        operations::{
            UpdateDirEntryInput, UpdateDirEntryOutput, UpdateEntryInput, UpdateEntryOutput,
            UpdateItemEntryInput, UpdateItemEntryOutput,
        },
        primitives::EntryPath,
        types::configuration::{
            CompositeDirConfigurationModel, CompositeItemConfigurationModel,
            docschema::RawDirConfiguration,
        },
    },
    storage::segments,
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

        let mut path: Arc<Path> = input.path.clone().into();
        if let Some(name) = input.name {
            path = worktree.rename_entry(input.id, &name).await?;
        }

        let configuration = worktree
            .with_entry_item_mut(input.id, |entry| {
                let mut is_config_updated = false;

                if let Some(protocol) = input.protocol {
                    entry.set_protocol(protocol)?;
                    is_config_updated = true;
                }

                if let Some(expanded) = input.expanded {
                    entry.set_expanded(expanded);
                }

                if is_config_updated {
                    Ok(Some(entry.configuration().clone()))
                } else {
                    Ok(None)
                }
            })
            .await?;

        if let Some(configuration) = &configuration {
            worktree
                .update_entry(
                    input.path,
                    false,
                    hcl::to_string(&configuration)?.as_bytes(),
                )
                .await?;
        }

        let path = EntryPath::new(path.to_path_buf());
        let model = configuration.map(CompositeItemConfigurationModel::from);
        let is_db_update_needed = input.order.is_some() || input.expanded.is_some();
        if !is_db_update_needed {
            return Ok(UpdateEntryOutput::Item(UpdateItemEntryOutput {
                id: input.id,
                path,
                configuration: model,
            }));
        }

        let mut txn = self.storage().begin_write()?;
        let store = self.storage().resource_store();

        if let Some(order) = input.order {
            let segkey = segments::segkey_entry_order(&input.id.to_string());
            TransactionalPutItem::put(store.as_ref(), &mut txn, segkey, AnyValue::from(order))?;
        }

        // TODO: update expanded entries

        txn.commit()?;

        Ok(UpdateEntryOutput::Item(UpdateItemEntryOutput {
            id: input.id,
            path,
            configuration: model,
        }))
    }

    pub(super) async fn update_dir_entry(
        &mut self,
        input: UpdateDirEntryInput,
    ) -> OperationResult<UpdateEntryOutput> {
        input.validate()?;

        let worktree = self.worktree();

        let mut path = input.path.clone().into();
        if let Some(name) = input.name {
            path = worktree.rename_entry(input.id, &name).await?;
        }

        let configuration = worktree
            .with_entry_dir_mut(input.id, |entry| {
                if let Some(expanded) = input.expanded {
                    entry.set_expanded(expanded);
                }

                Ok::<Option<RawDirConfiguration>, _>(None)
            })
            .await?;

        if let Some(configuration) = &configuration {
            worktree
                .update_entry(
                    input.path,
                    false,
                    hcl::to_string(&configuration)?.as_bytes(),
                )
                .await?;
        }

        let path = EntryPath::new(path.to_path_buf());
        let model = configuration.map(CompositeDirConfigurationModel::from);
        let is_db_update_needed = input.order.is_some() || input.expanded.is_some();
        if !is_db_update_needed {
            return Ok(UpdateEntryOutput::Dir(UpdateDirEntryOutput {
                id: input.id,
                path,
                configuration: model,
            }));
        }

        let mut txn = self.storage().begin_write()?;
        let store = self.storage().resource_store();

        if let Some(order) = input.order {
            let segkey = segments::segkey_entry_order(&input.id.to_string());
            TransactionalPutItem::put(store.as_ref(), &mut txn, segkey, AnyValue::from(order))?;
        }

        // TODO: update expanded entries

        txn.commit()?;

        Ok(UpdateEntryOutput::Dir(UpdateDirEntryOutput {
            id: input.id,
            path,
            configuration: None,
        }))
    }
}

// fn update_order_if_needed(
//     input: Option<ChangeUsize>,
//     txn: &mut Transaction,
//     store: &ResourceStore,
// ) -> OperationResult<()> {
//     // match input {
//     //     Some(ChangeUsize::Update(order)) => {
//     //         let segkey = segments::segkey_entry_order(&input.id.to_string());
//     //         TransactionalPutItem::put(store.as_ref(), &mut txn, segkey, AnyValue::from(order))?;
//     //     }
//     //     Some(ChangeUsize::Remove) => {
//     //         let segkey = segments::segkey_entry_order(&input.id.to_string());
//     //         TransactionalRemoveItem::remove(store.as_ref(), &mut txn, segkey)?;
//     //     }
//     //     _ => {}
//     // };

//     Ok(())
// }
