//! Internal types representing data abstracted from VCS providers.
//!
//! This module contains internal types that serve as a unified interface for business logic,
//! abstracting away the differences between various VCS providers (GitHub, GitLab, Bitbucket, etc.).
//!
//! Since different providers have different structures and data sets in their API endpoints,
//! we need an abstraction layer that provides a consistent interface for all underlying business logic.
//! These types act as a common data model that can be populated from any VCS provider's API response.

pub struct OwnerInfo {
    pub username: String,
}

pub struct RepositoryInfo {
    pub updated_at: String,
    pub owner: OwnerInfo,
}

pub struct ContributorInfo {
    pub username: String,
    pub avatar_url: Option<String>,
}
