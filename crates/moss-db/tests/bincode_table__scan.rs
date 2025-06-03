pub mod shared;

use moss_db::{DatabaseClient, common::DatabaseError};

use crate::shared::setup_test_bincode_table;

#[test]
fn scan_empty() {
    let (client, table, path) = setup_test_bincode_table::<i32>();

    {
        let read = client.begin_read().unwrap();
        assert_eq!(table.scan(&read).unwrap().into_iter().count(), 0);
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn scan_multiple() {
    let (client, table, path) = setup_test_bincode_table::<i32>();

    {
        let mut write = client.begin_write().unwrap();
        for i in 0..100 {
            table.insert(&mut write, i.to_string(), &i).unwrap();
        }
        write.commit().unwrap();
    }

    let expected = (0..100).map(|i| (i.to_string(), i)).collect::<Vec<_>>();
    {
        let read = client.begin_read().unwrap();
        let mut scan_result = table.scan(&read).unwrap().into_iter().collect::<Vec<_>>();
        scan_result.sort_by_key(|(_, i)| *i);
        assert_eq!(scan_result, expected);
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn scan_in_write_transaction() {
    let (client, table, path) = setup_test_bincode_table::<i32>();

    {
        let write = client.begin_write().unwrap();
        let result = table.scan(&write);
        assert!(matches!(result, Err(DatabaseError::Transaction(_))));
    }

    std::fs::remove_file(path).unwrap();
}
