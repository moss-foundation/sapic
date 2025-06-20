use anyhow::Result;
use image::{GenericImageView, imageops::FilterType};

struct ImageUploadService {}

impl ImageUploadService {
    fn upload_icon(img_path: &str, output_path: &str, icon_size: u32) -> Result<()> {
        // Load the image
        let img = image::open(img_path)?;
        let (w, h) = img.dimensions();

        // Crop to centered square
        let side = w.min(h);
        let x0 = (w - side) / 2;
        let y0 = (h - side) / 2;
        let square = image::imageops::crop_imm(&img, x0, y0, side, side).to_image();

        // Resize down/up to your avatar_size
        let icon = image::imageops::resize(&square, icon_size, icon_size, FilterType::Lanczos3);

        // Save to the given output path
        icon.save(output_path)?;
        Ok(())
    }
}
