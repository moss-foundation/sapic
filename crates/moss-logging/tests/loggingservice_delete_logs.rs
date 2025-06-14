use crate::shared::set_up_logging_service;
use moss_logging::{
    LogPayload, LogScope,
    models::operations::{DeleteLogInput, DeleteLogsInput, ListLogsInput},
};
use std::fs::remove_dir_all;

mod shared;

/// These tests can work one at a time, but cannot be executed together using `cargo test`.
/// This is because LoggingService will set a global default subscriber.
/// However, it can only be set once per a program,
/// While the `cargo test` model will run every test as part of the same program.
/// Thus, they are marked as ignored.

#[ignore]
#[tokio::test]
async fn test_delete_logs_all() {
    let (logging_service, applog_path) = set_up_logging_service().await;
    for _ in 0..20 {
        logging_service.debug(
            LogScope::App,
            LogPayload {
                resource: None,
                message: "".to_string(),
            },
        );
    }

    // Wait for all writes to finish
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    let logs = logging_service
        .list_logs(&ListLogsInput {
            dates: vec![],
            levels: vec![],
            resource: None,
        })
        .await
        .unwrap()
        .contents;

    let delete_logs_input = DeleteLogsInput {
        entries: logs
            .into_iter()
            .map(|entry| DeleteLogInput {
                timestamp: entry.timestamp,
                id: entry.id,
            })
            .collect(),
    };

    let delete_logs_result = logging_service.delete_logs(&delete_logs_input).await;
    assert!(delete_logs_result.is_ok());
    assert_eq!(delete_logs_result.unwrap().deleted_entries.len(), 20);

    let new_logs = logging_service
        .list_logs(&ListLogsInput {
            dates: vec![],
            levels: vec![],
            resource: None,
        })
        .await
        .unwrap()
        .contents;

    assert_eq!(new_logs.len(), 0);

    remove_dir_all(applog_path).unwrap();
}
