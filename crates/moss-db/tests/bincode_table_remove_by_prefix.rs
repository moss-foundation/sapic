pub mod shared;

use moss_db::DatabaseClientWithContext;

use crate::shared::setup_test_bincode_table;

#[tokio::test]
async fn remove_by_prefix_empty() {
    let (client, ctx, table, path) = setup_test_bincode_table::<i32>();

    {
        let mut write = client.begin_write_with_context(&ctx).await.unwrap();
        let result = table
            .remove_by_prefix_with_context(&ctx, &mut write, "user:")
            .await
            .unwrap();
        assert_eq!(result.len(), 0);
        write.commit().unwrap();
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[tokio::test]
async fn remove_by_prefix_hierarchical_keys() {
    let (client, ctx, table, path) = setup_test_bincode_table::<i32>();

    // Insert test data with hierarchical keys
    {
        let mut write = client.begin_write_with_context(&ctx).await.unwrap();

        // Keys with prefix "user:"
        for i in 0..10 {
            table
                .insert_with_context(&ctx, &mut write, format!("user:{}:profile", i), &i)
                .await
                .unwrap();
        }

        // Keys with prefix "user:admin:"
        for i in 10..15 {
            table
                .insert_with_context(&ctx, &mut write, format!("user:admin:{}", i), &i)
                .await
                .unwrap();
        }

        // Keys with prefix "order:"
        for i in 50..60 {
            table
                .insert_with_context(&ctx, &mut write, format!("order:{}:details", i), &i)
                .await
                .unwrap();
        }

        // Keys with prefix "settings:"
        for i in 100..105 {
            table
                .insert_with_context(&ctx, &mut write, format!("settings:{}:config", i), &i)
                .await
                .unwrap();
        }

        write.commit().unwrap();
    }

    // Count total entries before removal
    let count_before = {
        let read = client.begin_read_with_context(&ctx).await.unwrap();
        let all_entries = table.scan_with_context(&ctx, &read).await.unwrap();
        all_entries.collect::<Vec<_>>().len()
    };
    assert_eq!(count_before, 30); // 10 + 5 + 10 + 5 = 30

    // Remove by prefix "user:" - should remove all user entries (including admin)
    {
        let mut write = client.begin_write_with_context(&ctx).await.unwrap();
        let result = table
            .remove_by_prefix_with_context(&ctx, &mut write, "user:")
            .await
            .unwrap();

        assert_eq!(result.len(), 15); // 10 + 5 = 15 user entries

        // All keys should start with "user:"
        for (key, _) in &result {
            assert!(key.starts_with("user:"));
        }

        // Verify all values are correct
        let mut values: Vec<_> = result.iter().map(|(_, v)| *v).collect();
        values.sort();
        let expected_values: Vec<_> = (0..15).collect();
        assert_eq!(values, expected_values);

        write.commit().unwrap();
    }

    // Count total entries after removal
    let count_after = {
        let read = client.begin_read_with_context(&ctx).await.unwrap();
        let all_entries = table.scan_with_context(&ctx, &read).await.unwrap();
        all_entries.collect::<Vec<_>>().len()
    };
    assert_eq!(count_after, 15); // 30 - 15 = 15 remaining

    // Verify that user entries are actually gone
    {
        let read = client.begin_read_with_context(&ctx).await.unwrap();
        let user_profile_result = table
            .read_with_context(&ctx, &read, "user:0:profile".to_string())
            .await;
        assert!(user_profile_result.is_err());

        let user_admin_result = table
            .read_with_context(&ctx, &read, "user:admin:10".to_string())
            .await;
        assert!(user_admin_result.is_err());
    }

    // Verify that other entries still exist
    {
        let read = client.begin_read_with_context(&ctx).await.unwrap();
        let order_result = table
            .read_with_context(&ctx, &read, "order:50:details".to_string())
            .await
            .unwrap();
        assert_eq!(order_result, 50);

        let settings_result = table
            .read_with_context(&ctx, &read, "settings:100:config".to_string())
            .await
            .unwrap();
        assert_eq!(settings_result, 100);
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[tokio::test]
async fn remove_by_prefix_specific_path() {
    let (client, ctx, table, path) = setup_test_bincode_table::<i32>();

    // Insert test data with nested hierarchical keys
    {
        let mut write = client.begin_write_with_context(&ctx).await.unwrap();

        // User profiles
        table
            .insert_with_context(&ctx, &mut write, "user:1:profile:name".to_string(), &1)
            .await
            .unwrap();
        table
            .insert_with_context(&ctx, &mut write, "user:1:profile:email".to_string(), &2)
            .await
            .unwrap();
        table
            .insert_with_context(&ctx, &mut write, "user:1:settings:theme".to_string(), &3)
            .await
            .unwrap();
        table
            .insert_with_context(&ctx, &mut write, "user:2:profile:name".to_string(), &4)
            .await
            .unwrap();
        table
            .insert_with_context(&ctx, &mut write, "user:2:profile:email".to_string(), &5)
            .await
            .unwrap();

        // Other data
        table
            .insert_with_context(&ctx, &mut write, "config:database:host".to_string(), &6)
            .await
            .unwrap();
        table
            .insert_with_context(&ctx, &mut write, "config:database:port".to_string(), &7)
            .await
            .unwrap();

        write.commit().unwrap();
    }

    // Remove only profile data for user:1
    {
        let mut write = client.begin_write_with_context(&ctx).await.unwrap();
        let result = table
            .remove_by_prefix_with_context(&ctx, &mut write, "user:1:profile:")
            .await
            .unwrap();

        assert_eq!(result.len(), 2); // name and email

        // Verify removed keys
        let keys: Vec<_> = result.iter().map(|(k, _)| k.clone()).collect();
        assert!(keys.contains(&"user:1:profile:name".to_string()));
        assert!(keys.contains(&"user:1:profile:email".to_string()));

        write.commit().unwrap();
    }

    // Verify specific removal
    {
        let read = client.begin_read_with_context(&ctx).await.unwrap();

        // These should be gone
        assert!(
            table
                .read_with_context(&ctx, &read, "user:1:profile:name".to_string())
                .await
                .is_err()
        );
        assert!(
            table
                .read_with_context(&ctx, &read, "user:1:profile:email".to_string())
                .await
                .is_err()
        );

        // These should still exist
        assert!(
            table
                .read_with_context(&ctx, &read, "user:1:settings:theme".to_string())
                .await
                .is_ok()
        );
        assert!(
            table
                .read_with_context(&ctx, &read, "user:2:profile:name".to_string())
                .await
                .is_ok()
        );
        assert!(
            table
                .read_with_context(&ctx, &read, "config:database:host".to_string())
                .await
                .is_ok()
        );
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[tokio::test]
async fn remove_by_prefix_case_sensitivity() {
    let (client, ctx, table, path) = setup_test_bincode_table::<i32>();

    {
        let mut write = client.begin_write_with_context(&ctx).await.unwrap();

        // Insert keys with mixed case
        table
            .insert_with_context(&ctx, &mut write, "User:1:Profile".to_string(), &1)
            .await
            .unwrap();
        table
            .insert_with_context(&ctx, &mut write, "user:2:profile".to_string(), &2)
            .await
            .unwrap();
        table
            .insert_with_context(&ctx, &mut write, "USER:3:PROFILE".to_string(), &3)
            .await
            .unwrap();

        write.commit().unwrap();
    }

    // Case-sensitive removal should find exact matches only
    {
        let mut write = client.begin_write_with_context(&ctx).await.unwrap();

        // Capital "User:" should match only "User:1:Profile"
        let result = table
            .remove_by_prefix_with_context(&ctx, &mut write, "User:")
            .await
            .unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "User:1:Profile");
        assert_eq!(result[0].1, 1);

        write.commit().unwrap();
    }

    // Verify case-sensitive removal
    {
        let read = client.begin_read_with_context(&ctx).await.unwrap();

        // Should be gone
        assert!(
            table
                .read_with_context(&ctx, &read, "User:1:Profile".to_string())
                .await
                .is_err()
        );

        // Should still exist
        assert!(
            table
                .read_with_context(&ctx, &read, "user:2:profile".to_string())
                .await
                .is_ok()
        );
        assert!(
            table
                .read_with_context(&ctx, &read, "USER:3:PROFILE".to_string())
                .await
                .is_ok()
        );
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[tokio::test]
async fn remove_by_prefix_write_transaction_required() {
    let (client, ctx, table, path) = setup_test_bincode_table::<i32>();

    {
        // This test demonstrates that remove_by_prefix properly requires write transactions
        // The method signature requires &mut Transaction, so read transactions can't be passed

        let mut write = client.begin_write_with_context(&ctx).await.unwrap();

        // This should work fine with a write transaction
        let result = table
            .remove_by_prefix_with_context(&ctx, &mut write, "test:")
            .await;
        assert!(result.is_ok());

        write.commit().unwrap();
    }

    std::fs::remove_file(path).unwrap();
}

#[tokio::test]
async fn remove_by_prefix_empty_prefix() {
    let (client, ctx, table, path) = setup_test_bincode_table::<i32>();

    // Insert test data
    {
        let mut write = client.begin_write_with_context(&ctx).await.unwrap();
        table
            .insert_with_context(&ctx, &mut write, "user:1:name".to_string(), &1)
            .await
            .unwrap();
        table
            .insert_with_context(&ctx, &mut write, "order:2:total".to_string(), &2)
            .await
            .unwrap();
        table
            .insert_with_context(&ctx, &mut write, "config:3:value".to_string(), &3)
            .await
            .unwrap();
        write.commit().unwrap();
    }

    // Remove with empty prefix should remove all entries
    {
        let mut write = client.begin_write_with_context(&ctx).await.unwrap();
        let result = table
            .remove_by_prefix_with_context(&ctx, &mut write, "")
            .await
            .unwrap();

        assert_eq!(result.len(), 3);
        write.commit().unwrap();
    }

    // Verify all entries are gone
    {
        let read = client.begin_read_with_context(&ctx).await.unwrap();
        let all_entries = table.scan_with_context(&ctx, &read).await.unwrap();
        assert_eq!(all_entries.collect::<Vec<_>>().len(), 0);
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[tokio::test]
async fn remove_by_prefix_non_existent_prefix() {
    let (client, ctx, table, path) = setup_test_bincode_table::<i32>();

    // Insert test data
    {
        let mut write = client.begin_write_with_context(&ctx).await.unwrap();
        table
            .insert_with_context(&ctx, &mut write, "user:1:name".to_string(), &1)
            .await
            .unwrap();
        table
            .insert_with_context(&ctx, &mut write, "order:2:total".to_string(), &2)
            .await
            .unwrap();
        write.commit().unwrap();
    }

    // Remove with non-existent prefix
    {
        let mut write = client.begin_write_with_context(&ctx).await.unwrap();
        let result = table
            .remove_by_prefix_with_context(&ctx, &mut write, "nonexistent:")
            .await
            .unwrap();

        assert_eq!(result.len(), 0);
        write.commit().unwrap();
    }

    // Verify no entries were removed
    {
        let read = client.begin_read_with_context(&ctx).await.unwrap();
        let all_entries = table.scan_with_context(&ctx, &read).await.unwrap();
        assert_eq!(all_entries.collect::<Vec<_>>().len(), 2);
    }

    {
        std::fs::remove_file(path).unwrap();
    }
}

#[tokio::test]
async fn remove_by_prefix_with_complex_data() {
    use crate::shared::{TEST_NODE_1, TEST_NODE_2, TEST_NODE_3, TestNode};

    let (client, ctx, table, path) = setup_test_bincode_table::<TestNode>();

    {
        let mut write = client.begin_write_with_context(&ctx).await.unwrap();

        // Insert different types of test nodes with hierarchical keys
        table
            .insert_with_context(&ctx, &mut write, "tree:branch:1".to_string(), &*TEST_NODE_1)
            .await
            .unwrap();
        table
            .insert_with_context(&ctx, &mut write, "tree:leaf:1".to_string(), &*TEST_NODE_2)
            .await
            .unwrap();
        table
            .insert_with_context(&ctx, &mut write, "tree:branch:2".to_string(), &*TEST_NODE_3)
            .await
            .unwrap();
        table
            .insert_with_context(
                &ctx,
                &mut write,
                "config:other:1".to_string(),
                &*TEST_NODE_1,
            )
            .await
            .unwrap();

        write.commit().unwrap();
    }

    // Remove by prefix "tree:branch:"
    {
        let mut write = client.begin_write_with_context(&ctx).await.unwrap();
        let result = table
            .remove_by_prefix_with_context(&ctx, &mut write, "tree:branch:")
            .await
            .unwrap();

        assert_eq!(result.len(), 2);

        // Verify the correct entries were removed
        let keys: Vec<_> = result.iter().map(|(k, _)| k.clone()).collect();
        assert!(keys.contains(&"tree:branch:1".to_string()));
        assert!(keys.contains(&"tree:branch:2".to_string()));

        write.commit().unwrap();
    }

    // Verify removal worked
    {
        let read = client.begin_read_with_context(&ctx).await.unwrap();

        // Should be gone
        assert!(
            table
                .read_with_context(&ctx, &read, "tree:branch:1".to_string())
                .await
                .is_err()
        );
        assert!(
            table
                .read_with_context(&ctx, &read, "tree:branch:2".to_string())
                .await
                .is_err()
        );

        // Should still exist
        assert!(
            table
                .read_with_context(&ctx, &read, "tree:leaf:1".to_string())
                .await
                .is_ok()
        );
        assert!(
            table
                .read_with_context(&ctx, &read, "config:other:1".to_string())
                .await
                .is_ok()
        );
    }

    std::fs::remove_file(path).unwrap();
}

#[tokio::test]
async fn remove_by_prefix_with_different_prefix_types() {
    let (client, ctx, table, path) = setup_test_bincode_table::<i32>();

    {
        let mut write = client.begin_write_with_context(&ctx).await.unwrap();
        table
            .insert_with_context(&ctx, &mut write, "test:key:1".to_string(), &1)
            .await
            .unwrap();
        table
            .insert_with_context(&ctx, &mut write, "test:key:2".to_string(), &2)
            .await
            .unwrap();
        write.commit().unwrap();
    }

    // Test with different types that implement AsRef<str>
    {
        let mut write = client.begin_write_with_context(&ctx).await.unwrap();

        // String
        let result = table
            .remove_by_prefix_with_context(&ctx, &mut write, "test:key:1".to_string())
            .await
            .unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "test:key:1");

        // &str
        let result = table
            .remove_by_prefix_with_context(&ctx, &mut write, "test:key:2")
            .await
            .unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "test:key:2");

        write.commit().unwrap();
    }

    // Verify both entries were removed
    {
        let read = client.begin_read_with_context(&ctx).await.unwrap();
        let all_entries = table.scan_with_context(&ctx, &read).await.unwrap();
        assert_eq!(all_entries.collect::<Vec<_>>().len(), 0);
    }

    std::fs::remove_file(path).unwrap();
}
