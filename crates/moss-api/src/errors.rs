use joinerror::error::ErrorMarker;
use validator::ValidationErrors;

pub struct InvalidInput;

impl ErrorMarker for InvalidInput {
    const MESSAGE: &'static str = "invalid_input";
}

impl InvalidInput {
    pub fn from(errors: ValidationErrors) -> joinerror::Error {
        joinerror::Error::new::<Self>(errors.to_string())
    }
}

pub struct PreconditionFailed;

impl ErrorMarker for PreconditionFailed {
    const MESSAGE: &'static str = "precondition_failed";
}
