pub mod shared;

use moss_db::{DatabaseClientWithContext, common::DatabaseError};

use crate::shared::setup_test_bincode_table;

#[tokio::test]
async fn scan_empty() {
    let (client, ctx, table, path) = setup_test_bincode_table::<i32>();

    {
        let read = client.begin_read_with_context(&ctx).await.unwrap();
        assert_eq!(
            table
                .scan_with_context(&ctx, &read)
                .await
                .unwrap()
                .into_iter()
                .count(),
            0
        );
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[tokio::test]
async fn scan_multiple() {
    let (client, ctx, table, path) = setup_test_bincode_table::<i32>();

    {
        let mut write = client.begin_write_with_context(&ctx).await.unwrap();
        for i in 0..100 {
            table
                .insert_with_context(&ctx, &mut write, i.to_string(), &i)
                .await
                .unwrap();
        }
        write.commit().unwrap();
    }

    let expected = (0..100).map(|i| (i.to_string(), i)).collect::<Vec<_>>();
    {
        let read = client.begin_read_with_context(&ctx).await.unwrap();
        let mut scan_result = table
            .scan_with_context(&ctx, &read)
            .await
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>();
        scan_result.sort_by_key(|(_, i)| *i);
        assert_eq!(scan_result, expected);
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[tokio::test]
async fn scan_in_write_transaction() {
    let (client, ctx, table, path) = setup_test_bincode_table::<i32>();

    {
        let write = client.begin_write_with_context(&ctx).await.unwrap();
        let result = table.scan_with_context(&ctx, &write).await;
        assert!(matches!(result, Err(DatabaseError::Transaction(_))));
    }

    std::fs::remove_file(path).unwrap();
}
