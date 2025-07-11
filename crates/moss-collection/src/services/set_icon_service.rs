use anyhow::Result;
use async_trait::async_trait;
use image::{GenericImageView, imageops::FilterType};
use moss_applib::ServiceMarker;
use moss_fs::{FileSystem, RemoveOptions};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{
    dirs,
    services::{AnySetIconService, set_icon_service::constants::COLLECTION_ICON_FILENAME},
};

// FIXME: This either shouldn’t be public or should be moved to `lib.rs`.
pub mod constants {
    pub const COLLECTION_ICON_FILENAME: &str = "icon.png";
    pub const ICON_SIZE: u32 = 128;
}

pub struct SetIconService {
    fs: Arc<dyn FileSystem>,
    assets_abs_path: Arc<Path>,
    icon_size: u32,
}

impl ServiceMarker for SetIconService {}

#[async_trait]
impl AnySetIconService for SetIconService {
    fn set_icon(&self, img_path: &Path) -> Result<()> {
        let img = image::open(img_path)?;
        let (w, h) = img.dimensions();

        let side = w.min(h);
        let x0 = (w - side) / 2;
        let y0 = (h - side) / 2;
        let square = image::imageops::crop_imm(&img, x0, y0, side, side).to_image();

        let icon = image::imageops::resize(
            &square,
            self.icon_size,
            self.icon_size,
            FilterType::Lanczos3,
        );
        icon.save(self.assets_abs_path.join(COLLECTION_ICON_FILENAME))?;

        Ok(())
    }

    async fn remove_icon(&self) -> Result<()> {
        self.fs
            .remove_file(
                &self.assets_abs_path.join(COLLECTION_ICON_FILENAME),
                RemoveOptions {
                    recursive: false,
                    ignore_if_not_exists: true,
                },
            )
            .await?;
        Ok(())
    }

    fn icon_path(&self) -> Option<PathBuf> {
        let path = self.assets_abs_path.join(COLLECTION_ICON_FILENAME);
        path.exists().then_some(path)
    }
}

impl SetIconService {
    pub fn new(collection_abs_path: Arc<Path>, fs: Arc<dyn FileSystem>, icon_size: u32) -> Self {
        Self {
            fs,
            assets_abs_path: collection_abs_path.join(dirs::ASSETS_DIR).into(),
            icon_size,
        }
    }
}
