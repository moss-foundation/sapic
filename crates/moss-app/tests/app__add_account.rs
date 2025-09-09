#![cfg(feature = "integration-tests")]

pub mod shared;

use moss_app::models::operations::{AddAccountInput, CreateProfileInput};
use moss_user::models::primitives::AccountKind;

use crate::shared::set_up_test_app;

#[tokio::test]
async fn add_account_github_success() {
    let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

    // First create a profile
    let profile_result = app
        .create_profile(
            &ctx,
            &app_delegate,
            CreateProfileInput {
                name: "Test Profile".to_string(),
                is_default: Some(true),
            },
        )
        .await;
    assert!(profile_result.is_ok());

    // Add GitHub account
    let add_result = app
        .add_account(
            &ctx,
            &app_delegate,
            AddAccountInput {
                host: "github.com".to_string(),
                label: Some("Test GitHub Account".to_string()),
                kind: AccountKind::GitHub,
            },
        )
        .await;

    assert!(add_result.is_ok());
    let add_output = add_result.unwrap();

    // Verify account was added by checking it has a valid ID
    assert!(!add_output.account_id.to_string().is_empty());

    cleanup().await;
}

#[tokio::test]
async fn add_account_gitlab_success() {
    let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

    // First create a profile
    let profile_result = app
        .create_profile(
            &ctx,
            &app_delegate,
            CreateProfileInput {
                name: "Test Profile".to_string(),
                is_default: Some(true),
            },
        )
        .await;
    assert!(profile_result.is_ok());

    // Add GitLab account
    let add_result = app
        .add_account(
            &ctx,
            &app_delegate,
            AddAccountInput {
                host: "gitlab.com".to_string(),
                label: Some("Test GitLab Account".to_string()),
                kind: AccountKind::GitLab,
            },
        )
        .await;

    assert!(add_result.is_ok());
    let add_output = add_result.unwrap();

    // Verify account was added by checking it has a valid ID
    assert!(!add_output.account_id.to_string().is_empty());

    cleanup().await;
}

#[tokio::test]
async fn add_account_custom_host() {
    let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

    // First create a profile
    let profile_result = app
        .create_profile(
            &ctx,
            &app_delegate,
            CreateProfileInput {
                name: "Test Profile".to_string(),
                is_default: Some(true),
            },
        )
        .await;
    assert!(profile_result.is_ok());

    // Add account with custom host
    let custom_host = "git.example.com";
    let add_result = app
        .add_account(
            &ctx,
            &app_delegate,
            AddAccountInput {
                host: custom_host.to_string(),
                label: Some("Custom GitLab Instance".to_string()),
                kind: AccountKind::GitLab,
            },
        )
        .await;

    assert!(add_result.is_ok());
    let add_output = add_result.unwrap();

    // Verify account was added by checking it has a valid ID
    assert!(!add_output.account_id.to_string().is_empty());

    cleanup().await;
}

#[tokio::test]
async fn add_multiple_accounts() {
    let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

    // First create a profile
    let profile_result = app
        .create_profile(
            &ctx,
            &app_delegate,
            CreateProfileInput {
                name: "Test Profile".to_string(),
                is_default: Some(true),
            },
        )
        .await;
    assert!(profile_result.is_ok());

    // Add GitHub account
    let github_result = app
        .add_account(
            &ctx,
            &app_delegate,
            AddAccountInput {
                host: "github.com".to_string(),
                label: Some("GitHub Account".to_string()),
                kind: AccountKind::GitHub,
            },
        )
        .await;
    assert!(github_result.is_ok());
    let github_account = github_result.unwrap();

    // Add GitLab account
    let gitlab_result = app
        .add_account(
            &ctx,
            &app_delegate,
            AddAccountInput {
                host: "gitlab.com".to_string(),
                label: Some("GitLab Account".to_string()),
                kind: AccountKind::GitLab,
            },
        )
        .await;
    assert!(gitlab_result.is_ok());
    let gitlab_account = gitlab_result.unwrap();

    // Verify both accounts have different IDs
    assert_ne!(github_account.account_id, gitlab_account.account_id);
    assert!(!github_account.account_id.to_string().is_empty());
    assert!(!gitlab_account.account_id.to_string().is_empty());

    cleanup().await;
}
