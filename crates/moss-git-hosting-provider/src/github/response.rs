use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct GetRepositoryResponse {
    pub updated_at: String,
    pub owner: Owner,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Owner {
    pub login: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(transparent)]
pub struct GetContributorsResponse {
    pub items: Vec<ContributorItem>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ContributorItem {
    pub login: String,
    pub avatar_url: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct GetUserResponse {
    pub id: u64,
    pub login: String,
    pub email: Option<String>,
}
