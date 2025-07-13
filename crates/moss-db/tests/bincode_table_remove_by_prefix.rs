// pub mod shared;

// use moss_db::DatabaseClient;

// use crate::shared::setup_test_bincode_table;

// #[test]
// fn remove_by_prefix_empty() {
//     let (client, table, path) = setup_test_bincode_table::<i32>();

//     {
//         let mut write = client.begin_write().unwrap();
//         let result = table.remove_by_prefix(&mut write, "user:").unwrap();
//         assert_eq!(result.len(), 0);
//         write.commit().unwrap();
//     }

//     {
//         std::fs::remove_file(path).unwrap();
//     }
// }

// #[test]
// fn remove_by_prefix_hierarchical_keys() {
//     let (client, table, path) = setup_test_bincode_table::<i32>();

//     // Insert test data with hierarchical keys
//     {
//         let mut write = client.begin_write().unwrap();

//         // Keys with prefix "user:"
//         for i in 0..10 {
//             table
//                 .insert(&mut write, format!("user:{}:profile", i), &i)
//                 .unwrap();
//         }

//         // Keys with prefix "user:admin:"
//         for i in 10..15 {
//             table
//                 .insert(&mut write, format!("user:admin:{}", i), &i)
//                 .unwrap();
//         }

//         // Keys with prefix "order:"
//         for i in 50..60 {
//             table
//                 .insert(&mut write, format!("order:{}:details", i), &i)
//                 .unwrap();
//         }

//         // Keys with prefix "settings:"
//         for i in 100..105 {
//             table
//                 .insert(&mut write, format!("settings:{}:config", i), &i)
//                 .unwrap();
//         }

//         write.commit().unwrap();
//     }

//     // Count total entries before removal
//     let count_before = {
//         let read = client.begin_read().unwrap();
//         let all_entries = table.scan(&read).unwrap();
//         all_entries.collect::<Vec<_>>().len()
//     };
//     assert_eq!(count_before, 30); // 10 + 5 + 10 + 5 = 30

//     // Remove by prefix "user:" - should remove all user entries (including admin)
//     {
//         let mut write = client.begin_write().unwrap();
//         let result = table.remove_by_prefix(&mut write, "user:").unwrap();

//         assert_eq!(result.len(), 15); // 10 + 5 = 15 user entries

//         // All keys should start with "user:"
//         for (key, _) in &result {
//             assert!(key.starts_with("user:"));
//         }

//         // Verify all values are correct
//         let mut values: Vec<_> = result.iter().map(|(_, v)| *v).collect();
//         values.sort();
//         let expected_values: Vec<_> = (0..15).collect();
//         assert_eq!(values, expected_values);

//         write.commit().unwrap();
//     }

//     // Count total entries after removal
//     let count_after = {
//         let read = client.begin_read().unwrap();
//         let all_entries = table.scan(&read).unwrap();
//         all_entries.collect::<Vec<_>>().len()
//     };
//     assert_eq!(count_after, 15); // 30 - 15 = 15 remaining

//     // Verify that user entries are actually gone
//     {
//         let read = client.begin_read().unwrap();
//         let user_profile_result = table.read(&read, "user:0:profile".to_string());
//         assert!(user_profile_result.is_err());

//         let user_admin_result = table.read(&read, "user:admin:10".to_string());
//         assert!(user_admin_result.is_err());
//     }

//     // Verify that other entries still exist
//     {
//         let read = client.begin_read().unwrap();
//         let order_result = table.read(&read, "order:50:details".to_string()).unwrap();
//         assert_eq!(order_result, 50);

//         let settings_result = table
//             .read(&read, "settings:100:config".to_string())
//             .unwrap();
//         assert_eq!(settings_result, 100);
//     }

//     {
//         std::fs::remove_file(path).unwrap();
//     }
// }

// #[test]
// fn remove_by_prefix_specific_path() {
//     let (client, table, path) = setup_test_bincode_table::<i32>();

//     // Insert test data with nested hierarchical keys
//     {
//         let mut write = client.begin_write().unwrap();

//         // User profiles
//         table
//             .insert(&mut write, "user:1:profile:name".to_string(), &1)
//             .unwrap();
//         table
//             .insert(&mut write, "user:1:profile:email".to_string(), &2)
//             .unwrap();
//         table
//             .insert(&mut write, "user:1:settings:theme".to_string(), &3)
//             .unwrap();
//         table
//             .insert(&mut write, "user:2:profile:name".to_string(), &4)
//             .unwrap();
//         table
//             .insert(&mut write, "user:2:profile:email".to_string(), &5)
//             .unwrap();

