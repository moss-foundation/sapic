pub mod shared;

use moss_db::{DatabaseClientWithContext, DatabaseError};

use crate::shared::{
    TEST_AAD_1, TEST_AAD_2, TEST_PASSWORD_1, TEST_PASSWORD_2, setup_test_encrypted_bincode_table,
};

#[tokio::test]
async fn read_success() {
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
        let read = client.begin_read_with_context(&ctx).await.unwrap();
        let result = table
            .read(&ctx, &read, "1".to_string(), TEST_PASSWORD_1, TEST_AAD_1)
            .await;
        assert_eq!(result.unwrap(), 1);
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[tokio::test]
async fn read_nonexistent() {
    let (client, ctx, table, path) = setup_test_encrypted_bincode_table::<i32>();

    {
        let read = client.begin_read_with_context(&ctx).await.unwrap();
        let result = table
            .read(&ctx, &read, "1".to_string(), TEST_PASSWORD_1, TEST_AAD_1)
            .await;
        assert!(matches!(result, Err(DatabaseError::NotFound { .. })));
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

// AEAD effectively ensures that incorrect keys will result in decryption error
#[tokio::test]
async fn read_with_incorrect_password() {
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
        // Try reading the entry with incorrect password
        let read = client.begin_read_with_context(&ctx).await.unwrap();
        let result = table
            .read(&ctx, &read, "1".to_string(), TEST_PASSWORD_2, TEST_AAD_1)
            .await;
        assert!(matches!(result, Err(DatabaseError::Internal(..))))
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[tokio::test]
async fn read_with_incorrect_aad() {
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
        // Try reading the entry with incorrect aad
        let read = client.begin_read_with_context(&ctx).await.unwrap();
        let result = table
            .read(&ctx, &read, "1".to_string(), TEST_PASSWORD_1, TEST_AAD_2)
            .await;
        assert!(matches!(result, Err(DatabaseError::Internal(..))))
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[tokio::test]
async fn read_in_write_transaction() {
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
        let write = client.begin_write_with_context(&ctx).await.unwrap();
        let result = table
            .read(&ctx, &write, "1".to_string(), TEST_PASSWORD_1, TEST_AAD_1)
            .await;
        assert!(matches!(result, Err(DatabaseError::Transaction(..))))
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}
