pub mod shared;

use moss_db::{DatabaseClient, common::DatabaseError};

use crate::shared::setup_test_bincode_table;

#[test]
fn read_existent() {
    let (client, table, path) = setup_test_bincode_table::<i32>();

    {
        // Setup
        let mut write = client.begin_write().unwrap();
        table.insert(&mut write, "1".to_string(), &1).unwrap();
        write.commit().unwrap();
    }

    {
        let read = client.begin_read().unwrap();
        let result = table.read(&read, "1".to_string());
        assert_eq!(result.ok(), Some(1));
    }

    {
        std::fs::remove_file(&path).unwrap();
    }
}

#[test]
fn read_non_existent() {
    let (client, table, path) = setup_test_bincode_table::<i32>();

    {
        let read = client.begin_read().unwrap();
        let result = table.read(&read, "1".to_string());
        assert!(matches!(result, Err(DatabaseError::NotFound { .. })));
    }

    {
        std::fs::remove_file(&path).unwrap();
    }
}

#[test]
fn read_in_write_transaction() {
    let (client, table, path) = setup_test_bincode_table::<i32>();

    {
        let mut write = client.begin_write().unwrap();
        table.insert(&mut write, "1".to_string(), &1).unwrap();
        write.commit().unwrap();
    }

    {
        let write = client.begin_write().unwrap();
        let result = table.read(&write, "1".to_string());
        assert!(matches!(result, Err(DatabaseError::Transaction(..))))
    }

    {
        std::fs::remove_file(&path).unwrap();
    }
}
