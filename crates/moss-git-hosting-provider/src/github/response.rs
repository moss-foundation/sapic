use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct GetRepositoryResponse {
    pub owner: Owner,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Owner {
    pub login: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(transparent)]
pub struct GetContributorsResponse {
    pub items: Vec<Contributor>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Contributor {
    pub login: String,
    pub avatar_url: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct GetUserResponse {
    pub id: u64,
    pub login: String,
    pub email: Option<String>,
}
