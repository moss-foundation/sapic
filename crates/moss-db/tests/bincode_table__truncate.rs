pub mod shared;

use moss_db::DatabaseClient;
use moss_db::common::DatabaseError;

use crate::shared::setup_test_bincode_table;

#[test]
fn truncate_empty() {
    let (client, table, path) = setup_test_bincode_table::<i32>();

    {
        let mut write = client.begin_write().unwrap();
        let result = table.truncate(&mut write);
        assert!(result.is_ok());
        write.commit().unwrap();
    }

    {
        let read = client.begin_read().unwrap();
        assert_eq!(table.scan(&read).unwrap().count(), 0);
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn truncate_non_empty() {
    let (client, table, path) = setup_test_bincode_table::<i32>();

    {
        let mut write = client.begin_write().unwrap();
        table.insert(&mut write, "1".to_string(), &1).unwrap();
        table.insert(&mut write, "2".to_string(), &2).unwrap();
        table.insert(&mut write, "3".to_string(), &3).unwrap();
    }

    {
        let mut write = client.begin_write().unwrap();
        let result = table.truncate(&mut write);
        assert!(result.is_ok());
    }

    {
        let read = client.begin_read().unwrap();
        assert_eq!(table.scan(&read).unwrap().count(), 0);
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn truncate_in_read_transaction() {
    let (client, table, path) = setup_test_bincode_table::<i32>();

    {
        let mut read = client.begin_read().unwrap();
        let result = table.truncate(&mut read);
        assert!(matches!(result, Err(DatabaseError::Transaction(..))));
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn truncate_uncommitted() {
    let (client, table, path) = setup_test_bincode_table::<i32>();

    {
        let mut write = client.begin_write().unwrap();
        table.insert(&mut write, "1".to_string(), &1).unwrap();
        table.insert(&mut write, "2".to_string(), &2).unwrap();
        table.insert(&mut write, "3".to_string(), &3).unwrap();
        write.commit().unwrap();
    }

    {
        // Uncommitted
        let mut write = client.begin_write().unwrap();
        table.truncate(&mut write).unwrap();
    }

    {
        let read = client.begin_read().unwrap();
        assert_eq!(table.scan(&read).unwrap().count(), 3);
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}
