// TODO: restore this in the crate where these operations will be moved.

// #![cfg(feature = "integration-tests")]

// pub mod shared;

// use moss_user::models::primitives::AccountKind;
// use window::{
//     models::{
//         operations::{CreateProfileInput, UpdateProfileInput},
//         types::AddAccountParams,
//     },
//     window::OnWindowReadyOptions,
// };

// use crate::shared::{TEST_GITHUB_USERNAME, TEST_GITLAB_USERNAME, set_up_test_app};

// #[tokio::test]
// async fn add_account_github_success() {
//     let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

//     // First create a profile
//     let profile_result = app
//         .create_profile(
//             &ctx,
//             &app_delegate,
//             CreateProfileInput {
//                 name: "Test Profile".to_string(),
//                 is_default: Some(true),
//             },
//         )
//         .await;
//     assert!(profile_result.is_ok());

//     // Add GitHub account using update_profile
//     let update_result = app
//         .update_profile(
//             &ctx,
//             &app_delegate,
//             UpdateProfileInput {
//                 accounts_to_add: vec![AddAccountParams {
//                     host: "github.com".to_string(),
//                     kind: AccountKind::GitHub,
//                     pat: None,
//                 }],
//                 accounts_to_remove: vec![],
//                 accounts_to_update: vec![],
//             },
//         )
//         .await;

//     assert!(update_result.is_ok());
//     let update_output = update_result.unwrap();
//     assert_eq!(update_output.added_accounts.len(), 1);
//     assert_eq!(update_output.removed_accounts.len(), 0);

//     let account_id = &update_output.added_accounts[0];

//     // Verify account was added to active profile
//     let active_profile = app
//         .active_profile()
//         .await
//         .expect("active profile should exist");
//     let account = active_profile.account(account_id).await.unwrap().info();

//     assert_eq!(account.username, TEST_GITHUB_USERNAME);
//     assert_eq!(account.host, "github.com");
//     assert_eq!(account.kind, AccountKind::GitHub);

//     cleanup().await;
// }

// #[tokio::test]
// async fn add_account_gitlab_success() {
//     let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

//     // First create a profile
//     let profile_result = app
//         .create_profile(
//             &ctx,
//             &app_delegate,
//             CreateProfileInput {
//                 name: "Test Profile".to_string(),
//                 is_default: Some(true),
//             },
//         )
//         .await;
//     assert!(profile_result.is_ok());

//     // Add GitLab account using update_profile
//     let update_result = app
//         .update_profile(
//             &ctx,
//             &app_delegate,
//             UpdateProfileInput {
//                 accounts_to_add: vec![AddAccountParams {
//                     host: "gitlab.com".to_string(),
//                     kind: AccountKind::GitLab,
//                     pat: None,
//                 }],
//                 accounts_to_remove: vec![],
//                 accounts_to_update: vec![],
//             },
//         )
//         .await;

//     assert!(update_result.is_ok());
//     let update_output = update_result.unwrap();
//     assert_eq!(update_output.added_accounts.len(), 1);
//     assert_eq!(update_output.removed_accounts.len(), 0);

//     let account_id = &update_output.added_accounts[0];

//     // Verify account was added to active profile
//     let active_profile = app
//         .active_profile()
//         .await
//         .expect("active profile should exist");
//     let account = active_profile.account(account_id).await.unwrap().info();

//     assert_eq!(account.username, TEST_GITLAB_USERNAME);
//     assert_eq!(account.host, "gitlab.com");
//     assert_eq!(account.kind, AccountKind::GitLab);

//     cleanup().await;
// }

// #[tokio::test]
// async fn add_account_custom_host() {
//     let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

//     // First create a profile
//     let profile_result = app
//         .create_profile(
//             &ctx,
//             &app_delegate,
//             CreateProfileInput {
//                 name: "Test Profile".to_string(),
//                 is_default: Some(true),
//             },
//         )
//         .await;
//     assert!(profile_result.is_ok());

