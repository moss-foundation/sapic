use moss_app_delegate::AppDelegate;
use moss_applib::{
    AppRuntime,
    subscription::{Event, Subscription},
};
use moss_fs::FileSystem;
use moss_logging::session;
use moss_text::{ReadOnlyStr, read_only_str};
use serde_json::Value as JsonValue;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use tokio::sync::RwLock;

use crate::{OnDidChangeProfile, OnDidChangeWorkspace, dirs, profile::PROFILE_SETTINGS_FILE};

#[derive(Clone)]
pub struct ConfigurationModel {
    /// A set of all keys present in this object.
    pub keys: HashSet<ReadOnlyStr>,
    /// A JSON object with string keys, where the values are specific settings.
    pub contents: HashMap<ReadOnlyStr, JsonValue>,
}

impl ConfigurationModel {
    pub fn merge(&self, other: &Self) -> Self {
        let mut all_keys: HashSet<ReadOnlyStr> = self.keys.iter().cloned().collect();
        all_keys.extend(other.keys.iter().cloned());

        let mut merged_contents = self.contents.clone();
        for (k, v) in &other.contents {
            merged_contents.insert(k.clone(), v.clone());
        }

        Self {
            keys: all_keys.into_iter().collect(),
            contents: merged_contents,
        }
    }
}

pub struct ConfigurationService {
    defaults: ConfigurationModel,
    profile: Arc<RwLock<Option<ConfigurationModel>>>,
    workspace: Arc<RwLock<Option<ConfigurationModel>>>,

    _on_did_change_profile: Subscription<OnDidChangeProfile>,
    _on_did_change_workspace: Subscription<OnDidChangeWorkspace>,
}

impl ConfigurationService {
    pub async fn new<R: AppRuntime>(
        app_delegate: &AppDelegate<R>,
        fs: Arc<dyn FileSystem>,
        on_did_change_profile: &Event<OnDidChangeProfile>,
        on_did_change_workspace: &Event<OnDidChangeWorkspace>,
    ) -> Self {
        // HACK: hardcoded here for now
        let defaults = HashMap::from([
            (
                read_only_str!("colorTheme"),
                JsonValue::String("moss.sapic-theme.lightDefault".to_string()),
            ),
            (
                read_only_str!("locale"),
                JsonValue::String("moss.sapic-locale.en".to_string()),
            ),
        ]);

        let profile = Arc::new(RwLock::new(None));
        let workspace = Arc::new(RwLock::new(None));

        let app_dir = app_delegate.app_dir();
        let profile_dir = app_dir.join(dirs::PROFILES_DIR);
        let _workspace_dir = app_dir.join(dirs::WORKSPACES_DIR);

        let fs_clone = fs.clone();
        let profile_clone = profile.clone();

        Self {
            defaults: ConfigurationModel {
                keys: defaults.keys().map(|key| key.clone()).collect(),
                contents: defaults,
            },
            profile,
            workspace,
            _on_did_change_profile: on_did_change_profile
                .subscribe(move |event| {
                    let settings_path = profile_dir
                        .join(event.id.to_string())
                        .join(PROFILE_SETTINGS_FILE);
                    let fs = fs_clone.clone();
                    let profile = profile_clone.clone();

                    async move {
                        if !settings_path.exists() {
                            *profile.write().await = Some(ConfigurationModel {
                                keys: HashSet::new(),
                                contents: HashMap::new(),
                            });

                            return;
                        }

                        let rdr = match fs.open_file(&settings_path).await {
                            Ok(rdr) => rdr,
                            Err(e) => {
                                session::error!(
                                    "failed to open profile settings file: {}",
                                    e.to_string()
                                );
                                return;
                            }
                        };

                        let parsed: HashMap<ReadOnlyStr, JsonValue> =
                            match serde_json::from_reader(rdr) {
                                Ok(parsed) => parsed,
                                Err(e) => {
                                    session::error!(
                                        "failed to parse profile settings file: {}",
                                        e.to_string()
                                    );
                                    return;
                                }
                            };

                        *profile.write().await = Some(ConfigurationModel {
                            keys: parsed.keys().map(|key| key.clone()).collect(),
                            contents: parsed,
                        });
                    }
                })
                .await,
            _on_did_change_workspace: on_did_change_workspace
                .subscribe(move |_event| async {})
                .await,
        }
    }

    pub async fn configuration(&self) -> ConfigurationModel {
        let mut configuration = self.defaults.clone();

        if let Some(profile_conf) = &*self.profile.read().await {
            configuration = configuration.merge(profile_conf);
        }

        if let Some(workspace_conf) = &*self.workspace.read().await {
            configuration = configuration.merge(workspace_conf);
        }

        configuration
    }
}
