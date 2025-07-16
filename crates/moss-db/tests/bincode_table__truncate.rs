pub mod shared;

use moss_db::{DatabaseClientWithContext, DatabaseError};

use crate::shared::setup_test_bincode_table;

#[tokio::test]
async fn truncate_empty() {
    let (client, ctx, table, path) = setup_test_bincode_table::<i32>();

    {
        let mut write = client.begin_write_with_context(&ctx).await.unwrap();
        let result = table.truncate_with_context(&ctx, &mut write).await;
        let _ = result.unwrap();
        write.commit().unwrap();
    }

    {
        let read = client.begin_read_with_context(&ctx).await.unwrap();
        assert_eq!(
            table.scan_with_context(&ctx, &read).await.unwrap().count(),
            0
        );
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[tokio::test]
async fn truncate_non_empty() {
    let (client, ctx, table, path) = setup_test_bincode_table::<i32>();

    {
        let mut write = client.begin_write_with_context(&ctx).await.unwrap();
        table
            .insert_with_context(&ctx, &mut write, "1".to_string(), &1)
            .await
            .unwrap();
        table
            .insert_with_context(&ctx, &mut write, "2".to_string(), &2)
            .await
            .unwrap();
        table
            .insert_with_context(&ctx, &mut write, "3".to_string(), &3)
            .await
            .unwrap();
    }

    {
        let mut write = client.begin_write_with_context(&ctx).await.unwrap();
        let result = table.truncate_with_context(&ctx, &mut write).await;
        let _ = result.unwrap();
    }

    {
        let read = client.begin_read_with_context(&ctx).await.unwrap();
        assert_eq!(
            table.scan_with_context(&ctx, &read).await.unwrap().count(),
            0
        );
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[tokio::test]
async fn truncate_in_read_transaction() {
    let (client, ctx, table, path) = setup_test_bincode_table::<i32>();

    {
        let mut read = client.begin_read_with_context(&ctx).await.unwrap();
        let result = table.truncate_with_context(&ctx, &mut read).await;
        assert!(matches!(result, Err(DatabaseError::Transaction(..))));
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[tokio::test]
async fn truncate_uncommitted() {
    let (client, ctx, table, path) = setup_test_bincode_table::<i32>();

    {
        let mut write = client.begin_write_with_context(&ctx).await.unwrap();
        table
            .insert_with_context(&ctx, &mut write, "1".to_string(), &1)
            .await
            .unwrap();
        table
            .insert_with_context(&ctx, &mut write, "2".to_string(), &2)
            .await
            .unwrap();
        table
            .insert_with_context(&ctx, &mut write, "3".to_string(), &3)
            .await
            .unwrap();
        write.commit().unwrap();
    }

    {
        // Uncommitted
        let mut write = client.begin_write_with_context(&ctx).await.unwrap();
        table.truncate_with_context(&ctx, &mut write).await.unwrap();
    }

    {
        let read = client.begin_read_with_context(&ctx).await.unwrap();
        assert_eq!(
            table.scan_with_context(&ctx, &read).await.unwrap().count(),
            3
        );
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}
