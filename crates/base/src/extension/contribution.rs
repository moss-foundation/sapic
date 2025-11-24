#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContributionKey(&'static str);

impl ContributionKey {
    pub const fn new(key: &'static str) -> Self {
        Self(key)
    }

    pub fn as_str(&self) -> &'static str {
        self.0
    }
}
