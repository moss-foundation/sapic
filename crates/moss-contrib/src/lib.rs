pub mod include;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContributionKey {
    Configuration,
    Themes,
    Localizations,
    ResourceParams,
}

impl std::fmt::Display for ContributionKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContributionKey::Configuration => write!(f, "configuration"),
            ContributionKey::Themes => write!(f, "themes"),
            ContributionKey::Localizations => write!(f, "localizations"),
            ContributionKey::ResourceParams => write!(f, "resource_params"),
        }
    }
}

impl TryFrom<&str> for ContributionKey {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "configuration" => Ok(ContributionKey::Configuration),
            "themes" => Ok(ContributionKey::Themes),
            "localizations" => Ok(ContributionKey::Localizations),
            "resource_params" => Ok(ContributionKey::ResourceParams),
            _ => Err(format!("Unknown contribution key: {}", value)),
        }
    }
}
