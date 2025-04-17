use crate::shared::{
    setup_test_bincode_table, setup_test_encrypted_bincode_table, TEST_AAD_1, TEST_AAD_2,
    TEST_PASSWORD_1, TEST_PASSWORD_2,
};
use moss_db::common::DatabaseError;
use moss_db::DatabaseClient;

mod shared;

#[test]
fn write_success() {
    let (client, table, path) = setup_test_encrypted_bincode_table();

    {
        let mut write = client.begin_write().unwrap();
        let result = table.write(&mut write, "1".to_string(), &1, TEST_PASSWORD_1, TEST_AAD_1);
        assert!(result.is_ok());
        write.commit().unwrap();
    }

    {
        let read = client.begin_read().unwrap();
        let result = table
            .read(&read, "1".to_string(), TEST_PASSWORD_1, TEST_AAD_1)
            .unwrap();
        assert_eq!(result, 1);
        read.commit().unwrap();
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn write_overwrite() {
    let (client, table, path) = setup_test_encrypted_bincode_table();

    {
        let mut write = client.begin_write().unwrap();
        table
            .write(&mut write, "1".to_string(), &1, TEST_PASSWORD_1, TEST_AAD_1)
            .unwrap();
        write.commit().unwrap();
    }

    {
        // Overwrite existing key
        let mut write = client.begin_write().unwrap();
        let result = table.write(&mut write, "1".to_string(), &2, TEST_PASSWORD_1, TEST_AAD_1);
        assert!(result.is_ok());
        write.commit().unwrap();
    }

    {
        // Check the key is overwritten
        let read = client.begin_read().unwrap();
        let result = table
            .read(&read, "1".to_string(), TEST_PASSWORD_1, TEST_AAD_1)
            .unwrap();
        assert_eq!(result, 2);
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn write_multiple_entries_with_different_password() {
    let (client, table, path) = setup_test_encrypted_bincode_table();

    {
        let mut write = client.begin_write().unwrap();
        table
            .write(&mut write, "1".to_string(), &1, TEST_PASSWORD_1, TEST_AAD_1)
            .unwrap();
        table
            .write(&mut write, "2".to_string(), &2, TEST_PASSWORD_2, TEST_AAD_2)
            .unwrap();
        write.commit().unwrap();
    }

    {
        // Check both entries are inserted with correct password
        let read = client.begin_read().unwrap();
        let result_1 = table
            .read(&read, "1".to_string(), TEST_PASSWORD_1, TEST_AAD_1)
            .unwrap();
        assert_eq!(result_1, 1);
        let result_2 = table
            .read(&read, "2".to_string(), TEST_PASSWORD_2, TEST_AAD_2)
            .unwrap();
        assert_eq!(result_2, 2);
        read.commit().unwrap();
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn write_in_read_transaction() {
    let (client, table, path) = setup_test_encrypted_bincode_table();

    {
        let mut read = client.begin_read().unwrap();
        let result = table.write(&mut read, "1".to_string(), &1, TEST_PASSWORD_1, TEST_AAD_1);
        assert!(matches!(result, Err(DatabaseError::Transaction(..))))
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}
