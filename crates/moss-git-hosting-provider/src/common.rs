use moss_git::GitAuthAgent;

pub mod ssh_auth_agent;

pub trait SHHAuthAgent: GitAuthAgent {}
