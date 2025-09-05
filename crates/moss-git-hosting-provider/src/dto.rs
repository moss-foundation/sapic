use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TokenExchangeRequest {
    pub code: String,
    pub state: String,
}
