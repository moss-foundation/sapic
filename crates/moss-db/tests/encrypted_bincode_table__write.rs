pub mod shared;

use moss_db::{DatabaseClientWithContext, DatabaseError};

use crate::shared::{
    TEST_AAD_1, TEST_AAD_2, TEST_PASSWORD_1, TEST_PASSWORD_2, setup_test_encrypted_bincode_table,
};

#[tokio::test]
async fn write_success() {
    let (client, ctx, table, path) = setup_test_encrypted_bincode_table::<i32>();

    {
        let mut write = client.begin_write_with_context(&ctx).await.unwrap();
        let result = table
            .write(
                &ctx,
                &mut write,
                "1".to_string(),
                &1,
                TEST_PASSWORD_1,
                TEST_AAD_1,
            )
            .await;
        let _ = result.unwrap();
        write.commit().unwrap();
    }

    {
        let read = client.begin_read_with_context(&ctx).await.unwrap();
        let result = table
            .read(&ctx, &read, "1".to_string(), TEST_PASSWORD_1, TEST_AAD_1)
            .await
            .unwrap();
        assert_eq!(result, 1);
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[tokio::test]
async fn write_overwrite() {
    let (client, ctx, table, path) = setup_test_encrypted_bincode_table::<i32>();

    {
        let mut write = client.begin_write_with_context(&ctx).await.unwrap();
        table
            .write(
                &ctx,
                &mut write,
                "1".to_string(),
                &1,
                TEST_PASSWORD_1,
                TEST_AAD_1,
            )
            .await
            .unwrap();
        write.commit().unwrap();
    }

    {
        // Overwrite existing key
        let mut write = client.begin_write_with_context(&ctx).await.unwrap();
        let result = table
            .write(
                &ctx,
                &mut write,
                "1".to_string(),
                &2,
                TEST_PASSWORD_1,
                TEST_AAD_1,
            )
            .await;
        let _ = result.unwrap();
        write.commit().unwrap();
    }

    {
        // Check the key is overwritten
        let read = client.begin_read_with_context(&ctx).await.unwrap();
        let result = table
            .read(&ctx, &read, "1".to_string(), TEST_PASSWORD_1, TEST_AAD_1)
            .await
            .unwrap();
        assert_eq!(result, 2);
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[tokio::test]
async fn write_multiple_entries_with_different_password() {
    let (client, ctx, table, path) = setup_test_encrypted_bincode_table::<i32>();

    {
        let mut write = client.begin_write_with_context(&ctx).await.unwrap();
        table
            .write(
                &ctx,
                &mut write,
                "1".to_string(),
                &1,
                TEST_PASSWORD_1,
                TEST_AAD_1,
            )
            .await
            .unwrap();
        table
            .write(
                &ctx,
                &mut write,
                "2".to_string(),
                &2,
                TEST_PASSWORD_2,
                TEST_AAD_2,
            )
            .await
            .unwrap();
        write.commit().unwrap();
    }

    {
        // Check both entries are inserted with correct password
        let read = client.begin_read_with_context(&ctx).await.unwrap();
        let result_1 = table
            .read(&ctx, &read, "1".to_string(), TEST_PASSWORD_1, TEST_AAD_1)
            .await
            .unwrap();
        assert_eq!(result_1, 1);
        let result_2 = table
            .read(&ctx, &read, "2".to_string(), TEST_PASSWORD_2, TEST_AAD_2)
            .await
            .unwrap();
        assert_eq!(result_2, 2);
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[tokio::test]
async fn write_in_read_transaction() {
    let (client, ctx, table, path) = setup_test_encrypted_bincode_table::<i32>();

    {
        let mut read = client.begin_read_with_context(&ctx).await.unwrap();
        let result = table
            .write(
                &ctx,
                &mut read,
                "1".to_string(),
                &1,
                TEST_PASSWORD_1,
                TEST_AAD_1,
            )
            .await;
        assert!(matches!(result, Err(DatabaseError::Transaction(..))))
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[tokio::test]
async fn write_uncommitted() {
    let (client, ctx, table, path) = setup_test_encrypted_bincode_table::<i32>();

    {
        // Uncommitted write
        let mut write = client.begin_write_with_context(&ctx).await.unwrap();
        table
            .write(
                &ctx,
                &mut write,
                "1".to_string(),
                &1,
                TEST_PASSWORD_1,
                TEST_AAD_1,
            )
            .await
            .unwrap();
    }

    {
        let read = client.begin_read_with_context(&ctx).await.unwrap();
        let result = table
            .read(&ctx, &read, "1".to_string(), TEST_PASSWORD_1, TEST_AAD_1)
            .await;
        assert!(matches!(result, Err(DatabaseError::NotFound { .. })))
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}
