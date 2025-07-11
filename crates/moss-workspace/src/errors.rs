use joinerror::ErrorMarker;

pub struct ErrorInvalidInput;
impl ErrorMarker for ErrorInvalidInput {
    const MESSAGE: &'static str = "invalid_input";
}

pub struct ErrorAlreadyExists;
impl ErrorMarker for ErrorAlreadyExists {
    const MESSAGE: &'static str = "already_exists";
}
pub struct ErrorNotFound;
impl ErrorMarker for ErrorNotFound {
    const MESSAGE: &'static str = "not_found";
}
pub struct ErrorIo;
impl ErrorMarker for ErrorIo {
    const MESSAGE: &'static str = "io";
}
pub struct ErrorInternal;
impl ErrorMarker for ErrorInternal {
    const MESSAGE: &'static str = "internal";
}

pub struct ErrorUnknown;
impl ErrorMarker for ErrorUnknown {
    const MESSAGE: &'static str = "unknown";
}
