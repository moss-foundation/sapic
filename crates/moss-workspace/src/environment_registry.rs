// use std::{path::PathBuf, sync::Arc};

// use moss_applib::AppRuntime;
// use moss_environment_provider::EnvironmentProvider;
// use moss_fs::FileSystem;
// use rustc_hash::FxHashMap;
// use tokio::sync::RwLock;

// pub(crate) type ProviderId = String;

// pub(crate) struct EnvironmentProviderRegistry {
//     providers: FxHashMap<ProviderId, EnvironmentProvider>,
// }

// impl EnvironmentProviderRegistry {
//     pub fn new() -> Self {
//         Self {
//             providers: FxHashMap::default(),
//         }
//     }

//     pub fn register<R: AppRuntime>(&mut self, id: ProviderId, provider: EnvironmentProvider) {
//         self.providers.insert(id, provider);
//     }

//     // pub async fn get(&self, id: impl AsRef<ProviderId>) -> Option<&EnvironmentProvider> {
//     //     let providers = self.providers.read().await;
//     //     letproviders.get(id.as_ref())
//     // }
// }
