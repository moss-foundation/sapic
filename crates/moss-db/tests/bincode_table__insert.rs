pub mod shared;

use moss_db::{DatabaseClientWithContext, DatabaseError};

use crate::shared::{TEST_NODE_1, TEST_NODE_2, TEST_NODE_3, TestNode, setup_test_bincode_table};

#[tokio::test]
async fn insert_success() {
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

    let expected = vec![
        ("1".to_string(), 1),
        ("2".to_string(), 2),
        ("3".to_string(), 3),
    ];

    {
        let read = client.begin_read_with_context(&ctx).await.unwrap();

        assert_eq!(
            table
                .scan_with_context(&ctx, &read)
                .await
                .unwrap()
                .collect::<Vec<_>>(),
            expected
        )
    }
    // Cleanup
    {
        std::fs::remove_file(path).unwrap();
    }
}

#[tokio::test]
async fn insert_existing_key() {
    let (client, ctx, table, path) = setup_test_bincode_table::<i32>();

    {
        let mut write = client.begin_write_with_context(&ctx).await.unwrap();
        table
            .insert_with_context(&ctx, &mut write, "1".to_string(), &1)
            .await
            .unwrap();
        write.commit().unwrap();
    }

    {
        // Overwrite existing key
        let mut write = client.begin_write_with_context(&ctx).await.unwrap();
        table
            .insert_with_context(&ctx, &mut write, "1".to_string(), &2)
            .await
            .unwrap();
        write.commit().unwrap();
    }

    let expected = vec![("1".to_string(), 2)];
    {
        let read = client.begin_read_with_context(&ctx).await.unwrap();
        assert_eq!(
            table
                .scan_with_context(&ctx, &read)
                .await
                .unwrap()
                .collect::<Vec<_>>(),
            expected
        )
    }

    // Cleanup
    {
        std::fs::remove_file(path).unwrap();
    }
}

#[tokio::test]
async fn insert_in_read_transaction() {
    let (client, ctx, table, path) = setup_test_bincode_table::<i32>();

    {
        let mut read = client.begin_read_with_context(&ctx).await.unwrap();
        let result = table
            .insert_with_context(&ctx, &mut read, "1".to_string(), &1)
            .await;
        assert!(matches!(result, Err(DatabaseError::Transaction(..))));
    }

    // Cleanup
    {
        std::fs::remove_file(path).unwrap();
    }
}

#[tokio::test]
async fn insert_uncommitted() {
    let (client, ctx, table, path) = setup_test_bincode_table::<i32>();

    {
        // Uncommitted transaction
        let mut write = client.begin_write_with_context(&ctx).await.unwrap();
        table
            .insert_with_context(&ctx, &mut write, "1".to_string(), &1)
            .await
            .unwrap();
    }

    {
        let read = client.begin_read_with_context(&ctx).await.unwrap();
        assert!(
            table
                .scan_with_context(&ctx, &read)
                .await
                .unwrap()
                .collect::<Vec<_>>()
                .is_empty()
        );
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[tokio::test]
async fn insert_complex_type() {
    let (client, ctx, table, path) = setup_test_bincode_table::<TestNode>();

    {
        let mut write = client.begin_write_with_context(&ctx).await.unwrap();
        table
            .insert_with_context(&ctx, &mut write, "1".to_string(), &TEST_NODE_1)
            .await
            .unwrap();
        table
            .insert_with_context(&ctx, &mut write, "2".to_string(), &TEST_NODE_2)
            .await
            .unwrap();
        table
            .insert_with_context(&ctx, &mut write, "3".to_string(), &TEST_NODE_3)
            .await
            .unwrap();
        write.commit().unwrap();
    }

    let expected = vec![
        ("1".to_string(), TEST_NODE_1.clone()),
        ("2".to_string(), TEST_NODE_2.clone()),
        ("3".to_string(), TEST_NODE_3.clone()),
    ];

    {
        let read = client.begin_read_with_context(&ctx).await.unwrap();

        assert_eq!(
            table
                .scan_with_context(&ctx, &read)
                .await
                .unwrap()
                .collect::<Vec<_>>(),
            expected
        )
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}
