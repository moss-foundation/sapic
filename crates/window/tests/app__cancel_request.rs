// TODO: restore this in the crate where these operations will be moved.

// #![cfg(feature = "integration-tests")]

// use futures::future;
// use moss_applib::context::{AnyAsyncContext, AnyContext, MutableContext, Reason};
// use std::time::Duration;
// use tokio::time::timeout;
// use window::models::operations::CancelRequestInput;

// use crate::shared::set_up_test_app;
// mod shared;

// #[tokio::test]
// async fn cancel_request_success() {
//     let (app, _, ctx, cleanup) = set_up_test_app().await;

//     let ctx = MutableContext::new_with_timeout(ctx, Duration::from_secs(30));
//     let request_id = "request".to_string();

//     let cancellation_map = app.cancellation_map();

//     cancellation_map
//         .write()
//         .await
//         .insert(request_id.clone(), ctx.get_canceller());

//     let ctx = ctx.freeze();
//     let (canceled_tx, canceled_rx) = tokio::sync::oneshot::channel();

//     {
//         let ctx = ctx.clone();
//         tokio::spawn(async move {
//             loop {
//                 if matches!(ctx.done(), Some(Reason::Canceled)) {
//                     eprintln!("Background task cancelled");
//                     canceled_tx.send(true).unwrap();
//                     return;
//                 }
//                 eprintln!("Background task running");
//                 tokio::time::sleep(Duration::from_millis(100)).await;
//             }
//         });
//     }
//     tokio::time::sleep(Duration::from_millis(1000)).await;

//     app.cancel_request(CancelRequestInput { request_id })
//         .await
//         .unwrap();

//     if let Err(_) = timeout(Duration::from_secs(5), canceled_rx).await {
//         panic!("Cancellation failed");
//     } else {
//         println!("Cancellation succeeded");
//     }

//     cleanup().await;
// }

// #[tokio::test]
// async fn cancel_request_nonexistent() {
//     let (app, _, _, cleanup) = set_up_test_app().await;

//     let request_id = "nonexistent".to_string();

//     assert!(
//         app.cancel_request(CancelRequestInput { request_id })
//             .await
//             .is_err()
//     );

//     cleanup().await;
// }

// #[tokio::test]
// async fn cancel_request_with_child() {
//     let (app, _, ctx, cleanup) = set_up_test_app().await;

//     let cancellation_map = app.cancellation_map();

//     let parent_ctx = MutableContext::new_with_timeout(ctx, Duration::from_secs(30));
//     let parent_id = "parent".to_string();

//     cancellation_map
//         .write()
//         .await
//         .insert(parent_id.clone(), parent_ctx.get_canceller());

//     let (parent_canceled_tx, parent_canceled_rx) = tokio::sync::oneshot::channel();
//     let (child_canceled_tx, child_canceled_rx) = tokio::sync::oneshot::channel();
//     {
//         let cancellation_map = cancellation_map.clone();
//         let parent_ctx = parent_ctx.freeze();
//         tokio::spawn(async move {
//             let child_ctx = AnyAsyncContext::new(parent_ctx.clone());
//             let child_id = "child".to_string();
//             cancellation_map
//                 .write()
//                 .await
//                 .insert(child_id.clone(), child_ctx.get_canceller());
//             {
//                 let child_ctx = child_ctx.freeze();

//                 // Child Context
//                 tokio::spawn(async move {
//                     loop {
//                         if matches!(child_ctx.done(), Some(Reason::Canceled)) {
//                             eprintln!("Child task cancelled");
//                             child_canceled_tx.send(true).unwrap();
//                             return;
//                         }
//                         eprintln!("Child task running");
//                         tokio::time::sleep(Duration::from_millis(100)).await;
//                     }
//                 });
//             }

//             // Parent Context
//             loop {
//                 if matches!(parent_ctx.done(), Some(Reason::Canceled)) {
//                     eprintln!("Parent task cancelled");
//                     parent_canceled_tx.send(true).unwrap();
//                     return;
//                 }
//                 eprintln!("Parent task running");
//                 tokio::time::sleep(Duration::from_millis(100)).await;
//             }
//         });
//     }

//     tokio::time::sleep(Duration::from_millis(1000)).await;
//     app.cancel_request(CancelRequestInput {
//         request_id: parent_id,
//     })
//     .await
//     .unwrap();

//     // Cancelling the parent context should also cancel the child context
//     let canceled_fut = future::join(parent_canceled_rx, child_canceled_rx);

//     if let Err(_) = timeout(Duration::from_secs(5), canceled_fut).await {
//         panic!("Cancellation failed");
//     } else {
//         println!("Cancellation succeeded");
//     }

//     cleanup().await;
// }
