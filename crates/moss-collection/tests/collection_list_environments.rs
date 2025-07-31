// TODO: cant really test this, need to implement environments functionality first
#![cfg(feature = "integration-tests")]

// Trigger a CI workflow

#[tokio::test]
async fn test_warnings() {
    let x = 42;
}
