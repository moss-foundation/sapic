mod edit;

use joinerror::{OptionExt, ResultExt};
use json_patch::{PatchOperation, ReplaceOperation, jsonptr::PointerBuf};
use moss_app_delegate::AppDelegate;
use moss_applib::{
    AppRuntime,
    errors::{FailedPrecondition, Internal},
    subscription::{Event, EventEmitter, Subscription},
};
use moss_configuration::models::primitives::ConfigurationTarget;
use moss_edit::json::EditOptions;
use moss_fs::{CreateOptions, FileSystem, FsResultExt};
use moss_logging::session;
use moss_text::ReadOnlyStr;
use rustc_hash::{FxHashMap, FxHashSet};
use sapic_runtime::{
    app::settings_storage::SettingScope,
    globals::{GlobalConfigurationRegistry, GlobalSettingsStorage},
};
use sapic_system::configuration::configuration_registry::ConfigurationRegistry;
use serde_json::Value as JsonValue;
use std::{
    collections::{HashMap, HashSet},
    path::Path,
    sync::Arc,
};
use tokio::sync::RwLock;

use crate::{
    configuration::edit::ConfigurationEdit,
    dirs,
    internal::events::{OnDidChangeConfiguration, OnDidChangeProfile, OnDidChangeWorkspace},
    profile::PROFILE_SETTINGS_FILE,
};

#[derive(Clone)]
pub struct ConfigurationModelOld {
    /// A set of all keys present in this object.
    pub keys: HashSet<ReadOnlyStr>,
    /// A JSON object with string keys, where the values are specific settings.
    pub contents: HashMap<ReadOnlyStr, JsonValue>,
}

impl ConfigurationModelOld {
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

struct ConfigurationHandle {
    fs: Arc<dyn FileSystem>,
    edit: ConfigurationEdit,
    model: ConfigurationModelOld,
}

impl ConfigurationHandle {
    fn new(fs: Arc<dyn FileSystem>, source: Arc<Path>) -> Self {
        Self {
            fs: fs.clone(),
            edit: ConfigurationEdit::new(fs, source),
            model: ConfigurationModelOld {
                keys: HashSet::new(),
                contents: HashMap::new(),
            },
        }
    }

    async fn load(fs: Arc<dyn FileSystem>, source: Arc<Path>) -> joinerror::Result<Self> {
        let parsed = Self::load_internal(fs.as_ref(), &source).await?;
        Ok(ConfigurationHandle {
            fs: fs.clone(),
            edit: ConfigurationEdit::new(fs, source),
            model: ConfigurationModelOld {
                keys: parsed.keys().map(|key| key.clone()).collect(),
                contents: parsed,
            },
        })
    }

    async fn reload(&mut self) -> joinerror::Result<()> {
        let parsed = Self::load_internal(self.fs.as_ref(), self.edit.abs_path()).await?;
        self.model = ConfigurationModelOld {
            keys: parsed.keys().map(|key| key.clone()).collect(),
            contents: parsed,
        };

        Ok(())
    }

    async fn load_internal(
        fs: &dyn FileSystem,
        source: &Path,
    ) -> joinerror::Result<HashMap<ReadOnlyStr, JsonValue>> {
        let rdr = fs.open_file(&source).await.join_err_with::<()>(|| {
            format!("failed to open profile settings file: {}", source.display())
        })?;
        let parsed: HashMap<ReadOnlyStr, JsonValue> = serde_json::from_reader(rdr)
            .join_err_with::<()>(|| {
                format!(
                    "failed to parse profile settings file: {}",
                    source.display()
                )
            })?;

        Ok(parsed)
    }

    async fn update_value(&self, key: &str, value: JsonValue) -> joinerror::Result<()> {
        if !self.edit.abs_path().exists() {
            let parent = self.edit.abs_path().parent().unwrap();
            self.fs.create_dir_all(parent).await?;
            self.fs
                .create_file_with(
                    self.edit.abs_path(),
                    b"{}",
                    CreateOptions {
                        overwrite: true,
                        ignore_if_exists: false,
                    },
                )
                .await?;
        }

        self.edit
            .edit(&[(
                PatchOperation::Replace(ReplaceOperation {
                    path: unsafe { PointerBuf::new_unchecked(format!("/{}", key)) },
                    value: value.clone(),
                }),
                EditOptions {
                    ignore_if_not_exists: false,
                    create_missing_segments: true,
                },
            )])
            .await
            .join_err::<Internal>("failed to edit settings file")?;

        Ok(())
    }
}

pub(crate) struct ConfigurationServiceOld {
    registry: Arc<dyn ConfigurationRegistry>,
    defaults: ConfigurationModelOld,
    profile: Arc<RwLock<Option<ConfigurationHandle>>>,
    workspace: Arc<RwLock<Option<ConfigurationHandle>>>,

