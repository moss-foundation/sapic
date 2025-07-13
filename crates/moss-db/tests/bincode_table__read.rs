pub mod shared;

use moss_db::{DatabaseClient, common::DatabaseError};

use crate::shared::setup_test_bincode_table;

#[tokio::test]
async fn read_existent() {
    let (client, ctx, table, path) = setup_test_bincode_table::<i32>();

    {
        // Setup
        let mut write = client.begin_write(&ctx).await.unwrap();
        table
            .insert(&ctx, &mut write, "1".to_string(), &1)
            .await
            .unwrap();
        write.commit().unwrap();
    }

    {
        let read = client.begin_read(&ctx).await.unwrap();
        let result = table.read(&ctx, &read, "1".to_string()).await;
        assert_eq!(result.ok(), Some(1));
    }

    {
        std::fs::remove_file(&path).unwrap();
    }
}

#[tokio::test]
async fn read_non_existent() {
    let (client, ctx, table, path) = setup_test_bincode_table::<i32>();

    {
        let read = client.begin_read(&ctx).await.unwrap();
        let result = table.read(&ctx, &read, "1".to_string()).await;
        assert!(matches!(result, Err(DatabaseError::NotFound { .. })));
    }

    {
        std::fs::remove_file(&path).unwrap();
    }
}

#[tokio::test]
async fn read_in_write_transaction() {
    let (client, ctx, table, path) = setup_test_bincode_table::<i32>();

    {
        let mut write = client.begin_write(&ctx).await.unwrap();
        table
            .insert(&ctx, &mut write, "1".to_string(), &1)
            .await
            .unwrap();
        write.commit().unwrap();
    }

    {
        let write = client.begin_write(&ctx).await.unwrap();
        let result = table.read(&ctx, &write, "1".to_string()).await;
        assert!(matches!(result, Err(DatabaseError::Transaction(..))))
    }

    {
        std::fs::remove_file(&path).unwrap();
    }
}
