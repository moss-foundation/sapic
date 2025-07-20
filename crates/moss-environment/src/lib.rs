pub mod builder;
pub mod configuration;
pub mod environment;
pub mod file;
pub mod models;
pub mod services;

pub mod errors {
    use joinerror::error::ErrorMarker;

    pub struct ErrorEnvironmentAlreadyExists;
    impl ErrorMarker for ErrorEnvironmentAlreadyExists {
        const MESSAGE: &'static str = "already_exists";
    }

    pub struct ErrorEnvironmentNotFound;
    impl ErrorMarker for ErrorEnvironmentNotFound {
        const MESSAGE: &'static str = "not_found";
    }
}
