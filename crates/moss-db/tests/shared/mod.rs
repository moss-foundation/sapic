use moss_db::bincode_table::BincodeTable;
use moss_db::encrypted_bincode_table::{EncryptedBincodeTable, EncryptionOptions};
use moss_db::ReDbClient;
use moss_testutils::random_name::random_string;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::string::ToString;
use std::sync::LazyLock;

pub(crate) const TEST_PASSWORD_1: &[u8] = "password_1".as_bytes();
pub(crate) const TEST_PASSWORD_2: &[u8] = "password_2".as_bytes();
pub(crate) const TEST_AAD_1: &[u8] = "aad_1".as_bytes();
pub(crate) const TEST_AAD_2: &[u8] = "aad_2".as_bytes();

fn random_db_name() -> String {
    format!("Test_{}.db", random_string(10))
}

pub(crate) fn test_db_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(random_db_name())
}

pub fn setup_test_bincode_table<T>() -> (ReDbClient, BincodeTable<'static, String, T>, PathBuf)
where
    T: Serialize + DeserializeOwned,
{
    let test_db_path = test_db_path();
    let bincode_table = BincodeTable::new("test");
    let client = ReDbClient::new(test_db_path.clone())
        .unwrap()
        .with_table(&bincode_table)
        .unwrap();

    (client, bincode_table, test_db_path)
}

pub fn setup_test_encrypted_bincode_table<T>() -> (
    ReDbClient,
    EncryptedBincodeTable<'static, String, T>,
    PathBuf,
)
where
    T: Serialize + DeserializeOwned,
{
    let test_db_path = test_db_path();
    let encrypted_bincode_table = EncryptedBincodeTable::new("test", EncryptionOptions::default());

    let client = ReDbClient::new(test_db_path.clone())
        .unwrap()
        .with_table(&encrypted_bincode_table)
        .unwrap();

    (client, encrypted_bincode_table, test_db_path)
}

// A basic test type modelled after `EditorGridNode` in `moss-workspace`
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub(crate) struct TestLeafData {
    pub view: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub(crate) enum TestNode {
    Branch { data: Vec<TestNode>, size: f64 },
    Leaf { data: TestLeafData, size: f64 },
}

pub(crate) static TEST_NODE_1: LazyLock<TestNode> = LazyLock::new(|| TestNode::Branch {
    data: vec![],
    size: 10.0,
});
pub(crate) static TEST_NODE_2: LazyLock<TestNode> = LazyLock::new(|| TestNode::Leaf {
    data: TestLeafData {
        view: vec!["view".to_string()],
    },
    size: 10.0,
});
pub(crate) static TEST_NODE_3: LazyLock<TestNode> = LazyLock::new(|| TestNode::Branch {
    data: vec![TestNode::Leaf {
        data: TestLeafData {
            view: vec!["view".to_string()],
        },
        size: 10.0,
    }],
    size: 10.0,
});
