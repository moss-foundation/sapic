pub mod shared;

use moss_db::{DatabaseClient, common::DatabaseError};

use crate::shared::setup_test_bincode_table;

#[tokio::test]
async fn scan_by_prefix_empty() {
    let (client, ctx, table, path) = setup_test_bincode_table::<i32>();

    {
        let read = client.begin_read(&ctx).await.unwrap();
        let result = table.scan_by_prefix(&ctx, &read, "test_").await.unwrap();
        assert_eq!(result.len(), 0);
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[tokio::test]
async fn scan_by_prefix_string_keys() {
    let (client, ctx, table, path) = setup_test_bincode_table::<i32>();

    // Insert test data with different prefixes
    {
        let mut write = client.begin_write(&ctx).await.unwrap();

        // Keys with prefix "user_"
        for i in 0..50 {
            table
                .insert(&ctx, &mut write, format!("user_{}", i), &i)
                .await
                .unwrap();
        }

        // Keys with prefix "order_"
        for i in 50..100 {
            table
                .insert(&ctx, &mut write, format!("order_{}", i), &i)
                .await
                .unwrap();
        }

        // Keys with no specific prefix
        for i in 100..150 {
            table
                .insert(&ctx, &mut write, i.to_string(), &i)
                .await
                .unwrap();
        }

        write.commit().unwrap();
    }

    // Query using prefix "user_" - should return only user entries
    {
        let read = client.begin_read(&ctx).await.unwrap();
        let result = table.scan_by_prefix(&ctx, &read, "user_").await.unwrap();

        assert_eq!(result.len(), 50);

        // All keys should start with "user_"
        for (key, _) in &result {
            assert!(key.starts_with("user_"));
        }

        // Verify all values are correct
        let mut values: Vec<_> = result.iter().map(|(_, v)| *v).collect();
        values.sort();
        assert_eq!(values, (0..50).collect::<Vec<_>>());
    }

    // Query using prefix "order_" - should return only order entries
    {
        let read = client.begin_read(&ctx).await.unwrap();
        let result = table.scan_by_prefix(&ctx, &read, "order_").await.unwrap();

        assert_eq!(result.len(), 50);

        // All keys should start with "order_"
        for (key, _) in &result {
            assert!(key.starts_with("order_"));
        }

        // Verify all values are correct
        let mut values: Vec<_> = result.iter().map(|(_, v)| *v).collect();
        values.sort();
        assert_eq!(values, (50..100).collect::<Vec<_>>());
    }

    // Query using empty prefix - should return all entries
    {
        let read = client.begin_read(&ctx).await.unwrap();
        let result = table.scan_by_prefix(&ctx, &read, "").await.unwrap();

        assert_eq!(result.len(), 150);
    }

    // Query using non-existent prefix
    {
        let read = client.begin_read(&ctx).await.unwrap();
        let result = table
            .scan_by_prefix(&ctx, &read, "nonexistent_")
            .await
            .unwrap();

        assert_eq!(result.len(), 0);
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

// Test for case sensitivity in prefix matching
#[tokio::test]
async fn scan_by_prefix_case_sensitivity() {
    let (client, ctx, table, path) = setup_test_bincode_table::<i32>();

    {
        let mut write = client.begin_write(&ctx).await.unwrap();

        // Insert keys with mixed case
        table
            .insert(&ctx, &mut write, "User_1".to_string(), &1)
            .await
            .unwrap();
        table
            .insert(&ctx, &mut write, "user_2".to_string(), &2)
            .await
            .unwrap();
        table
            .insert(&ctx, &mut write, "USER_3".to_string(), &3)
            .await
            .unwrap();

        write.commit().unwrap();
    }

    // Case-sensitive search should find exact matches only
    {
        let read = client.begin_read(&ctx).await.unwrap();

        // Capital "User_" should match only "User_1"
        let result = table.scan_by_prefix(&ctx, &read, "User_").await.unwrap();
        assert_eq!(result.len(), 1);

        // Lowercase "user_" should match only "user_2"
        let result = table.scan_by_prefix(&ctx, &read, "user_").await.unwrap();
        assert_eq!(result.len(), 1);

        // All uppercase "USER_" should match only "USER_3"
        let result = table.scan_by_prefix(&ctx, &read, "USER_").await.unwrap();
        assert_eq!(result.len(), 1);
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[tokio::test]
async fn scan_by_prefix_in_write_transaction() {
    let (client, ctx, table, path) = setup_test_bincode_table::<i32>();

    {
        let write = client.begin_write(&ctx).await.unwrap();
        let result = table.scan_by_prefix(&ctx, &write, "test_").await;
        assert!(matches!(result, Err(DatabaseError::Transaction(_))));
    }

    std::fs::remove_file(path).unwrap();
}
