use joinerror::OptionExt;
use moss_environment::constants;
use moss_text::sanitized::desanitize;
use sapic_base::environment::types::primitives::EnvironmentId;

pub mod project_edit_backend;
pub mod project_service_fs;

pub(super) fn format_env_file_name(id: &EnvironmentId) -> String {
    format!(
        "{}.{}",
        id.to_string(),
        constants::ENVIRONMENT_FILE_EXTENSION
    )
}

pub(super) fn parse_file_name(filename: &str) -> joinerror::Result<String> {
    let name = filename
        .split('.')
        .next()
        .ok_or_join_err_with::<()>(|| format!("invalid file name: {}", filename))?;

    Ok(desanitize(name))
}