//     // Add account with custom host using update_profile
//     let custom_host = "git.example.com";
//     let update_result = app
//         .update_profile(
//             &ctx,
//             &app_delegate,
//             UpdateProfileInput {
//                 accounts_to_add: vec![AddAccountParams {
//                     host: custom_host.to_string(),
//                     kind: AccountKind::GitLab,
//                     pat: None,
//                 }],
//                 accounts_to_remove: vec![],
//                 accounts_to_update: vec![],
//             },
//         )
//         .await;

//     assert!(update_result.is_ok());
//     let update_output = update_result.unwrap();
//     assert_eq!(update_output.added_accounts.len(), 1);
//     assert_eq!(update_output.removed_accounts.len(), 0);

//     let account_id = &update_output.added_accounts[0];

//     // Verify account was added to active profile with custom host
//     let active_profile = app
//         .active_profile()
//         .await
//         .expect("active profile should exist");
//     let account = active_profile.account(account_id).await.unwrap().info();

//     assert_eq!(account.username, TEST_GITLAB_USERNAME);
//     assert_eq!(account.host, custom_host);
//     assert_eq!(account.kind, AccountKind::GitLab);

//     cleanup().await;
// }

// #[tokio::test]
// async fn add_account_pat() {
//     let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

//     // First create a profile
//     let profile_result = app
//         .create_profile(
//             &ctx,
//             &app_delegate,
//             CreateProfileInput {
//                 name: "Test Profile".to_string(),
//                 is_default: Some(true),
//             },
//         )
//         .await;
//     assert!(profile_result.is_ok());

//     // Add account with PAT
//     let host = "github.com";
//     let update_result = app
//         .update_profile(
//             &ctx,
//             &app_delegate,
//             UpdateProfileInput {
//                 accounts_to_add: vec![AddAccountParams {
//                     host: host.to_string(),
//                     kind: AccountKind::GitHub,
//                     pat: Some("Test PAT".to_string()),
//                 }],
//                 accounts_to_remove: vec![],
//                 accounts_to_update: vec![],
//             },
//         )
//         .await;

//     assert!(update_result.is_ok());
//     let update_output = update_result.unwrap();
//     assert_eq!(update_output.added_accounts.len(), 1);
//     assert_eq!(update_output.removed_accounts.len(), 0);

//     let account_id = &update_output.added_accounts[0];

//     // Verify account was added to active profile with custom host
//     let active_profile = app
//         .active_profile()
//         .await
//         .expect("active profile should exist");
//     let account = active_profile.account(account_id).await.unwrap().info();

//     assert_eq!(account.username, TEST_GITHUB_USERNAME);
//     assert_eq!(account.host, host);
//     assert_eq!(account.kind, AccountKind::GitHub);

//     cleanup().await;
// }

// #[tokio::test]
// async fn add_multiple_accounts() {
//     let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

//     // First create a profile
//     let profile_result = app
//         .create_profile(
//             &ctx,
//             &app_delegate,
//             CreateProfileInput {
//                 name: "Test Profile".to_string(),
//                 is_default: Some(true),
//             },
//         )
//         .await;
//     assert!(profile_result.is_ok());

//     // Add both GitHub and GitLab accounts in one operation
//     let update_result = app
//         .update_profile(
//             &ctx,
//             &app_delegate,
//             UpdateProfileInput {
//                 accounts_to_add: vec![
//                     AddAccountParams {
//                         host: "github.com".to_string(),
//                         kind: AccountKind::GitHub,
//                         pat: None,
//                     },
//                     AddAccountParams {
//                         host: "gitlab.com".to_string(),
//                         kind: AccountKind::GitLab,
//                         pat: None,
//                     },
//                 ],
//                 accounts_to_remove: vec![],
//                 accounts_to_update: vec![],
//             },
//         )
//         .await;

//     assert!(update_result.is_ok());
//     let update_output = update_result.unwrap();
//     assert_eq!(update_output.added_accounts.len(), 2);
//     assert_eq!(update_output.removed_accounts.len(), 0);

//     let github_account_id = &update_output.added_accounts[0];
//     let gitlab_account_id = &update_output.added_accounts[1];

//     // Verify both accounts have different IDs
//     assert_ne!(github_account_id, gitlab_account_id);

//     // Verify both accounts are in the active profile
//     let active_profile = app
//         .active_profile()
//         .await
//         .expect("active profile should exist");

