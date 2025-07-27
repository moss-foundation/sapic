use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use moss_applib::{
    AppRuntime, ServiceMarker,
    context::{AsyncContext, MutableContext},
    mock::MockAppRuntime,
    providers::{ServiceMap, ServiceProvider},
    subscription::EventEmitter,
};
use moss_environment::{environment::Environment, services::storage_service::StorageService};
use moss_fs::RealFileSystem;

// pub fn random_string(length: usize) -> String {
//     use rand::{Rng, distr::Alphanumeric};

//     rand::rng()
//         .sample_iter(Alphanumeric)
//         .take(length)
//         .map(char::from)
//         .collect()
// }

// pub fn test_environment_data() -> (PathBuf) {
//     let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
//         .join("tests")
//         .join("data");
//     let environment_name = format!("Test_{}_Environment.json", random_string(10));
//     let environment_file_path = base_path.join(environment_name.clone());

//     environment_file_path
// }

pub async fn create_test_environment() -> (
    AsyncContext,
    Arc<Path>,
    Environment<MockAppRuntime>,
    ServiceProvider,
) {
    let ctx = MutableContext::background_with_timeout(Duration::from_secs(30)).freeze();
    let fs = Arc::new(RealFileSystem::new());

    let mut services: ServiceMap = Default::default();

    todo!()
}
