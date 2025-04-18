mod shared;

use moss_db::common::DatabaseError;
use moss_db::DatabaseClient;

use crate::shared::{
    setup_test_encrypted_bincode_table, TEST_AAD_1, TEST_AAD_2, TEST_PASSWORD_1, TEST_PASSWORD_2,
};

#[test]
fn read_success() {
    let (client, table, path) = setup_test_encrypted_bincode_table::<i32>();

    {
        let mut write = client.begin_write().unwrap();
        table
            .write(&mut write, "1".to_string(), &1, TEST_PASSWORD_1, TEST_AAD_1)
            .unwrap();
        write.commit().unwrap();
    }

    {
        let read = client.begin_read().unwrap();
        let result = table.read(&read, "1".to_string(), TEST_PASSWORD_1, TEST_AAD_1);
        // assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn read_nonexistent() {
    let (client, table, path) = setup_test_encrypted_bincode_table::<i32>();

    {
        let read = client.begin_read().unwrap();
        let result = table.read(&read, "1".to_string(), TEST_PASSWORD_1, TEST_AAD_1);
        assert!(matches!(result, Err(DatabaseError::NotFound { .. })));
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

// AEAD effectively ensures that incorrect keys will result in decryption error
#[test]
fn read_with_incorrect_password() {
    let (client, table, path) = setup_test_encrypted_bincode_table::<i32>();

    {
        let mut write = client.begin_write().unwrap();
        table
            .write(&mut write, "1".to_string(), &1, TEST_PASSWORD_1, TEST_AAD_1)
            .unwrap();
        write.commit().unwrap();
    }

    {
        // Try reading the entry with incorrect password
        let read = client.begin_read().unwrap();
        let result = table.read(&read, "1".to_string(), TEST_PASSWORD_2, TEST_AAD_1);
        assert!(matches!(result, Err(DatabaseError::Internal(..))))
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn read_with_incorrect_aad() {
    let (client, table, path) = setup_test_encrypted_bincode_table::<i32>();

    {
        let mut write = client.begin_write().unwrap();
        table
            .write(&mut write, "1".to_string(), &1, TEST_PASSWORD_1, TEST_AAD_1)
            .unwrap();
        write.commit().unwrap();
    }

    {
        // Try reading the entry with incorrect aad
        let read = client.begin_read().unwrap();
        let result = table.read(&read, "1".to_string(), TEST_PASSWORD_1, TEST_AAD_2);
        assert!(matches!(result, Err(DatabaseError::Internal(..))))
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn read_in_write_transaction() {
    let (client, table, path) = setup_test_encrypted_bincode_table::<i32>();

    {
        let mut write = client.begin_write().unwrap();
        table
            .write(&mut write, "1".to_string(), &1, TEST_PASSWORD_1, TEST_AAD_1)
            .unwrap();
        write.commit().unwrap();
    }

    {
        let mut write = client.begin_write().unwrap();
        let result = table.read(&write, "1".to_string(), TEST_PASSWORD_1, TEST_AAD_1);
        assert!(matches!(result, Err(DatabaseError::Transaction(..))))
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}
