use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct RepositoryResponse {
    pub created_at: String,
    pub updated_at: String,
    pub owner: Owner,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Owner {
    pub login: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct ContributorsResponse {
    pub items: Vec<ContributorItem>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct ContributorItem {
    pub login: String,
    pub avatar_url: String,
}
