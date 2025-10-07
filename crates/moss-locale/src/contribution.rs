use crate::models::primitives::{Direction, LocaleId};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocaleContributionDecl {
    pub identifier: LocaleId,
    pub display_name: String,
    pub code: String,
    pub direction: Option<Direction>,
    pub path: String,
}
