use moss_fs::utils::encode_directory_name;

use super::primitives::EndpointFileExt;

pub(super) fn request_file_name(name: &str, typ: &EndpointFileExt) -> String {
    format!("{}.{}.sapic", encode_directory_name(name), typ.to_string())
}
