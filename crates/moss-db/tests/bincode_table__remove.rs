pub mod shared;

use moss_db::{DatabaseClient, common::DatabaseError};

use crate::shared::setup_test_bincode_table;

#[tokio::test]
async fn remove_success() {
    let (client, ctx, table, path) = setup_test_bincode_table::<i32>();

    {
        let mut write = client.begin_write(&ctx).await.unwrap();
        table
            .insert(&ctx, &mut write, "1".to_string(), &1)
            .await
            .unwrap();
        table
            .insert(&ctx, &mut write, "2".to_string(), &2)
            .await
            .unwrap();
        write.commit().unwrap();
    }

    {
        let mut write = client.begin_write(&ctx).await.unwrap();
        let result = table.remove(&ctx, &mut write, "1".to_string()).await;
        assert_eq!(result.ok(), Some(1));
        write.commit().unwrap();
    }

    let expected = vec![("2".to_string(), 2)];
    {
        // Check the entry is removed from the db
        let read = client.begin_read(&ctx).await.unwrap();
        assert_eq!(
            table.scan(&ctx, &read).await.unwrap().collect::<Vec<_>>(),
            expected
        );
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[tokio::test]
async fn remove_nonexistent() {
    let (client, ctx, table, path) = setup_test_bincode_table::<i32>();

    {
        let mut write = client.begin_write(&ctx).await.unwrap();
        let result = table.remove(&ctx, &mut write, "1".to_string()).await;
        assert!(matches!(result, Err(DatabaseError::NotFound { .. })));
        write.commit().unwrap();
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[tokio::test]
async fn remove_in_read_transaction() {
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
        let mut read = client.begin_read(&ctx).await.unwrap();
        let result = table.remove(&ctx, &mut read, "1".to_string()).await;
        assert!(matches!(result, Err(DatabaseError::Transaction(..))))
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[tokio::test]
async fn remove_uncommitted() {
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
        let mut write = client.begin_write(&ctx).await.unwrap();
        let _ = table.remove(&ctx, &mut write, "1".to_string());
    }

    let expected = vec![("1".to_string(), 1)];
    {
        // Check that the change is not committed
        let read = client.begin_read(&ctx).await.unwrap();
        assert_eq!(
            table.scan(&ctx, &read).await.unwrap().collect::<Vec<_>>(),
            expected
        );
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}
