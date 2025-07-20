use anyhow::Result;
use moss_applib::{AppRuntime, providers::ServiceProvider};
use moss_file::json::JsonFileHandle;
use moss_fs::FileSystem;
use std::{collections::HashMap, path::Path, sync::Arc};

// #[derive(Error, Debug)]
// pub enum EnvironmentError {
//     #[error("Failed to parse environment file as JSON: {0}")]
//     JsonParseError(#[from] serde_json::Error),

//     #[error("Failed to open environment file {path}: {err}")]
//     FileOpenError { err: anyhow::Error, path: PathBuf },

//     #[error("Failed to create environment file {path}: {err}")]
//     FileCreateError { err: anyhow::Error, path: PathBuf },

//     #[error("Failed to rename environment file {old_path} to {new_path}: {err}")]
//     FileRenameError {
//         old_path: PathBuf,
//         new_path: PathBuf,
//         err: anyhow::Error,
//     },

//     #[error("Unknown error: {0}")]
//     Unknown(anyhow::Error),
// }

// #[derive(Debug, Clone)]
// pub struct VariableItemParams {
//     pub disabled: bool,
// }

// VariableId: length-5?

// type VariableMap = HashMap<VariableName, VariableItem>;

pub struct Environment<R: AppRuntime> {
    // #[allow(dead_code)]
    // fs: Arc<dyn FileSystem>,
    abs_path: Arc<Path>,
    services: ServiceProvider,
    _marker: std::marker::PhantomData<R>,
    // variables: VariableMap,
    // #[allow(dead_code)]
    // store: Arc<dyn WorkspaceVariableStore<R::AsyncContext>>,
    // file: JsonFileHandle<FileModel>,
}

unsafe impl<R: AppRuntime> Send for Environment<R> {}
unsafe impl<R: AppRuntime> Sync for Environment<R> {}

// impl<R: AppRuntime> std::fmt::Debug for Environment<R> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("Environment")
//             .field("path", &self.abs_path)
//             .field("variables", &self.variables)
//             .finish()
//     }
// }

// pub struct LoadParams {
//     pub create_if_not_exists: bool,
// }

impl<R: AppRuntime> Environment<R> {
    pub(super) fn new(abs_path: Arc<Path>, services: ServiceProvider) -> Self {
        Self {
            abs_path,
            services,
            _marker: std::marker::PhantomData,
        }
    }
    // pub async fn load(
    //     abs_path: &Path,
    //     fs: Arc<dyn FileSystem>,
    //     store: Arc<dyn WorkspaceVariableStore<R::AsyncContext>>,
    //     params: LoadParams,
    // ) -> Result<Self> {
    //     let abs_path: Arc<Path> = abs_path.into();
    //     debug_assert!(abs_path.is_file());
    //     debug_assert!(abs_path.is_absolute());

    //     let file_handle = if abs_path.exists() {
    //         JsonFileHandle::load(Arc::clone(&fs), &abs_path).await?
    //     } else if !abs_path.exists() && params.create_if_not_exists {
    //         JsonFileHandle::create(Arc::clone(&fs), &abs_path, FileModel::new()).await?
    //     } else {
    //         return Err(anyhow::anyhow!(
    //             "Environment file {} is not found",
    //             abs_path.display()
    //         ));
    //     };

    //     if abs_path
    //         .extension()
    //         .map(|ext| ext != "json")
    //         .unwrap_or(false)
    //     {
    //         return Err(anyhow::anyhow!(
    //             "Environment file must have a .json extension"
    //         ));
    //     }

    //     let mut variables = HashMap::new();
    //     for (name, value) in file_handle.model().await.values {
    //         variables.insert(
    //             name,
    //             VariableItem {
    //                 id: VariableId::new(),
    //                 kind: value.kind,
    //                 global_value: value.value,
    //                 desc: value.desc,
    //                 params: VariableItemParams {
    //                     disabled: true, // TODO: restore this value from cache
    //                 },
    //             },
    //         );
    //     }

    //     Ok(Self {
    //         fs,
    //         abs_path,
    //         variables,
    //         store,
    //         file: file_handle,
    //     })
    // }

    // pub async fn id(&self) -> EnvironmentId {
    //     self.file.model().await.id.clone()
    // }

    // pub fn variables(&self) -> &VariableMap {
    //     &self.variables
    // }

    // pub async fn modify(&self, f: impl FnOnce(&mut FileModel) -> Result<()>) -> Result<()> {
    //     self.file
    //         .edit(f, |model| {
    //             serde_json::to_string(model)
    //                 .map_err(|err| anyhow::anyhow!("Failed to serialize environment file: {}", err))
    //         })
    //         .await?;

    //     Ok(())
    // }

    // pub async fn clear(&mut self) -> Result<()> {
    //     self.variables.clear();
    //     Ok(())
    // }
}
