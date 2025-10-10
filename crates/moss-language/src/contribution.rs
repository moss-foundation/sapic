use crate::models::primitives::{LanguageCode, LanguageDirection};
use serde::Deserialize;

pub struct RegisterTranslationContribution(pub &'static str);
inventory::collect!(RegisterTranslationContribution);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LanguageContributionDecl {
    pub display_name: String,
    pub code: LanguageCode,
    pub direction: Option<LanguageDirection>,
    pub path: String,
}
