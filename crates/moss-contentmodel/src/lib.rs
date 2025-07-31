pub mod hcl;
pub mod json;

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum ContentModel {
    Json(json::JsonModel),
}

impl ContentModel {
    pub fn as_json(&self) -> Option<&json::JsonModel> {
        match self {
            ContentModel::Json(model) => Some(model),
        }
    }

    pub fn as_json_mut(&mut self) -> Option<&mut json::JsonModel> {
        match self {
            ContentModel::Json(model) => Some(model),
        }
    }
}
