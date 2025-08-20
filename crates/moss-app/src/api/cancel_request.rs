use moss_applib::AppRuntime;

use crate::{App, models::operations::CancelRequestInput};

impl<R: AppRuntime> App<R> {
    pub async fn cancel_request(&self, input: CancelRequestInput) -> joinerror::Result<()> {
        let request_id = input.request_id;
        let cancellation_map = self.tracked_cancellations.read().await;

        if let Some(canceller) = cancellation_map.get(&request_id) {
            Ok(canceller.cancel())
        } else {
            Err(joinerror::Error::new::<()>(format!(
                "request with id `{request_id}` not found"
            )))
        }
    }
}
