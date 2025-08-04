// FIXME: These tests are temporarily commented out because we need to rethink our approach to log management.

// pub mod shared;

// use chrono::{DateTime, FixedOffset};
// use moss_app::{
//     models::{operations::ListLogsInput, primitives::LogLevel},
//     services::log_service::{LogPayload, LogScope},
// };

// use std::{str::FromStr, time::Duration};

// use crate::shared::set_up_test_app;

// /// These tests can work one at a time, but cannot be executed together using `cargo test`.
// /// This is because LoggingService will set a global default subscriber.
// /// However, it can only be set once per a program,
// /// While the `cargo test` model will run every test as part of the same program.
// /// Thus, they are marked as ignored.

// // We can't test dates filter now since we can't generate logs with custom dates

// #[ignore]
// #[tokio::test]
// async fn test_list_logs_empty() {
//     let (app, ctx, cleanup) = set_up_test_app().await;
//     // let _log_service = services.get::<LogService<MockAppRuntime>>();

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
//     let (app, ctx, cleanup) = set_up_test_app().await;
//     let log_service = services.get::<LogService<MockAppRuntime>>();

//     for _ in 0..25 {
//         log_service.warn(
//             LogScope::App,
//             LogPayload {
//                 resource: None,
//                 message: "".to_string(),
//             },
//         );
//         log_service.warn(
//             LogScope::Session,
//             LogPayload {
//                 resource: None,
//                 message: "".to_string(),
//             },
//         );
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
//     let (app, ctx, cleanup) = set_up_test_app().await;
//     // let log_service = services.get::<LogService<MockAppRuntime>>();

//     log_service.debug(
//         LogScope::App,
//         LogPayload {
//             resource: None,
//             message: "".to_string(),
//         },
//     );
//     log_service.warn(
//         LogScope::App,
//         LogPayload {
//             resource: None,
//             message: "".to_string(),
//         },
//     );
//     log_service.error(
//         LogScope::App,
//         LogPayload {
//             resource: None,
//             message: "".to_string(),
//         },
//     );

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
//     let (app, ctx, cleanup) = set_up_test_app().await;
//     // let log_service = services.get::<LogService<MockAppRuntime>>();

//     log_service.debug(
//         LogScope::App,
//         LogPayload {
//             resource: None,
//             message: "".to_string(),
//         },
//     );
//     log_service.debug(
//         LogScope::App,
//         LogPayload {
//             resource: Some("resource".to_string()),
//             message: "".to_string(),
//         },
//     );
//     log_service.debug(
//         LogScope::App,
//         LogPayload {
//             resource: Some("another".to_string()),
//             message: "".to_string(),
//         },
//     );

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
