use serde::Deserialize;

use crate::language::types::primitives::{LanguageCode, LanguageDirection};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LanguageContributionDecl {
    pub display_name: String,
    pub code: LanguageCode,
    pub direction: Option<LanguageDirection>,
    pub path: String,
}
