// TODO: restore this in the crate where these operations will be moved.

// #![cfg(feature = "integration-tests")]

// pub mod shared;

// use chrono::{DateTime, FixedOffset};
// use window::models::{operations::ListLogsInput, primitives::LogLevel};

// use crate::shared::set_up_test_app;
// use moss_logging::{app, session};
// use std::{str::FromStr, time::Duration};

// /// These tests can work one at a time, but cannot be executed together using `cargo test`.
// /// This is because LoggingService will set a global default subscriber.
// /// However, it can only be set once per a program,
// /// While the `cargo test` model will run every test as part of the same program.
// /// Thus, they are marked as ignored.

// // We can't test dates filter now since we can't generate logs with custom dates

// #[ignore]
// #[tokio::test]
// async fn test_list_logs_empty() {
//     let (app, _, ctx, cleanup) = set_up_test_app().await;

//     let list_logs_result = app
//         .list_logs(
//             &ctx,
//             &ListLogsInput {
//                 dates: vec![],
//                 levels: vec![],
//                 resource: None,
//             },
//         )
//         .await;

//     assert!(list_logs_result.is_ok());
//     let logs = list_logs_result.unwrap().contents;
//     assert!(logs.is_empty());

//     cleanup().await;
// }

// #[ignore]
// #[tokio::test]
// async fn test_list_logs_from_both_files_and_queue() {
//     // By default, the applong and session log queue will be flushed to files for every ten log
//     // We will create 25 of each to see that the logs are successfully combined
//     let (app, _, ctx, cleanup) = set_up_test_app().await;

//     for _ in 0..25 {
//         app::warn!("");
//         session::warn!("");
//     }

//     // Wait for all writes to finish
//     tokio::time::sleep(Duration::from_millis(500)).await;

//     let list_logs_output = app
//         .list_logs(
//             &ctx,
//             &ListLogsInput {
//                 dates: vec![],
//                 levels: vec![],
//                 resource: None,
//             },
//         )
//         .await
//         .unwrap();

//     let logs = list_logs_output.contents;
//     assert_eq!(logs.len(), 50);
//     // Check that the output logs are sorted chronologically
//     assert!(
//         logs.is_sorted_by_key(|log| DateTime::<FixedOffset>::from_str(&log.timestamp).unwrap())
//     );

//     cleanup().await;
// }

// #[ignore]
// #[tokio::test]
// async fn test_list_logs_by_level() {
//     let (app, _, ctx, cleanup) = set_up_test_app().await;

//     app::debug!("");
//     app::warn!("");
//     app::error!("");

//     let debug_logs = app
//         .list_logs(
//             &ctx,
//             &ListLogsInput {
//                 dates: vec![],
//                 levels: vec![LogLevel::DEBUG],
//                 resource: None,
//             },
//         )
//         .await
//         .unwrap()
//         .contents;
//     assert_eq!(debug_logs.len(), 1);

//     let warn_logs = app
//         .list_logs(
//             &ctx,
//             &ListLogsInput {
//                 dates: vec![],
//                 levels: vec![LogLevel::WARN],
//                 resource: None,
//             },
//         )
//         .await
//         .unwrap()
//         .contents;
//     assert_eq!(warn_logs.len(), 1);

//     let error_logs = app
//         .list_logs(
//             &ctx,
//             &ListLogsInput {
//                 dates: vec![],
//                 levels: vec![LogLevel::ERROR],
//                 resource: None,
//             },
//         )
//         .await
//         .unwrap()
//         .contents;
//     assert_eq!(error_logs.len(), 1);

//     cleanup().await;
// }

// #[ignore]
// #[tokio::test]
// async fn test_list_logs_by_resource() {
//     let (app, _, ctx, cleanup) = set_up_test_app().await;
//     // let log_service = services.get::<LogService<MockAppRuntime>>();

//     app::debug!("");
//     app::debug!("resource", "");
//     app::debug!("another", "");

//     let resource_logs = app
//         .list_logs(
//             &ctx,
//             &ListLogsInput {
//                 dates: vec![],
//                 levels: vec![],
//                 resource: Some("resource".to_string()),
//             },
//         )
//         .await
//         .unwrap()
//         .contents;

//     assert_eq!(resource_logs.len(), 1);

//     cleanup().await;
// }
