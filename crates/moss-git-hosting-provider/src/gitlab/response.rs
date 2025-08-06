use serde::{Deserialize, Serialize};

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
