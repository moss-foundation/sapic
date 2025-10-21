use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serialize};

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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct PATExpiresAtResponse {
    /// GitLab's PAT expires_at response is date only, e.g. 2025-11-19
    /// We need to process it into DateTime<Utc>

    #[serde(deserialize_with = "deserialize_expires_at_response")]
    pub expires_at: DateTime<Utc>,
}

fn deserialize_expires_at_response<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let date = NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(serde::de::Error::custom)?;
    let naive_dt = date.and_hms_opt(0, 0, 0).ok_or(serde::de::Error::custom(
        "failed to convert NaiveDate to NaiveDateTime",
    ))?;
    Ok(Utc.from_utc_datetime(&naive_dt))
}