//         // Other data
//         table
//             .insert(&mut write, "config:database:host".to_string(), &6)
//             .unwrap();
//         table
//             .insert(&mut write, "config:database:port".to_string(), &7)
//             .unwrap();

//         write.commit().unwrap();
//     }

//     // Remove only profile data for user:1
//     {
//         let mut write = client.begin_write().unwrap();
//         let result = table
//             .remove_by_prefix(&mut write, "user:1:profile:")
//             .unwrap();

//         assert_eq!(result.len(), 2); // name and email

//         // Verify removed keys
//         let keys: Vec<_> = result.iter().map(|(k, _)| k.clone()).collect();
//         assert!(keys.contains(&"user:1:profile:name".to_string()));
//         assert!(keys.contains(&"user:1:profile:email".to_string()));

//         write.commit().unwrap();
//     }

//     // Verify specific removal
//     {
//         let read = client.begin_read().unwrap();

//         // These should be gone
//         assert!(
//             table
//                 .read(&read, "user:1:profile:name".to_string())
//                 .is_err()
//         );
//         assert!(
//             table
//                 .read(&read, "user:1:profile:email".to_string())
//                 .is_err()
//         );

//         // These should still exist
//         assert!(
//             table
//                 .read(&read, "user:1:settings:theme".to_string())
//                 .is_ok()
//         );
//         assert!(table.read(&read, "user:2:profile:name".to_string()).is_ok());
//         assert!(
//             table
//                 .read(&read, "config:database:host".to_string())
//                 .is_ok()
//         );
//     }

//     {
//         std::fs::remove_file(path).unwrap();
//     }
// }

// #[test]
// fn remove_by_prefix_case_sensitivity() {
//     let (client, table, path) = setup_test_bincode_table::<i32>();

//     {
//         let mut write = client.begin_write().unwrap();

//         // Insert keys with mixed case
//         table
//             .insert(&mut write, "User:1:Profile".to_string(), &1)
//             .unwrap();
//         table
//             .insert(&mut write, "user:2:profile".to_string(), &2)
//             .unwrap();
//         table
//             .insert(&mut write, "USER:3:PROFILE".to_string(), &3)
//             .unwrap();

//         write.commit().unwrap();
//     }

//     // Case-sensitive removal should find exact matches only
//     {
//         let mut write = client.begin_write().unwrap();

//         // Capital "User:" should match only "User:1:Profile"
//         let result = table.remove_by_prefix(&mut write, "User:").unwrap();
//         assert_eq!(result.len(), 1);
//         assert_eq!(result[0].0, "User:1:Profile");
//         assert_eq!(result[0].1, 1);

//         write.commit().unwrap();
//     }

//     // Verify case-sensitive removal
//     {
//         let read = client.begin_read().unwrap();

//         // Should be gone
//         assert!(table.read(&read, "User:1:Profile".to_string()).is_err());

//         // Should still exist
//         assert!(table.read(&read, "user:2:profile".to_string()).is_ok());
//         assert!(table.read(&read, "USER:3:PROFILE".to_string()).is_ok());
//     }

//     {
//         std::fs::remove_file(path).unwrap();
//     }
// }

// #[test]
// fn remove_by_prefix_write_transaction_required() {
//     let (client, table, path) = setup_test_bincode_table::<i32>();

//     {
//         // This test demonstrates that remove_by_prefix properly requires write transactions
//         // The method signature requires &mut Transaction, so read transactions can't be passed

//         let mut write = client.begin_write().unwrap();

//         // This should work fine with a write transaction
//         let result = table.remove_by_prefix(&mut write, "test:");
//         assert!(result.is_ok());

//         write.commit().unwrap();
//     }

//     std::fs::remove_file(path).unwrap();
// }

// #[test]
// fn remove_by_prefix_empty_prefix() {
//     let (client, table, path) = setup_test_bincode_table::<i32>();

//     // Insert test data
//     {
//         let mut write = client.begin_write().unwrap();
//         table
//             .insert(&mut write, "user:1:name".to_string(), &1)
//             .unwrap();
//         table
//             .insert(&mut write, "order:2:total".to_string(), &2)
//             .unwrap();
//         table
//             .insert(&mut write, "config:3:value".to_string(), &3)
//             .unwrap();
//         write.commit().unwrap();
//     }

//     // Remove with empty prefix should remove all entries
//     {
//         let mut write = client.begin_write().unwrap();
//         let result = table.remove_by_prefix(&mut write, "").unwrap();

//         assert_eq!(result.len(), 3);
//         write.commit().unwrap();
//     }

