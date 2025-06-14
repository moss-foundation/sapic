use moss_common::api::OperationError;
use moss_logging::{
    LogPayload, LogScope,
    models::operations::{DeleteLogInput, ListLogsInput},
};
use std::fs::remove_dir_all;

use crate::shared::set_up_logging_service;

mod shared;

/// These tests can work one at a time, but cannot be executed together using `cargo test`.
/// This is because LoggingService will set a global default subscriber.
/// However, it can only be set once per a program,
/// While the `cargo test` model will run every test as part of the same program.
/// Thus, they are marked as ignored.

#[ignore]
#[tokio::test]
async fn test_delete_log_from_queue() {
    let (logging_service, applog_path) = set_up_logging_service().await;

    // We only have one log, which is less than the threshold required for dumping
    logging_service.debug(
        LogScope::App,
        LogPayload {
            resource: None,
            message: "".to_string(),
        },
    );

    let log_entries = logging_service
        .list_logs(&ListLogsInput {
            dates: vec![],
            levels: vec![],
            resource: None,
        })
        .await
        .unwrap();

    let id = log_entries.contents[0].id.clone();
    let timestamp = log_entries.contents[0].timestamp.clone();

    let delete_log_result = logging_service
        .delete_log(&DeleteLogInput { timestamp, id })
        .await;

    assert!(delete_log_result.is_ok());
    let current_logs = logging_service
        .list_logs(&ListLogsInput {
            dates: vec![],
            levels: vec![],
            resource: None,
        })
        .await
        .unwrap()
        .contents;

    assert!(current_logs.is_empty());

    remove_dir_all(applog_path).unwrap();
}

#[ignore]
#[tokio::test]
async fn test_delete_log_from_file() {
    let (logging_service, applog_path) = set_up_logging_service().await;

    for i in 0..20 {
        logging_service.debug(
            LogScope::App,
            LogPayload {
                resource: Some(i.to_string()),
                message: "".to_string(),
            },
        )
    }

    // Wait for all write tasks to finish
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // We find the oldest log entry, which should already be dumped to a file
    let log_entries = logging_service
        .list_logs(&ListLogsInput {
            dates: vec![],
            levels: vec![],
            resource: Some("0".to_string()),
        })
        .await
        .unwrap()
        .contents;

    let id = log_entries[0].id.clone();
    let timestamp = log_entries[0].timestamp.clone();

    let delete_log_result = logging_service
        .delete_log(&DeleteLogInput {
            timestamp,
            id: id.clone(),
        })
        .await;

    assert!(delete_log_result.is_ok());
    let current_logs = logging_service
        .list_logs(&ListLogsInput {
            dates: vec![],
            levels: vec![],
            resource: None,
        })
        .await
        .unwrap()
        .contents;

    // Check that only the target entry is deleted
    assert_eq!(current_logs.len(), 19);
    assert!(!current_logs.iter().any(|entry| { entry.id == id }));

    remove_dir_all(applog_path).unwrap();
}

#[ignore]
#[tokio::test]
async fn test_delete_log_nonexistent() {
    let (logging_service, applog_path) = set_up_logging_service().await;

    logging_service.debug(
        LogScope::App,
        LogPayload {
            resource: None,
            message: "".to_string(),
        },
    );

    let log_entries = logging_service
        .list_logs(&ListLogsInput {
            dates: vec![],
            levels: vec![],
            resource: None,
        })
        .await
        .unwrap()
        .contents;

    let id = log_entries[0].id.clone();
    let timestamp = log_entries[0].timestamp.clone();
    logging_service
        .delete_log(&DeleteLogInput {
            timestamp: timestamp.clone(),
            id: id.clone(),
        })
        .await
        .unwrap();

    // Try deleting an already removed entry
    let delete_log_result = logging_service
        .delete_log(&DeleteLogInput {
            timestamp: timestamp.clone(),
            id: id.clone(),
        })
        .await;

    assert!(matches!(
        delete_log_result,
        Err(OperationError::NotFound(..))
    ));

    remove_dir_all(applog_path).unwrap();
}
