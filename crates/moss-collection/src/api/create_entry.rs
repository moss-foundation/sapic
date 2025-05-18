use std::path::PathBuf;

use moss_common::api::OperationResult;
use serde_json::Value as JsonValue;

use crate::{
    collection::Collection, models::primitives::ChangesDiffSet,
    worktree::virtual_snapshot::Classification,
};

pub struct CreateEntryInput {
    pub destination: PathBuf,
    pub classification: Classification,
    pub specification: Option<JsonValue>,
    pub protocol: Option<String>,
    pub order: Option<usize>,
    pub is_dir: bool,
}

pub struct CreateEntryOutput {
    pub physical_changes: ChangesDiffSet,
    pub virtual_changes: ChangesDiffSet,
}

impl Collection {
    pub async fn create_entry(
        &self,
        input: CreateEntryInput,
    ) -> OperationResult<CreateEntryOutput> {
        // TODO: validate input

        // TODO: validate specification

        let content = if let Some(value) = input.specification {
            Some(serde_json::to_vec(&value)?)
        } else {
            None
        };

        let worktree = self.worktree().await?;
        let mut worktree_lock = worktree.write().await;

        let changes = worktree_lock
            .create_entry(
                input.destination,
                input.order,
                input.protocol,
                content,
                input.classification,
                true,
            )
            .await?;

        Ok(CreateEntryOutput {
            physical_changes: changes.physical_changes,
            virtual_changes: changes.virtual_changes,
        })
    }
}
