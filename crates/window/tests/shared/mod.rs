// use moss_app_delegate::AppDelegate;
// use moss_applib::{
//     context::{AsyncContext, MutableContext},
//     mock::MockAppRuntime,
// };
// use moss_configuration::registry::{AppConfigurationRegistry, ConfigurationRegistry};
// use moss_fs::RealFileSystem;
// use moss_git_hosting_provider::{
//     github::{
//         auth::{GitHubAuthAdapter, test::MockGitHubAuthAdapter},
//         client::{GitHubApiClient, test::MockGitHubApiClient},
//     },
//     gitlab::{
//         auth::{GitLabAuthAdapter, test::MockGitLabAuthAdapter},
//         client::{GitLabApiClient, test::MockGitLabApiClient},
//     },
// };
// use moss_keyring::test::MockKeyringClient;
// use moss_language::registry::{AppLanguageRegistry, LanguageRegistry};
// use moss_server_api::account_auth_gateway::AccountAuthGatewayApiClient;
// use moss_storage2::{AppStorage, AppStorageOptions, Storage};
// use moss_testutils::random_name::random_string;
// use moss_theme::registry::{AppThemeRegistry, ThemeRegistry};
// use reqwest::ClientBuilder as HttpClientBuilder;
// use std::{future::Future, path::PathBuf, pin::Pin, sync::Arc, time::Duration};
// use tauri::Manager;
// use window::{Window, WindowBuilder, window::OnWindowReadyOptions};

// pub type CleanupFn = Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>;

// const PROFILES: &str = r#"
// [
//   {
//     "id": "e_MChWGYcY",
//     "name": "Default",
//     "accounts": [],
//     "is_default": true
//   }
// ]
// "#;

// const ACCOUNT_AUTH_BASE_URL: &str = "https://account-auth-gateway-dev.20g10z3r.workers.dev";

// pub const TEST_GITHUB_USERNAME: &str = "test_login";
// pub const TEST_GITHUB_EMAIL: &str = "test_email@example.com";
// pub const TEST_GITLAB_USERNAME: &str = "test_username";
// pub const TEST_GITLAB_EMAIL: &str = "test_email@example.com";

// pub fn random_test_dir_path() -> PathBuf {
//     PathBuf::from(env!("CARGO_MANIFEST_DIR"))
//         .join("tests")
//         .join("data")
//         .join(random_string(10))
// }

// fn mock_github_api_client() -> Arc<MockGitHubApiClient> {
//     MockGitHubApiClient {
//         get_user_response: moss_git_hosting_provider::github::response::GetUserResponse {
//             id: 1,
//             login: TEST_GITHUB_USERNAME.to_string(),
//             email: Some(TEST_GITHUB_EMAIL.to_string()),
//         },
//         get_contributors_response:
//             moss_git_hosting_provider::github::response::GetContributorsResponse { items: vec![] },
//         get_repository_response:
//             moss_git_hosting_provider::github::response::GetRepositoryResponse {
//                 owner: moss_git_hosting_provider::github::response::Owner {
//                     login: TEST_GITHUB_USERNAME.to_string(),
//                 },
//                 updated_at: "test_updated_at".to_string(),
//             },
//         get_pat_expires_at_response: None,
//     }
//     .into()
// }

// fn mock_github_auth_adapter() -> Arc<MockGitHubAuthAdapter> {
//     MockGitHubAuthAdapter {
//         pkce_token_credentials:
//             moss_git_hosting_provider::github::auth::GitHubPkceTokenCredentials {
//                 access_token: "test_access_token".to_string(),
//             },
//         pat_token_credentials: (),
//     }
//     .into()
// }

// fn mock_gitlab_api_client() -> Arc<MockGitLabApiClient> {
//     MockGitLabApiClient {
//         get_user_response: moss_git_hosting_provider::gitlab::response::GetUserResponse {
//             username: TEST_GITLAB_USERNAME.to_string(),
//             commit_email: TEST_GITLAB_EMAIL.to_string(),
//         },
//         get_contributors_response:
//             moss_git_hosting_provider::gitlab::response::GetContributorsResponse { items: vec![] },
//         get_repository_response:
//             moss_git_hosting_provider::gitlab::response::GetRepositoryResponse {
//                 owner: moss_git_hosting_provider::gitlab::response::Owner {
//                     username: TEST_GITLAB_USERNAME.to_string(),
//                 },
//                 updated_at: "test_updated_at".to_string(),
//             },
//         get_pat_expires_at_response: None,
//     }
//     .into()
// }

