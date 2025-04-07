use moss_fs::utils::encode_directory_name;

use super::primitives::FileExt;

pub(super) fn request_file_name(name: &str, typ: &FileExt) -> String {
    format!("{}.{}.sapic", encode_directory_name(name), typ.to_string())
}
