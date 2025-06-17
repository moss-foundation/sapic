use moss_app::{
    models::{
        operations::{BatchDeleteLogInput, ListLogsInput},
        types::LogEntryRef,
    },
    services::log_service::{LogPayload, LogScope},
};
use std::fs::remove_dir_all;

use crate::shared::set_up_log_service;
mod shared;

/// These tests can work one at a time, but cannot be executed together using `cargo test`.
/// This is because LoggingService will set a global default subscriber.
/// However, it can only be set once per a program,
/// While the `cargo test` model will run every test as part of the same program.
/// Thus, they are marked as ignored.

#[tokio::test]
async fn test_delete_logs_from_queue() {
    let (log_service, applog_path) = set_up_log_service().await;

    // We only have one log, less than the dump threshold
    // So we should delete from the queue
    log_service.warn(
        LogScope::App,
        LogPayload {
            resource: None,
            message: "".to_string(),
        },
    );

    let logs = log_service
        .list_logs(&ListLogsInput {
            dates: vec![],
            levels: vec![],
            resource: None,
        })
        .await
        .unwrap()
        .contents;

    let input = BatchDeleteLogInput(
        logs.into_iter()
            .map(|log| LogEntryRef {
                timestamp: log.timestamp,
                id: log.id,
            })
            .collect(),
    );

    let output = log_service.batch_delete_log(&input).await.unwrap();
    let deleted_entries = output.deleted_entries;
    assert_eq!(deleted_entries.len(), 1);
    assert_eq!(deleted_entries[0].id, input.0[0].id);
    assert!(deleted_entries[0].file_path.is_none());

    let new_logs = log_service
        .list_logs(&ListLogsInput {
            dates: vec![],
            levels: vec![],
            resource: None,
        })
        .await
        .unwrap()
        .contents;
    assert!(new_logs.is_empty());
    remove_dir_all(applog_path).unwrap();
}

#[tokio::test]
async fn test_delete_logs_from_file() {
    let (log_service, applog_path) = set_up_log_service().await;

    // By default, the dump threshold is 10, which means that the first log
    for _ in 0..15 {
        log_service.warn(
            LogScope::App,
            LogPayload {
                resource: None,
                message: "".to_string(),
            },
        );
    }

    let logs = log_service
        .list_logs(&ListLogsInput {
            dates: vec![],
            levels: vec![],
            resource: None,
        })
        .await
        .unwrap()
        .contents;

    let input = BatchDeleteLogInput(vec![LogEntryRef {
        timestamp: logs[0].timestamp.clone(),
        id: logs[0].id.clone(),
    }]);

    let output = log_service.batch_delete_log(&input).await.unwrap();
    let deleted_entries = output.deleted_entries;
    assert_eq!(deleted_entries.len(), 1);
    assert_eq!(deleted_entries[0].id, input.0[0].id);
    // It should be deleted from a file
    assert!(deleted_entries[0].file_path.is_some());

    let new_logs = log_service
        .list_logs(&ListLogsInput {
            dates: vec![],
            levels: vec![],
            resource: None,
        })
        .await
        .unwrap()
        .contents;
    assert_eq!(new_logs.len(), 14);
    remove_dir_all(applog_path).unwrap();
}

#[tokio::test]
async fn test_delete_all_logs() {
    let (log_service, applog_path) = set_up_log_service().await;

    for _ in 0..15 {
        log_service.warn(
            LogScope::App,
            LogPayload {
                resource: None,
                message: "".to_string(),
            },
        );
    }

    let logs = log_service
        .list_logs(&ListLogsInput {
            dates: vec![],
            levels: vec![],
            resource: None,
        })
        .await
        .unwrap()
        .contents;

    let input = BatchDeleteLogInput(
        logs.into_iter()
            .map(|log| LogEntryRef {
                id: log.id,
                timestamp: log.timestamp,
            })
            .collect(),
    );
    let output = log_service.batch_delete_log(&input).await.unwrap();
    let deleted_entries = output.deleted_entries;
    assert_eq!(deleted_entries.len(), 15);

    let new_logs = log_service
        .list_logs(&ListLogsInput {
            dates: vec![],
            levels: vec![],
            resource: None,
        })
        .await
        .unwrap()
        .contents;
    assert!(new_logs.is_empty());

    remove_dir_all(applog_path).unwrap();
}
