pub mod hcl;
pub mod json;

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Model {
    Json(json::JsonModel),
}

impl Model {
    pub fn as_json(&self) -> Option<&json::JsonModel> {
        match self {
            Model::Json(model) => Some(model),
        }
    }

    pub fn as_json_mut(&mut self) -> Option<&mut json::JsonModel> {
        match self {
            Model::Json(model) => Some(model),
        }
    }
}