// fn mock_gitlab_auth_adapter() -> Arc<MockGitLabAuthAdapter> {
//     MockGitLabAuthAdapter {
//         pkce_token_credentials:
//             moss_git_hosting_provider::gitlab::auth::GitLabPkceTokenCredentials {
//                 access_token: "test_access_token".to_string(),
//                 refresh_token: "test_refresh_token".to_string(),
//                 expires_in: 3600,
//             },
//         pat_token_credentials: (),
//     }
//     .into()
// }

// pub async fn set_up_test_app() -> (
//     Window<MockAppRuntime>,
//     AppDelegate<MockAppRuntime>,
//     AsyncContext,
//     CleanupFn,
// ) {
//     let ctx = MutableContext::background_with_timeout(Duration::from_secs(30)).freeze();

//     let keyring = Arc::new(MockKeyringClient::new());
//     let tauri_app = tauri::test::mock_app();
//     let tao_app_handle = tauri_app.handle().to_owned();
//     let http_client = HttpClientBuilder::new()
//         .user_agent("SAPIC-TEST/1.0")
//         .build()
//         .expect("failed to build http client");
//     let auth_api_client = Arc::new(AccountAuthGatewayApiClient::new(
//         http_client.clone(),
//         ACCOUNT_AUTH_BASE_URL.to_string(),
//     ));

//     let test_dir_path = random_test_dir_path();
//     let resource_path = test_dir_path.join("resources");
//     let user_path = test_dir_path.join("user");

//     let app_delegate = {
//         let delegate = AppDelegate::<MockAppRuntime>::new(tao_app_handle.clone());
//         delegate.set_resource_dir(resource_path.clone());
//         delegate.set_user_dir(user_path.clone());

//         <dyn Storage>::set_global(
//             &delegate,
//             AppStorage::new(
//                 &delegate.globals_dir(),
//                 delegate.workspaces_dir(),
//                 AppStorageOptions {
//                     in_memory: Some(false),
//                     busy_timeout: Some(Duration::from_secs(5)),
//                 }
//                 .into(),
//             )
//             .await
//             .unwrap(),
//         );
//         <dyn GitHubAuthAdapter<MockAppRuntime>>::set_global(&delegate, mock_github_auth_adapter());
//         <dyn GitHubApiClient<MockAppRuntime>>::set_global(&delegate, mock_github_api_client());
//         <dyn GitLabAuthAdapter<MockAppRuntime>>::set_global(&delegate, mock_gitlab_auth_adapter());
//         <dyn GitLabApiClient<MockAppRuntime>>::set_global(&delegate, mock_gitlab_api_client());
//         <dyn ConfigurationRegistry>::set_global(
//             &delegate,
//             AppConfigurationRegistry::new().unwrap(), // TODO: probably should mock this
//         );
//         <dyn ThemeRegistry>::set_global(&delegate, AppThemeRegistry::new()); // TODO: probably should mock this
//         <dyn LanguageRegistry>::set_global(&delegate, AppLanguageRegistry::new());

//         delegate
//     };

//     {
//         tao_app_handle.manage(app_delegate.clone());
//     }

//     let workspaces_abs_path = user_path.join("workspaces");
//     let globals_abs_path = user_path.join("globals");
//     let profiles_abs_path = user_path.join("profiles");

//     {
//         tokio::fs::create_dir_all(&resource_path.join("extensions"))
//             .await
//             .unwrap();
//         tokio::fs::create_dir_all(&user_path.join("extensions"))
//             .await
//             .unwrap();

//         tokio::fs::create_dir(&workspaces_abs_path).await.unwrap();
//         tokio::fs::create_dir(&globals_abs_path).await.unwrap();
//         tokio::fs::create_dir(&profiles_abs_path).await.unwrap();

//         tokio::fs::write(&profiles_abs_path.join("profiles.json"), PROFILES)
//             .await
//             .unwrap();
//     }
//     let fs = Arc::new(RealFileSystem::new(&app_delegate.tmp_dir()));

//     let cleanup_fn = Box::new({
//         let path = test_dir_path.clone();
//         move || {
//             Box::pin(async move {
//                 if let Err(e) = tokio::fs::remove_dir_all(&path).await {
//                     eprintln!("Failed to clean up test directory: {}", e);
//                 }
//             }) as Pin<Box<dyn Future<Output = ()> + Send>>
//         }
//     });

//     let app = WindowBuilder::new(fs.clone(), keyring, auth_api_client, vec![])
//         .build(&ctx, &app_delegate)
//         .await;

//     app.on_window_ready(
//         &ctx,
//         &app_delegate,
//         OnWindowReadyOptions {
//             restore_last_workspace: false,
//         },
//     )
//     .await
//     .unwrap();

//     (app, app_delegate, ctx, cleanup_fn)
// }
