const TABLE_VAULT_2: EncryptedBincodeTable<String, MyStruct> =
    EncryptedBincodeTable::new("vault_2");

#[test]
fn test_encrypted_write_read() {
    let client = ReDbClient::new("sapic.db").unwrap();
    let store = EncryptedBincodeStore::new(
        client,
        TABLE_VAULT_2,
        EncryptionConfig {
            memory_cost: 65536,
            time_cost: 10,
            parallelism: 4,
            salt_len: 32,
            nonce_len: 12,
        },
    );

    store
        .write(|mut txn, table, config| {
            table
                .write(
                    &mut txn,
                    "my_key".to_string(),
                    &MyStruct { val: 42 },
                    TEST_PASSWORD,
                    TEST_AAD,
                    config,
                )
                .unwrap();

            Ok(txn.commit()?)
        })
        .unwrap();

    let r = store
        .read(|txn, table, config| {
            let r = table.read(&txn, "my_key".to_string(), TEST_PASSWORD, TEST_AAD, config)?;

            Ok(r)
        })
        .unwrap();

    println!("{:?}", r);
}
