use crate::dirs;
use anyhow::Result;
use image::{GenericImageView, imageops::FilterType};
use moss_fs::{FileSystem, RemoveOptions};
use sapic_core::context::AnyAsyncContext;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

const COLLECTION_ICON_FILENAME: &str = "icon.png";
pub struct SetIconService {
    fs: Arc<dyn FileSystem>,
    assets_abs_path: Arc<Path>,
    icon_size: u32,
}

impl SetIconService {
    pub fn set_icon(&self, img_path: &Path) -> Result<()> {
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

    pub async fn remove_icon(&self, ctx: &dyn AnyAsyncContext) -> Result<()> {
        self.fs
            .remove_file(
                ctx,
                &self.assets_abs_path.join(COLLECTION_ICON_FILENAME),
                RemoveOptions {
                    recursive: false,
                    ignore_if_not_exists: true,
                },
            )
            .await?;
        Ok(())
    }

    pub fn icon_path(&self) -> Option<PathBuf> {
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
