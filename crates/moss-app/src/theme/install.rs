use joinerror::OptionExt;
use std::path::Path;

use crate::theme::{
    conversion::convert_theme_to_css, models::types::Theme, validation::validate_theme,
};

// Temporarily set as public to be called by the installer tool
pub fn install_theme(
    input_path: &Path,
    policy_path: &Path,
    themes_dir: &Path,
) -> joinerror::Result<()> {
    let theme: Theme = serde_json::from_reader(std::fs::File::open(input_path)?)?;
    validate_theme(policy_path, &theme)?;

    let css = convert_theme_to_css(&theme)?;
    // TODO: should the filename be based on identifier?
    let file_stem = input_path
        .file_stem()
        .ok_or_join_err::<()>("theme file must have a valid filename")?;

    std::fs::write(
        themes_dir.join(file_stem).with_extension("css"),
        css.as_bytes(),
    )?;

    Ok(())
}
