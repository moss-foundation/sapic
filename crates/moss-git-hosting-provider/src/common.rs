pub mod ssh_auth_agent;
pub mod utils;

use moss_git::GitAuthAgent;

pub trait SSHAuthAgent: GitAuthAgent {}
