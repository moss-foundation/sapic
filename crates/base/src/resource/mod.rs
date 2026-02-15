pub mod manifest;
pub mod types;

pub mod constants {
    pub const ITEM_CONFIG_FILENAME: &str = "config.sap";
    pub const DIR_CONFIG_FILENAME: &str = "config-folder.sap";
}

pub mod errors {
    joinerror::errors! {
        ErrorPathInvalid => "path_invalid",
        ErrorPathNotFound => "path_not_found",
        ErrorPathAlreadyExists => "path_already_exists",

        ErrorNameInvalid => "name_invalid",
        ErrorNameAlreadyExists => "name_already_exists",
    }
}
