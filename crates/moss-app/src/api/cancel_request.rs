use moss_applib::AppRuntime;
use moss_common::api::{OperationError, OperationResult};

use crate::{App, models::operations::CancelRequestInput};

impl<R: AppRuntime> App<R> {
    pub async fn cancel_request(&self, input: CancelRequestInput) -> OperationResult<()> {
        let request_id = input.request_id;
        let cancellation_map = self.tracked_cancellations.read().await;

        if let Some(canceller) = cancellation_map.get(&request_id) {
            Ok(canceller.cancel())
        } else {
            Err(OperationError::NotFound(format!(
                "request with id `{request_id}` not found"
            )))
        }
    }
}