//     // Verify all entries are gone
//     {
//         let read = client.begin_read().unwrap();
//         let all_entries = table.scan(&read).unwrap();
//         assert_eq!(all_entries.collect::<Vec<_>>().len(), 0);
//     }

//     {
//         std::fs::remove_file(path).unwrap();
//     }
// }

// #[test]
// fn remove_by_prefix_non_existent_prefix() {
//     let (client, table, path) = setup_test_bincode_table::<i32>();

//     // Insert test data
//     {
//         let mut write = client.begin_write().unwrap();
//         table
//             .insert(&mut write, "user:1:name".to_string(), &1)
//             .unwrap();
//         table
//             .insert(&mut write, "order:2:total".to_string(), &2)
//             .unwrap();
//         write.commit().unwrap();
//     }

//     // Remove with non-existent prefix
//     {
//         let mut write = client.begin_write().unwrap();
//         let result = table.remove_by_prefix(&mut write, "nonexistent:").unwrap();

//         assert_eq!(result.len(), 0);
//         write.commit().unwrap();
//     }

//     // Verify no entries were removed
//     {
//         let read = client.begin_read().unwrap();
//         let all_entries = table.scan(&read).unwrap();
//         assert_eq!(all_entries.collect::<Vec<_>>().len(), 2);
//     }

//     {
//         std::fs::remove_file(path).unwrap();
//     }
// }

// #[test]
// fn remove_by_prefix_with_complex_data() {
//     use crate::shared::{TEST_NODE_1, TEST_NODE_2, TEST_NODE_3, TestNode};

//     let (client, table, path) = setup_test_bincode_table::<TestNode>();

//     {
//         let mut write = client.begin_write().unwrap();

//         // Insert different types of test nodes with hierarchical keys
//         table
//             .insert(&mut write, "tree:branch:1".to_string(), &*TEST_NODE_1)
//             .unwrap();
//         table
//             .insert(&mut write, "tree:leaf:1".to_string(), &*TEST_NODE_2)
//             .unwrap();
//         table
//             .insert(&mut write, "tree:branch:2".to_string(), &*TEST_NODE_3)
//             .unwrap();
//         table
//             .insert(&mut write, "config:other:1".to_string(), &*TEST_NODE_1)
//             .unwrap();

//         write.commit().unwrap();
//     }

//     // Remove by prefix "tree:branch:"
//     {
//         let mut write = client.begin_write().unwrap();
//         let result = table.remove_by_prefix(&mut write, "tree:branch:").unwrap();

//         assert_eq!(result.len(), 2);

//         // Verify the correct entries were removed
//         let keys: Vec<_> = result.iter().map(|(k, _)| k.clone()).collect();
//         assert!(keys.contains(&"tree:branch:1".to_string()));
//         assert!(keys.contains(&"tree:branch:2".to_string()));

//         write.commit().unwrap();
//     }

//     // Verify removal worked
//     {
//         let read = client.begin_read().unwrap();

//         // Should be gone
//         assert!(table.read(&read, "tree:branch:1".to_string()).is_err());
//         assert!(table.read(&read, "tree:branch:2".to_string()).is_err());

//         // Should still exist
//         assert!(table.read(&read, "tree:leaf:1".to_string()).is_ok());
//         assert!(table.read(&read, "config:other:1".to_string()).is_ok());
//     }

//     std::fs::remove_file(path).unwrap();
// }

// #[test]
// fn remove_by_prefix_with_different_prefix_types() {
//     let (client, table, path) = setup_test_bincode_table::<i32>();

//     {
//         let mut write = client.begin_write().unwrap();
//         table
//             .insert(&mut write, "test:key:1".to_string(), &1)
//             .unwrap();
//         table
//             .insert(&mut write, "test:key:2".to_string(), &2)
//             .unwrap();
//         write.commit().unwrap();
//     }

//     // Test with different types that implement AsRef<str>
//     {
//         let mut write = client.begin_write().unwrap();

//         // String
//         let result = table
//             .remove_by_prefix(&mut write, "test:key:1".to_string())
//             .unwrap();
//         assert_eq!(result.len(), 1);
//         assert_eq!(result[0].0, "test:key:1");

//         // &str
//         let result = table.remove_by_prefix(&mut write, "test:key:2").unwrap();
//         assert_eq!(result.len(), 1);
//         assert_eq!(result[0].0, "test:key:2");

//         write.commit().unwrap();
//     }

//     // Verify both entries were removed
//     {
//         let read = client.begin_read().unwrap();
//         let all_entries = table.scan(&read).unwrap();
//         assert_eq!(all_entries.collect::<Vec<_>>().len(), 0);
//     }

//     std::fs::remove_file(path).unwrap();
// }
