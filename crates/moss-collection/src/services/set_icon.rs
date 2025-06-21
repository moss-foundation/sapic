use anyhow::Result;
use image::{GenericImageView, imageops::FilterType};
use std::path::Path;

pub struct SetIconService {}

impl SetIconService {
    pub fn set_icon(img_path: &Path, output_path: &Path, icon_size: u32) -> Result<()> {
        let img = image::open(img_path)?;
        let (w, h) = img.dimensions();

        let side = w.min(h);
        let x0 = (w - side) / 2;
        let y0 = (h - side) / 2;
        let square = image::imageops::crop_imm(&img, x0, y0, side, side).to_image();

        let icon = image::imageops::resize(&square, icon_size, icon_size, FilterType::Lanczos3);
        icon.save(output_path)?;

        Ok(())
    }
}
