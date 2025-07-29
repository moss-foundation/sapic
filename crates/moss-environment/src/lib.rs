pub mod builder;
pub mod configuration;
pub mod environment;
pub mod file;
pub mod models;
pub mod services;

pub mod constants {
    pub const ENVIRONMENT_FILE_EXTENSION: &str = "env.sap";
}

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

    pub struct ErrorFailedToEncode;
    impl ErrorMarker for ErrorFailedToEncode {
        const MESSAGE: &'static str = "failed_to_encode";
    }

    pub struct ErrorFailedToDecode;
    impl ErrorMarker for ErrorFailedToDecode {
        const MESSAGE: &'static str = "failed_to_decode";
    }

    pub struct ErrorIo;
    impl ErrorMarker for ErrorIo {
        const MESSAGE: &'static str = "io";
    }
}
