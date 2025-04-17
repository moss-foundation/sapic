use crate::shared::setup_test_bincode_table;
use moss_db::DatabaseClient;

mod shared;

#[test]
fn truncate_empty() {
    let (client, table, path) = setup_test_bincode_table();

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
