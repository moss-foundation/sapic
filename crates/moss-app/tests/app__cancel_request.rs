use crate::shared::set_up_test_app;

mod shared;

#[tokio::test]
async fn cancel_request_success() {
    let (app, ctx, services, cleanup, abs_path) = set_up_test_app().await;
}