//     let github_profile_account = active_profile
//         .account(github_account_id)
//         .await
//         .unwrap()
//         .info();
//     assert_eq!(github_profile_account.username, TEST_GITHUB_USERNAME);
//     assert_eq!(github_profile_account.host, "github.com");
//     assert_eq!(github_profile_account.kind, AccountKind::GitHub);

//     let gitlab_profile_account = active_profile
//         .account(gitlab_account_id)
//         .await
//         .unwrap()
//         .info();
//     assert_eq!(gitlab_profile_account.username, TEST_GITLAB_USERNAME);
//     assert_eq!(gitlab_profile_account.host, "gitlab.com");
//     assert_eq!(gitlab_profile_account.kind, AccountKind::GitLab);

//     cleanup().await;
// }

// #[tokio::test]
// async fn add_duplicate_account_fails() {
//     let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

//     // First create a profile
//     let profile_result = app
//         .create_profile(
//             &ctx,
//             &app_delegate,
//             CreateProfileInput {
//                 name: "Test Profile".to_string(),
//                 is_default: Some(true),
//             },
//         )
//         .await;
//     assert!(profile_result.is_ok());

//     // Add first GitHub account
//     let first_update = app
//         .update_profile(
//             &ctx,
//             &app_delegate,
//             UpdateProfileInput {
//                 accounts_to_add: vec![AddAccountParams {
//                     host: "github.com".to_string(),
//                     kind: AccountKind::GitHub,
//                     pat: None,
//                 }],
//                 accounts_to_remove: vec![],
//                 accounts_to_update: vec![],
//             },
//         )
//         .await;
//     assert!(first_update.is_ok());

//     // Try to add duplicate GitHub account - should fail
//     let duplicate_update = app
//         .update_profile(
//             &ctx,
//             &app_delegate,
//             UpdateProfileInput {
//                 accounts_to_add: vec![AddAccountParams {
//                     host: "github.com".to_string(),
//                     kind: AccountKind::GitHub,
//                     pat: None,
//                 }],
//                 accounts_to_remove: vec![],
//                 accounts_to_update: vec![],
//             },
//         )
//         .await;
//     assert!(duplicate_update.is_err());

//     cleanup().await;
// }

// #[tokio::test]
// async fn remove_account_success() {
//     let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

//     // First create a profile
//     let profile_result = app
//         .create_profile(
//             &ctx,
//             &app_delegate,
//             CreateProfileInput {
//                 name: "Test Profile".to_string(),
//                 is_default: Some(true),
//             },
//         )
//         .await;
//     assert!(profile_result.is_ok());

//     // Add GitHub account
//     let add_result = app
//         .update_profile(
//             &ctx,
//             &app_delegate,
//             UpdateProfileInput {
//                 accounts_to_add: vec![AddAccountParams {
//                     host: "github.com".to_string(),
//                     kind: AccountKind::GitHub,
//                     pat: None,
//                 }],
//                 accounts_to_remove: vec![],
//                 accounts_to_update: vec![],
//             },
//         )
//         .await;
//     assert!(add_result.is_ok());
//     let account_id = add_result.unwrap().added_accounts[0].clone();

//     // Verify account exists in profile
//     let active_profile = app
//         .active_profile()
//         .await
//         .expect("active profile should exist");
//     assert!(active_profile.account(&account_id).await.is_some());

//     // Remove the account
//     let remove_result = app
//         .update_profile(
//             &ctx,
//             &app_delegate,
//             UpdateProfileInput {
//                 accounts_to_add: vec![],
//                 accounts_to_remove: vec![account_id.clone()],
//                 accounts_to_update: vec![],
//             },
//         )
//         .await;
//     assert!(remove_result.is_ok());
//     let remove_output = remove_result.unwrap();
//     assert_eq!(remove_output.added_accounts.len(), 0);
//     assert_eq!(remove_output.removed_accounts.len(), 1);
//     assert_eq!(remove_output.removed_accounts[0], account_id);

//     // Verify account was removed from profile
//     let active_profile_after = app
//         .active_profile()
//         .await
//         .expect("active profile should exist");
//     assert!(active_profile_after.account(&account_id).await.is_none());

//     cleanup().await;
// }

