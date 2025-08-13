#![cfg(feature = "integration-tests")]

use moss_applib::{
    context::{AsyncContext, MutableContext},
    mock::MockAppRuntime,
};
use moss_collection::{
    CollectionBuilder,
    builder::CollectionCreateParams,
    collection::Collection,
    dirs,
    models::{
        operations::{CreateDirEntryInput, CreateEntryInput, CreateItemEntryInput},
        primitives::EntryId,
        types::configuration::{
            ComponentDirConfigurationModel, ComponentItemConfigurationModel, DirConfigurationModel,
            DirHttpConfigurationModel, EndpointDirConfigurationModel, HttpEndpointDirConfiguration,
            ItemConfigurationModel, RequestDirConfigurationModel, SchemaDirConfigurationModel,
        },
    },
};
use moss_fs::RealFileSystem;
use moss_git_hosting_provider::{
    common::ssh_auth_agent::SSHAuthAgentImpl,
    github::{auth::GitHubAuthAgent, client::GitHubClient},
    gitlab::{auth::GitLabAuthAgent, client::GitLabClient},
};
use moss_keyring::KeyringClientImpl;
use moss_testutils::random_name::{random_collection_name, random_string};
use nanoid::nanoid;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

#[allow(dead_code)]
pub fn random_dir_name() -> String {
    format!("Test_{}_Dir", random_string(10))
}

pub fn random_entry_name() -> String {
    format!("Test_{}_Entry", random_string(10))
}

fn random_collection_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(nanoid!(10))
}

pub async fn create_test_collection() -> (
    AsyncContext, // TODO: this is temporary, should be a mock
    Arc<Path>,
    Collection<MockAppRuntime>,
) {
    let ctx = MutableContext::background_with_timeout(Duration::from_secs(30)).freeze();
    let fs = Arc::new(RealFileSystem::new());
    let internal_abs_path = random_collection_path();

    std::fs::create_dir_all(internal_abs_path.clone()).unwrap();

    let abs_path: Arc<Path> = internal_abs_path.clone().into();
    let reqwest_client = reqwest::Client::new();
    let keyring_client = Arc::new(KeyringClientImpl::new());

    // Collection operations shouldn't need any git operations
    let github_auth_agent = Arc::new(GitHubAuthAgent::new(
        keyring_client.clone(),
        "".to_string(),
        "".to_string(),
    ));

    let github_client = Arc::new(GitHubClient::new(
        reqwest_client.clone(),
        github_auth_agent,
        None as Option<SSHAuthAgentImpl>,
    ));

    let gitlab_auth_agent = Arc::new(GitLabAuthAgent::new(
        keyring_client.clone(),
        "".to_string(),
        "".to_string(),
    ));

    let gitlab_client = Arc::new(GitLabClient::new(
        reqwest_client.clone(),
        gitlab_auth_agent,
        None as Option<SSHAuthAgentImpl>,
    ));

    let collection = CollectionBuilder::new(fs, github_client, gitlab_client)
        .create(
            &ctx,
            CollectionCreateParams {
                name: Some(random_collection_name()),
                external_abs_path: None,
                internal_abs_path: abs_path.clone(),
                repository: None,
                git_provider_type: None,
                icon_path: None,
            },
        )
        .await
        .unwrap();

    (ctx, abs_path, collection)
}

// Since configuration models are empty enums, we need to use unreachable! for now
// This is a limitation of the current implementation
#[allow(dead_code)]
pub fn create_test_item_configuration() -> ItemConfigurationModel {
    // For now, we cannot create any variant since all configuration models are empty enums
    // This is a known issue in the codebase
    unreachable!("Configuration models are empty enums - cannot be instantiated")
}

#[allow(dead_code)]
pub fn create_test_request_dir_configuration() -> DirConfigurationModel {
    DirConfigurationModel::Request(RequestDirConfigurationModel::Http(
        DirHttpConfigurationModel {},
    ))
}

#[allow(dead_code)]
pub fn create_test_endpoint_dir_configuration() -> DirConfigurationModel {
    DirConfigurationModel::Endpoint(EndpointDirConfigurationModel::Http(
        HttpEndpointDirConfiguration {},
    ))
}

#[allow(dead_code)]
pub fn create_test_component_dir_configuration() -> DirConfigurationModel {
    DirConfigurationModel::Component(ComponentDirConfigurationModel {})
}

#[allow(dead_code)]
pub fn create_test_component_item_configuration() -> ItemConfigurationModel {
    ItemConfigurationModel::Component(ComponentItemConfigurationModel {})
}

#[allow(dead_code)]
pub fn create_test_schema_dir_configuration() -> DirConfigurationModel {
    DirConfigurationModel::Schema(SchemaDirConfigurationModel {})
}

#[allow(dead_code)]
pub async fn create_test_request_dir_entry(
    ctx: &AsyncContext,
    collection: &mut Collection<MockAppRuntime>,
    name: &str,
) -> EntryId {
    collection
        .create_entry(
            &ctx,
            CreateEntryInput::Dir(CreateDirEntryInput {
                path: PathBuf::from(dirs::REQUESTS_DIR),
                name: name.to_string(),
                order: 0,
                configuration: create_test_request_dir_configuration(),
            }),
        )
        .await
        .unwrap()
        .id
}

#[allow(dead_code)]
pub async fn create_test_endpoint_dir_entry(
    ctx: &AsyncContext,
    collection: &mut Collection<MockAppRuntime>,
    name: &str,
) -> EntryId {
    collection
        .create_entry(
            &ctx,
            CreateEntryInput::Dir(CreateDirEntryInput {
                path: PathBuf::from(dirs::ENDPOINTS_DIR),
                name: name.to_string(),
                order: 0,
                configuration: create_test_endpoint_dir_configuration(),
            }),
        )
        .await
        .unwrap()
        .id
}

#[allow(dead_code)]
pub async fn create_test_component_dir_entry(
    ctx: &AsyncContext,
    collection: &mut Collection<MockAppRuntime>,
    name: &str,
) -> EntryId {
    collection
        .create_entry(
            &ctx,
            CreateEntryInput::Dir(CreateDirEntryInput {
                path: PathBuf::from(dirs::COMPONENTS_DIR),
                name: name.to_string(),
                order: 0,
                configuration: create_test_component_dir_configuration(),
            }),
        )
        .await
        .unwrap()
        .id
}

#[allow(dead_code)]
pub async fn create_test_component_item_entry(
    ctx: &AsyncContext,
    collection: &mut Collection<MockAppRuntime>,
    name: &str,
) -> EntryId {
    collection
        .create_entry(
            &ctx,
            CreateEntryInput::Item(CreateItemEntryInput {
                path: PathBuf::from(dirs::COMPONENTS_DIR),
                name: name.to_string(),
                order: 0,
                configuration: create_test_component_item_configuration(),
            }),
        )
        .await
        .unwrap()
        .id
}

#[allow(dead_code)]
pub async fn create_test_schema_dir_entry(
    ctx: &AsyncContext,
    collection: &mut Collection<MockAppRuntime>,
    name: &str,
) -> EntryId {
    collection
        .create_entry(
            &ctx,
            CreateEntryInput::Dir(CreateDirEntryInput {
                path: PathBuf::from(dirs::SCHEMAS_DIR),
                name: name.to_string(),
                order: 0,
                configuration: create_test_schema_dir_configuration(),
            }),
        )
        .await
        .unwrap()
        .id
}