    /// Since the concept of configuration is an aggregated one and is built
    /// from multiple sources, when tracking changes we need to track not only
    /// what changes were applied, but also which configuration sources they were
    /// applied to, in order to correctly roll them back if needed.
    #[allow(unused)]
    edits: Vec<ConfigurationTarget>,

    on_did_change_configuration_emitter: EventEmitter<OnDidChangeConfiguration>,

    _on_did_change_profile: Subscription<OnDidChangeProfile>,
    _on_did_change_workspace: Subscription<OnDidChangeWorkspace>,
}

impl ConfigurationServiceOld {
    pub async fn new<R: AppRuntime>(
        app_delegate: &AppDelegate<R>,
        fs: Arc<dyn FileSystem>,

        on_did_change_configuration_emitter: EventEmitter<OnDidChangeConfiguration>,

        on_did_change_profile_event: &Event<OnDidChangeProfile>,
        on_did_change_workspace_event: &Event<OnDidChangeWorkspace>,
    ) -> joinerror::Result<Self> {
        let registry = GlobalConfigurationRegistry::get(app_delegate);
        let defaults = registry.defaults();

        let profile = Arc::new(RwLock::new(None));
        let workspace = Arc::new(RwLock::new(None));

        let user_dir = app_delegate.user_dir();
        let profile_dir = user_dir.join(dirs::PROFILES_DIR);
        let _workspace_dir = user_dir.join(dirs::WORKSPACES_DIR);

        let fs_clone = fs.clone();
        let profile_clone = profile.clone();

        Ok(Self {
            registry,
            defaults: ConfigurationModelOld {
                keys: defaults.keys().map(|key| key.clone()).collect(),
                contents: defaults,
            },
            profile,
            workspace,
            edits: vec![],

            on_did_change_configuration_emitter,

            _on_did_change_profile: on_did_change_profile_event
                .subscribe(move |event| {
                    let source: Arc<Path> = profile_dir
                        .join(event.id.to_string())
                        .join(PROFILE_SETTINGS_FILE)
                        .into();
                    let fs = fs_clone.clone();
                    let profile = profile_clone.clone();

                    async move {
                        if !source.exists() {
                            *profile.write().await = Some(ConfigurationHandle::new(fs, source));

                            return;
                        }

                        let handle = match ConfigurationHandle::load(fs, source).await {
                            Ok(handle) => handle,
                            Err(e) => {
                                session::error!(
                                    "failed to load profile settings file: {}",
                                    e.to_string()
                                );
                                return;
                            }
                        };

                        *profile.write().await = Some(handle);
                    }
                })
                .await,
            _on_did_change_workspace: on_did_change_workspace_event
                .subscribe(move |_event| async {})
                .await,
        })
    }

    pub async fn configuration<R: AppRuntime>(
        &self,
        delegate: &AppDelegate<R>,
    ) -> HashMap<String, JsonValue> {
        // HACK: A temporary solution to maintain backward compatibility

        let settings_storage = GlobalSettingsStorage::get(delegate);
        HashMap::from_iter(
            settings_storage
                .values(&SettingScope::User)
                .await
                .into_iter(),
        )
    }

    pub async fn update_value<R: AppRuntime>(
        &self,
        delegate: &AppDelegate<R>,
        key: &str,
        value: JsonValue,
        _target: ConfigurationTarget,
    ) -> joinerror::Result<()> {
        // HACK: A temporary solution to maintain backward compatibility

        // if !self.registry.is_parameter_known(key) {
        //     session::warn!("parameter '{}' is unknown", key);
        // } else {
        //     self.registry.validate_parameter(key, &value)?;
        // }

        // match target {
        //     ConfigurationTarget::Profile => {
        //         let mut handle_lock = self.profile.write().await;
        //         let handle = handle_lock
        //             .as_mut()
        //             .ok_or_join_err::<FailedPrecondition>("no profile configuration handle")?;

        //         handle.update_value(key, value.clone()).await?;
        //         handle.reload().await?;
        //     }
        //     ConfigurationTarget::Workspace => {
        //         unimplemented!()
        //     }
        // }

        // self.on_did_change_configuration_emitter
        //     .fire(OnDidChangeConfiguration {
        //         affected_keys: FxHashSet::from_iter([key.to_string()]),
        //         changes: FxHashMap::from_iter([(key.to_string(), value)]),
        //     })
        //     .await;

        // Ok(())

        let settings_storage = GlobalSettingsStorage::get(delegate);
        settings_storage
            .update_value(&SettingScope::User, key, value)
            .await?;

        Ok(())
    }
}