// #[tokio::test]
// async fn remove_multiple_accounts() {
//     let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

//     // First create a profile
//     let profile_result = app
//         .create_profile(
//             &ctx,
//             &app_delegate,
//             CreateProfileInput {
//                 name: "Test Profile".to_string(),
//                 is_default: Some(true),
//             },
//         )
//         .await;
//     assert!(profile_result.is_ok());

//     // Add both accounts
//     let add_result = app
//         .update_profile(
//             &ctx,
//             &app_delegate,
//             UpdateProfileInput {
//                 accounts_to_add: vec![
//                     AddAccountParams {
//                         host: "github.com".to_string(),
//                         kind: AccountKind::GitHub,
//                         pat: None,
//                     },
//                     AddAccountParams {
//                         host: "gitlab.com".to_string(),
//                         kind: AccountKind::GitLab,
//                         pat: None,
//                     },
//                 ],
//                 accounts_to_remove: vec![],
//                 accounts_to_update: vec![],
//             },
//         )
//         .await;
//     assert!(add_result.is_ok());
//     let add_output = add_result.unwrap();
//     let github_account_id = add_output.added_accounts[0].clone();
//     let gitlab_account_id = add_output.added_accounts[1].clone();

//     // Verify both accounts exist
//     let active_profile = app
//         .active_profile()
//         .await
//         .expect("active profile should exist");
//     assert!(active_profile.account(&github_account_id).await.is_some());
//     assert!(active_profile.account(&gitlab_account_id).await.is_some());

//     // Remove GitHub account first
//     let remove_github = app
//         .update_profile(
//             &ctx,
//             &app_delegate,
//             UpdateProfileInput {
//                 accounts_to_add: vec![],
//                 accounts_to_remove: vec![github_account_id.clone()],
//                 accounts_to_update: vec![],
//             },
//         )
//         .await;
//     assert!(remove_github.is_ok());

//     // Verify GitHub account was removed but GitLab account still exists
//     let active_profile_after = app
//         .active_profile()
//         .await
//         .expect("active profile should exist");
//     assert!(
//         active_profile_after
//             .account(&github_account_id)
//             .await
//             .is_none()
//     );
//     assert!(
//         active_profile_after
//             .account(&gitlab_account_id)
//             .await
//             .is_some()
//     );

//     let gitlab_account = active_profile_after
//         .account(&gitlab_account_id)
//         .await
//         .unwrap()
//         .info();
//     assert_eq!(gitlab_account.username, TEST_GITLAB_USERNAME);
//     assert_eq!(gitlab_account.kind, AccountKind::GitLab);

//     // Remove GitLab account
//     let remove_gitlab = app
//         .update_profile(
//             &ctx,
//             &app_delegate,
//             UpdateProfileInput {
//                 accounts_to_add: vec![],
//                 accounts_to_remove: vec![gitlab_account_id.clone()],
//                 accounts_to_update: vec![],
//             },
//         )
//         .await;
//     assert!(remove_gitlab.is_ok());

//     // Verify both accounts are now removed
//     let final_profile = app
//         .active_profile()
//         .await
//         .expect("active profile should exist");
//     assert!(final_profile.account(&github_account_id).await.is_none());
//     assert!(final_profile.account(&gitlab_account_id).await.is_none());

//     cleanup().await;
// }

// #[tokio::test]
// async fn remove_nonexistent_account_succeeds() {
//     let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;
//     app.on_window_ready(
//         &ctx,
//         &app_delegate,
//         OnWindowReadyOptions {
//             restore_last_workspace: false,
//         },
//     )
//     .await
//     .unwrap();

//     // First create a profile
//     let profile_result = app
//         .create_profile(
//             &ctx,
//             &app_delegate,
//             CreateProfileInput {
//                 name: "Test Profile".to_string(),
//                 is_default: Some(true),
//             },
//         )
//         .await;
//     assert!(profile_result.is_ok());

//     // Try to remove a non-existent account
//     use moss_user::models::primitives::AccountId;
//     let fake_account_id = AccountId::new();

//     let remove_result = app
//         .update_profile(
//             &ctx,
//             &app_delegate,
//             UpdateProfileInput {
//                 accounts_to_add: vec![],
//                 accounts_to_remove: vec![fake_account_id.clone()],
//                 accounts_to_update: vec![],
//             },
//         )
//         .await;

