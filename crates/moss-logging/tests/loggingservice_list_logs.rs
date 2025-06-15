use crate::shared::set_up_logging_service;
use moss_logging::{
    LogPayload, LogScope,
    models::{operations::ListLogsInput, types::LogLevel},
};
use std::fs::remove_dir_all;

mod shared;

/// These tests can work one at a time, but cannot be executed together using `cargo test`.
/// This is because LoggingService will set a global default subscriber.
/// However, it can only be set once per a program,
/// While the `cargo test` model will run every test as part of the same program.
/// Thus, they are marked as ignored.

// We can't test dates filter now since we can't generate logs with custom dates
#[ignore]
#[tokio::test]
async fn test_list_logs_empty() {
    let (logging_service, applog_path) = set_up_logging_service().await;

    let list_logs_result = logging_service
        .list_logs(&ListLogsInput {
            dates: vec![],
            levels: vec![],
            resource: None,
        })
        .await;

    assert!(list_logs_result.is_ok());
    let logs = list_logs_result.unwrap().contents;
    assert!(logs.is_empty());

    remove_dir_all(applog_path).unwrap();
}

#[ignore]
#[tokio::test]
async fn test_list_logs_no_filter() {
    let (logging_service, applog_path) = set_up_logging_service().await;

    // Check that both app logs and session logs are returned
    logging_service.debug(
        LogScope::App,
        LogPayload {
            resource: None,
            message: "".to_string(),
        },
    );
    logging_service.debug(
        LogScope::Session,
        LogPayload {
            resource: None,
            message: "".to_string(),
        },
    );
    logging_service.warn(
        LogScope::App,
        LogPayload {
            resource: None,
            message: "".to_string(),
        },
    );
    logging_service.warn(
        LogScope::Session,
        LogPayload {
            resource: None,
            message: "".to_string(),
        },
    );

    let list_logs_result = logging_service
        .list_logs(&ListLogsInput {
            dates: vec![],
            levels: vec![],
            resource: None,
        })
        .await;
    assert!(list_logs_result.is_ok());
    let logs = list_logs_result.unwrap().contents;

    assert_eq!(logs.len(), 4);
    remove_dir_all(applog_path).unwrap();
}

#[ignore]
#[tokio::test]
async fn test_list_logs_filter_by_levels() {
    let (logging_service, applog_path) = set_up_logging_service().await;
    logging_service.debug(
        LogScope::App,
        LogPayload {
            resource: None,
            message: "".to_string(),
        },
    );
    logging_service.warn(
        LogScope::App,
        LogPayload {
            resource: None,
            message: "".to_string(),
        },
    );
    logging_service.error(
        LogScope::App,
        LogPayload {
            resource: None,
            message: "".to_string(),
        },
    );

    let debug_logs = logging_service
        .list_logs(&ListLogsInput {
            dates: vec![],
            levels: vec![LogLevel::DEBUG],
            resource: None,
        })
        .await
        .unwrap()
        .contents;
    assert_eq!(debug_logs.len(), 1);

    let warn_logs = logging_service
        .list_logs(&ListLogsInput {
            dates: vec![],
            levels: vec![LogLevel::WARN],
            resource: None,
        })
        .await
        .unwrap()
        .contents;
    assert_eq!(warn_logs.len(), 1);

    let error_logs = logging_service
        .list_logs(&ListLogsInput {
            dates: vec![],
            levels: vec![LogLevel::ERROR],
            resource: None,
        })
        .await
        .unwrap()
        .contents;
    assert_eq!(error_logs.len(), 1);

    remove_dir_all(applog_path).unwrap();
}

#[ignore]
#[tokio::test]
async fn test_list_logs_filter_by_resource() {
    let (logging_service, applog_path) = set_up_logging_service().await;
    logging_service.debug(
        LogScope::App,
        LogPayload {
            resource: None,
            message: "".to_string(),
        },
    );
    logging_service.debug(
        LogScope::App,
        LogPayload {
            resource: Some("A".to_string()),
            message: "".to_string(),
        },
    );
    logging_service.debug(
        LogScope::App,
        LogPayload {
            resource: Some("B".to_string()),
            message: "".to_string(),
        },
    );

    let a_logs = logging_service
        .list_logs(&ListLogsInput {
            dates: vec![],
            levels: vec![],
            resource: Some("A".to_string()),
        })
        .await
        .unwrap()
        .contents;
    assert_eq!(a_logs.len(), 1);

    let b_logs = logging_service
        .list_logs(&ListLogsInput {
            dates: vec![],
            levels: vec![],
            resource: Some("B".to_string()),
        })
        .await
        .unwrap()
        .contents;
    assert_eq!(b_logs.len(), 1);

    remove_dir_all(applog_path).unwrap();
}
