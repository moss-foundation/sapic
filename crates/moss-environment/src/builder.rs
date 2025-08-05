use crate::{
    configuration::{MetadataDecl, SourceFile},
    edit::EnvironmentEditing,
    environment::{Environment, EnvironmentPath},
    errors::{
        ErrorEnvironmentAlreadyExists, ErrorEnvironmentNotFound, ErrorFailedToEncode, ErrorIo,
    },
    models::primitives::EnvironmentId,
    utils,
};
use joinerror::{Error, ResultExt};
use moss_applib::AppRuntime;
use moss_fs::{CreateOptions, FileSystem, FsResultExt};
use moss_hcl::{Block, HclResultExt};
use moss_storage::common::VariableStore;
use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::watch;

pub struct CreateEnvironmentParams<'a> {
    pub name: String,
    pub abs_path: &'a Path,
    pub color: Option<String>,
    pub order: isize,
}

pub struct EnvironmentLoadParams {
    pub abs_path: PathBuf,
}

pub struct EnvironmentBuilder {
    fs: Arc<dyn FileSystem>,
}

impl EnvironmentBuilder {
    pub fn new(fs: Arc<dyn FileSystem>) -> Self {
        Self { fs }
    }

    pub async fn initialize<'a>(
        &self,
        params: CreateEnvironmentParams<'a>,
    ) -> joinerror::Result<PathBuf> {
        debug_assert!(params.abs_path.is_absolute());

        let id = EnvironmentId::new();
        let file_name = utils::format_file_name(&params.name);
        let abs_path = params.abs_path.join(&file_name);
        if abs_path.exists() {
            return Err(Error::new::<ErrorEnvironmentAlreadyExists>(
                abs_path.display().to_string(),
            ));
        }

        let content = hcl::to_string(&SourceFile {
            metadata: Block::new(MetadataDecl {
                id: id.clone(),
                color: params.color,
            }),
            variables: None,
        })
        .join_err_with::<ErrorFailedToEncode>(|| {
            format!("failed to encode environment file {}", abs_path.display())
        })?;

        self.fs
            .create_file_with(
                &abs_path,
                content.as_bytes(),
                CreateOptions {
                    overwrite: false,
                    ignore_if_exists: false,
                },
            )
            .await
            .join_err_with::<ErrorIo>(|| {
                format!("failed to create environment file {}", abs_path.display())
            })?;

        Ok(abs_path)
    }

    pub async fn create<'a, R: AppRuntime>(
        self,
        params: CreateEnvironmentParams<'a>,
        variable_store: Arc<dyn VariableStore<R::AsyncContext>>,
    ) -> joinerror::Result<Environment<R>> {
        debug_assert!(params.abs_path.is_absolute());

        let abs_path = self
            .initialize(params)
            .await
            .join_err::<()>("failed to initialize environment")?;

        let (abs_path_tx, abs_path_rx) = watch::channel(EnvironmentPath::new(abs_path)?);

        Ok(Environment {
            fs: self.fs.clone(),
            abs_path_rx,
            edit: EnvironmentEditing::new(self.fs.clone(), abs_path_tx),
            variable_store,
        })
    }

    pub async fn load<R: AppRuntime>(
        self,
        params: EnvironmentLoadParams,
        variable_store: Arc<dyn VariableStore<R::AsyncContext>>,
    ) -> joinerror::Result<Environment<R>> {
        debug_assert!(params.abs_path.is_absolute());

        if !params.abs_path.exists() {
            return Err(Error::new::<ErrorEnvironmentNotFound>(
                params.abs_path.display().to_string(),
            ));
        }

        let (abs_path_tx, abs_path_rx) = watch::channel(EnvironmentPath::new(params.abs_path)?);

        Ok(Environment {
            fs: self.fs.clone(),
            abs_path_rx,
            edit: EnvironmentEditing::new(self.fs.clone(), abs_path_tx),
            variable_store,
        })
    }
}
