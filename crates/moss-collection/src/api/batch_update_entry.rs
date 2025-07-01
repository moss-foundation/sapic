use moss_common::api::OperationResult;

use crate::{
    collection::Collection,
    models::operations::{
        BatchUpdateEntryInput, BatchUpdateEntryInputKind, BatchUpdateEntryOutput, UpdateEntryOutput,
    },
};

impl Collection {
    pub async fn batch_update_entry(
        &mut self,
        input: BatchUpdateEntryInput,
    ) -> OperationResult<BatchUpdateEntryOutput> {
        let mut results = Vec::new();
        for entry in input.entries {
            match entry {
                BatchUpdateEntryInputKind::Item(input) => {
                    let output = self.update_item_entry(input).await?;
                    results.push(UpdateEntryOutput::Item(output));
                }
                BatchUpdateEntryInputKind::Dir(input) => {
                    let output = self.update_dir_entry(input).await?;
                    results.push(UpdateEntryOutput::Dir(output));
                }
            }
        }

        Ok(BatchUpdateEntryOutput { entries: results })
    }
}
