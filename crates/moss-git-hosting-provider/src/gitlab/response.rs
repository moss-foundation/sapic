use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetRepositoryResponse {
    pub updated_at: String,
    pub owner: Owner,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Owner {
    pub username: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GetContributorsResponse {
    pub items: Vec<ContributorItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContributorItem {
    pub name: String,
    pub email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct AvatarResponse {
    pub avatar_url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetUserResponse {
    pub username: String,
    pub commit_email: String,
}
