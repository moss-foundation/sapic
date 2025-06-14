use moss_common::api::OperationResult;

use crate::{
    collection::Collection,
    models::operations::{UpdateEntryInput, UpdateEntryOutput},
};

impl Collection {
    pub async fn update_entry(
        &mut self,
        _input: UpdateEntryInput,
    ) -> OperationResult<UpdateEntryOutput> {
        unimplemented!()
    }
}
