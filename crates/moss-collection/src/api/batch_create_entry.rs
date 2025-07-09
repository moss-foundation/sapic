use moss_common::api::OperationResult;

use crate::{
    Collection,
    models::operations::{
        BatchCreateEntryInput, BatchCreateEntryKind, BatchCreateEntryOutput, CreateEntryInput,
    },
};

impl Collection {
    pub async fn batch_create_entry(
        &self,
        input: BatchCreateEntryInput,
    ) -> OperationResult<BatchCreateEntryOutput> {
        // Split directories from items and create directories first
        let mut items = Vec::new();
        let mut dirs = Vec::new();
        let mut ids = Vec::new();
        for entry in input.entries {
            match entry {
                BatchCreateEntryKind::Item(item) => {
                    items.push(item);
                }
                BatchCreateEntryKind::Dir(dir) => {
                    dirs.push(dir);
                }
            }
        }

        // Sort dir creation input by depth and create from shallow to deep
        dirs.sort_by(|a, b| {
            let depth_a = a.path.components().count();
            let depth_b = b.path.components().count();
            match depth_a.cmp(&depth_b) {
                std::cmp::Ordering::Equal => a.path.cmp(&b.path),
                other => other,
            }
        });

        for dir in dirs {
            let output = self.create_entry(CreateEntryInput::Dir(dir)).await?;
            ids.push(output.id);
        }

        for item in items {
            let output = self.create_entry(CreateEntryInput::Item(item)).await?;
            ids.push(output.id);
        }

        Ok(BatchCreateEntryOutput { ids })
    }
}
