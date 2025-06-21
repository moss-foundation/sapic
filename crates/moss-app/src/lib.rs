pub mod api;
pub mod app;
pub mod command;
pub mod context;
pub mod models;
pub mod services;
pub mod storage;

pub mod constants {

    // ##################################################################
    // ###                                                            ###
    // ### !!! PLEASE UPDATE THE TYPESCRIPT CONSTANTS IN index.ts !!! ###
    // ###                                                            ###
    // ##################################################################

    // When adding/removing/modifying any constants here,
    // you must also update the corresponding TypeScript constants
    // in moss-app/index.ts to match your changes.

    pub const LOGGING_SERVICE_CHANNEL: &'static str = "logging";
}

pub mod dirs {
    pub const WORKSPACES_DIR: &str = "workspaces";
    pub const GLOBALS_DIR: &str = "globals";
}
