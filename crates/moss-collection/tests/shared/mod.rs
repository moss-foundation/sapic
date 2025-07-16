use moss_applib::{
    context::{AsyncContext, MutableContext},
    mock::MockAppRuntime,
    providers::{ServiceMap, ServiceProvider},
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
    services::{
        DynStorageService, DynWorktreeService, storage_service::StorageService,
        worktree_service::WorktreeService,
    },
};
use moss_fs::RealFileSystem;
use moss_testutils::random_name::{random_collection_name, random_string};
use nanoid::nanoid;
use std::{
    any::TypeId,
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
    ServiceProvider,
) {
    let ctx = MutableContext::background_with_timeout(Duration::from_secs(30)).freeze();
    let fs = Arc::new(RealFileSystem::new());
    let internal_abs_path = random_collection_path();

    std::fs::create_dir_all(internal_abs_path.clone()).unwrap();

    let abs_path: Arc<Path> = internal_abs_path.clone().into();

    let mut services: ServiceMap = Default::default();

    let storage_service: Arc<StorageService<MockAppRuntime>> =
        StorageService::new(&abs_path).unwrap().into();
    let storage_service_dyn: Arc<DynStorageService<MockAppRuntime>> =
        DynStorageService::new(storage_service.clone()).into();

    let worktree_service: Arc<WorktreeService<MockAppRuntime>> =
        WorktreeService::new(abs_path.clone(), fs.clone(), storage_service_dyn.clone()).into();
    let worktree_service_dyn: Arc<DynWorktreeService<MockAppRuntime>> =
        DynWorktreeService::new(worktree_service.clone()).into();

    {
        services.insert(
            TypeId::of::<StorageService<MockAppRuntime>>(),
            storage_service,
        );
        services.insert(
            TypeId::of::<WorktreeService<MockAppRuntime>>(),
            worktree_service,
        );
    }

    let collection = CollectionBuilder::new(fs)
        .with_service::<DynStorageService<MockAppRuntime>>(storage_service_dyn)
        .with_service::<DynWorktreeService<MockAppRuntime>>(worktree_service_dyn)
        .create(
            &ctx,
            CollectionCreateParams {
                name: Some(random_collection_name()),
                external_abs_path: None,
                repository: None,
                internal_abs_path: abs_path.clone(),
                icon_path: None,
            },
        )
        .await
        .unwrap();

    (ctx, abs_path, collection, services.into())
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
