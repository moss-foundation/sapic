use crate::shared::setup_test_bincode_table;

use moss_db::common::DatabaseError;
use moss_db::DatabaseClient;

mod shared;

#[test]
fn insert_success() {
    let (client, table, path) = setup_test_bincode_table();

    {
        let mut write = client.begin_write().unwrap();
        table.insert(&mut write, "1".to_string(), &1).unwrap();
        table.insert(&mut write, "2".to_string(), &2).unwrap();
        table.insert(&mut write, "3".to_string(), &3).unwrap();
        write.commit().unwrap();
    }

    let expected = vec![
        ("1".to_string(), 1),
        ("2".to_string(), 2),
        ("3".to_string(), 3),
    ];

    {
        let read = client.begin_read().unwrap();

        assert_eq!(table.scan(&read).unwrap().collect::<Vec<_>>(), expected)
    }
    // Cleanup
    {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn insert_existing_key() {
    let (client, table, path) = setup_test_bincode_table();

    {
        let mut write = client.begin_write().unwrap();
        table.insert(&mut write, "1".to_string(), &1).unwrap();
        write.commit().unwrap();
    }

    {
        // Overwrite existing key
        let mut write = client.begin_write().unwrap();
        table.insert(&mut write, "1".to_string(), &2).unwrap();
        write.commit().unwrap();
    }

    let expected = vec![("1".to_string(), 2)];
    {
        let read = client.begin_read().unwrap();
        assert_eq!(table.scan(&read).unwrap().collect::<Vec<_>>(), expected)
    }

    // Cleanup
    {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn insert_in_read_transaction() {
    let (client, table, path) = setup_test_bincode_table();

    {
        let mut read = client.begin_read().unwrap();
        let result = table.insert(&mut read, "1".to_string(), &1);
        assert!(matches!(result, Err(DatabaseError::Transaction(..))));
    }

    // Cleanup
    {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn insert_uncommitted() {
    let (client, table, path) = setup_test_bincode_table();

    {
        // Uncommitted transaction
        let mut write = client.begin_write().unwrap();
        table.insert(&mut write, "1".to_string(), &1).unwrap();
    }

    {
        let read = client.begin_read().unwrap();
        assert!(table.scan(&read).unwrap().collect::<Vec<_>>().is_empty());
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}
