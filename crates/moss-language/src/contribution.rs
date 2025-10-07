use crate::models::primitives::{Direction, LanguageId};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LanguageContributionDecl {
    pub identifier: LanguageId,
    pub display_name: String,
    pub code: String,
    pub direction: Option<Direction>,
    pub path: String,
}
