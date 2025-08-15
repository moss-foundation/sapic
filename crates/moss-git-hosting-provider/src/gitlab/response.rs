use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct RepositoryResponse {
    pub created_at: String,
    pub updated_at: String,
    pub owner: Owner,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Owner {
    pub username: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct ContributorsResponse {
    pub items: Vec<ContributorItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct ContributorItem {
    pub name: String,
    pub email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct AvatarResponse {
    pub avatar_url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct UserResponse {
    pub username: String,
    pub commit_email: String,
}
