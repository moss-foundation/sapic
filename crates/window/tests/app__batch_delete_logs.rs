pub mod shared;

use moss_logging::app;
use window::models::operations::{BatchDeleteLogInput, ListLogsInput};

use crate::shared::set_up_test_app;

/// These tests can work one at a time, but cannot be executed together using `cargo test`.
/// This is because LoggingService will set a global default subscriber.
/// However, it can only be set once per a program,
/// While the `cargo test` model will run every test as part of the same program.
/// Thus, they are marked as ignored.

#[ignore]
#[tokio::test]
async fn test_delete_logs_from_queue() {
    let (app, _, ctx, cleanup) = set_up_test_app().await;

    // We only have one log, less than the dump threshold
    // So we should delete from the queue
    app::warn!("");

    let logs = app
        .list_logs(
            &ctx,
            &ListLogsInput {
                dates: vec![],
                levels: vec![],
                resource: None,
            },
        )
        .await
        .unwrap()
        .contents;

    let input = BatchDeleteLogInput {
        ids: logs.into_iter().map(|log| log.id).collect(),
    };

    let output = app.batch_delete_log(&ctx, &input).await.unwrap();
    let deleted_entries = output.deleted_entries;
    assert_eq!(deleted_entries.len(), 1);
    assert_eq!(deleted_entries[0].id, input.ids[0]);
    assert!(deleted_entries[0].file_path.is_none());

    let new_logs = app
        .list_logs(
            &ctx,
            &ListLogsInput {
                dates: vec![],
                levels: vec![],
                resource: None,
            },
        )
        .await
        .unwrap()
        .contents;
    assert!(new_logs.is_empty());
    cleanup().await;
}

#[ignore]
#[tokio::test]
async fn test_delete_logs_from_file() {
    let (app, _, ctx, cleanup) = set_up_test_app().await;

    // By default, the dump threshold is 10, which means that the first log
    for _ in 0..15 {
        app::warn!("");
    }

    let logs = app
        .list_logs(
            &ctx,
            &ListLogsInput {
                dates: vec![],
                levels: vec![],
                resource: None,
            },
        )
        .await
        .unwrap()
        .contents;

    let input = BatchDeleteLogInput {
        ids: vec![logs[0].id.clone()],
    };

    let output = app.batch_delete_log(&ctx, &input).await.unwrap();
    let deleted_entries = output.deleted_entries;
    assert_eq!(deleted_entries.len(), 1);
    assert_eq!(deleted_entries[0].id, input.ids[0]);
    // It should be deleted from a file
    assert!(deleted_entries[0].file_path.is_some());

    let new_logs = app
        .list_logs(
            &ctx,
            &ListLogsInput {
                dates: vec![],
                levels: vec![],
                resource: None,
            },
        )
        .await
        .unwrap()
        .contents;
    assert_eq!(new_logs.len(), 14);
    cleanup().await;
}

#[tokio::test]
async fn test_delete_all_logs() {
    let (app, _, ctx, cleanup) = set_up_test_app().await;

    for _ in 0..15 {
        app::warn!("");
    }

    // Wait for all writes to finish
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    let logs = app
        .list_logs(
            &ctx,
            &ListLogsInput {
                dates: vec![],
                levels: vec![],
                resource: None,
            },
        )
        .await
        .unwrap()
        .contents;

    let input = BatchDeleteLogInput {
        ids: logs.into_iter().map(|log| log.id.clone()).collect(),
    };

    let output = app.batch_delete_log(&ctx, &input).await.unwrap();
    let deleted_entries = output.deleted_entries;
    assert_eq!(deleted_entries.len(), 15);

    let new_logs = app
        .list_logs(
            &ctx,
            &ListLogsInput {
                dates: vec![],
                levels: vec![],
                resource: None,
            },
        )
        .await
        .unwrap()
        .contents;
    assert!(new_logs.is_empty());

    cleanup().await;
}
