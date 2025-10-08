use crate::models::primitives::{LanguageDirection, LanguageId};
use serde::Deserialize;

pub struct RegisterTranslationContribution(pub &'static str);
inventory::collect!(RegisterTranslationContribution);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LanguageContributionDecl {
    pub identifier: LanguageId,
    pub display_name: String,
    pub code: String,
    pub direction: Option<LanguageDirection>,
    pub path: String,
}