//     // This should succeed (idempotent operation)
//     assert!(remove_result.is_ok());
//     let remove_output = remove_result.unwrap();
//     assert_eq!(remove_output.added_accounts.len(), 0);
//     assert_eq!(remove_output.removed_accounts.len(), 1);
//     assert_eq!(remove_output.removed_accounts[0], fake_account_id);

//     cleanup().await;
// }

// #[tokio::test]
// async fn add_and_remove_accounts_simultaneously() {
//     let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;
//     app.on_window_ready(
//         &ctx,
//         &app_delegate,
//         OnWindowReadyOptions {
//             restore_last_workspace: false,
//         },
//     )
//     .await
//     .unwrap();

//     // First create a profile
//     let profile_result = app
//         .create_profile(
//             &ctx,
//             &app_delegate,
//             CreateProfileInput {
//                 name: "Test Profile".to_string(),
//                 is_default: Some(true),
//             },
//         )
//         .await;
//     assert!(profile_result.is_ok());

//     // Add initial GitHub account
//     let initial_add = app
//         .update_profile(
//             &ctx,
//             &app_delegate,
//             UpdateProfileInput {
//                 accounts_to_add: vec![AddAccountParams {
//                     host: "github.com".to_string(),
//                     kind: AccountKind::GitHub,
//                     pat: None,
//                 }],
//                 accounts_to_remove: vec![],
//                 accounts_to_update: vec![],
//             },
//         )
//         .await;
//     assert!(initial_add.is_ok());
//     let github_account_id = initial_add.unwrap().added_accounts[0].clone();

//     // Simultaneously add GitLab account and remove GitHub account
//     let update_result = app
//         .update_profile(
//             &ctx,
//             &app_delegate,
//             UpdateProfileInput {
//                 accounts_to_add: vec![AddAccountParams {
//                     host: "gitlab.com".to_string(),
//                     kind: AccountKind::GitLab,
//                     pat: None,
//                 }],
//                 accounts_to_remove: vec![github_account_id.clone()],
//                 accounts_to_update: vec![],
//             },
//         )
//         .await;

//     assert!(update_result.is_ok());
//     let update_output = update_result.unwrap();
//     assert_eq!(update_output.added_accounts.len(), 1);
//     assert_eq!(update_output.removed_accounts.len(), 1);
//     assert_eq!(update_output.removed_accounts[0], github_account_id);

//     let gitlab_account_id = update_output.added_accounts[0].clone();

//     // Verify final state: GitHub removed, GitLab added
//     let final_profile = app
//         .active_profile()
//         .await
//         .expect("active profile should exist");
//     assert!(final_profile.account(&github_account_id).await.is_none());
//     assert!(final_profile.account(&gitlab_account_id).await.is_some());

//     let gitlab_account = final_profile
//         .account(&gitlab_account_id)
//         .await
//         .unwrap()
//         .info();
//     assert_eq!(gitlab_account.username, TEST_GITLAB_USERNAME);
//     assert_eq!(gitlab_account.kind, AccountKind::GitLab);

//     cleanup().await;
// }

// #[tokio::test]
// async fn empty_update_profile_succeeds() {
//     let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

//     // First create a profile
//     let profile_result = app
//         .create_profile(
//             &ctx,
//             &app_delegate,
//             CreateProfileInput {
//                 name: "Test Profile".to_string(),
//                 is_default: Some(true),
//             },
//         )
//         .await;
//     assert!(profile_result.is_ok());

//     // Empty update (no accounts to add or remove)
//     let update_result = app
//         .update_profile(
//             &ctx,
//             &app_delegate,
//             UpdateProfileInput {
//                 accounts_to_add: vec![],
//                 accounts_to_remove: vec![],
//                 accounts_to_update: vec![],
//             },
//         )
//         .await;

//     assert!(update_result.is_ok());
//     let update_output = update_result.unwrap();
//     assert_eq!(update_output.added_accounts.len(), 0);
//     assert_eq!(update_output.removed_accounts.len(), 0);

//     cleanup().await;
// }
