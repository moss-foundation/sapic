pub mod shared;

use moss_db::{DatabaseClient, common::DatabaseError};

use crate::shared::setup_test_bincode_table;

#[test]
fn remove_success() {
    let (client, table, path) = setup_test_bincode_table::<i32>();

    {
        let mut write = client.begin_write().unwrap();
        table.insert(&mut write, "1".to_string(), &1).unwrap();
        table.insert(&mut write, "2".to_string(), &2).unwrap();
        write.commit().unwrap();
    }

    {
        let mut write = client.begin_write().unwrap();
        let result = table.remove(&mut write, "1".to_string());
        assert_eq!(result.ok(), Some(1));
        write.commit().unwrap();
    }

    let expected = vec![("2".to_string(), 2)];
    {
        // Check the entry is removed from the db
        let read = client.begin_read().unwrap();
        assert_eq!(table.scan(&read).unwrap().collect::<Vec<_>>(), expected);
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn remove_nonexistent() {
    let (client, table, path) = setup_test_bincode_table::<i32>();

    {
        let mut write = client.begin_write().unwrap();
        let result = table.remove(&mut write, "1".to_string());
        assert!(matches!(result, Err(DatabaseError::NotFound { .. })));
        write.commit().unwrap();
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn remove_in_read_transaction() {
    let (client, table, path) = setup_test_bincode_table::<i32>();

    {
        let mut write = client.begin_write().unwrap();
        table.insert(&mut write, "1".to_string(), &1).unwrap();
        write.commit().unwrap();
    }

    {
        let mut read = client.begin_read().unwrap();
        let result = table.remove(&mut read, "1".to_string());
        assert!(matches!(result, Err(DatabaseError::Transaction(..))))
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn remove_uncommitted() {
    let (client, table, path) = setup_test_bincode_table::<i32>();

    {
        let mut write = client.begin_write().unwrap();
        table.insert(&mut write, "1".to_string(), &1).unwrap();
        write.commit().unwrap();
    }

    {
        let mut write = client.begin_write().unwrap();
        let _ = table.remove(&mut write, "1".to_string());
    }

    let expected = vec![("1".to_string(), 1)];
    {
        // Check that the change is not committed
        let read = client.begin_read().unwrap();
        assert_eq!(table.scan(&read).unwrap().collect::<Vec<_>>(), expected);
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}
