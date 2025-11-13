use serde::Deserialize;
use std::path::PathBuf;

use crate::theme::types::primitives::{ThemeId, ThemeMode};

#[derive(Deserialize, Debug)]
pub struct ThemeContributionDecl {
    pub id: ThemeId,
    pub label: String,
    pub mode: ThemeMode,
    pub path: PathBuf,
}
