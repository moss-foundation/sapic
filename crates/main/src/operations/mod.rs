pub mod activate_environment;
pub mod archive_project;
pub mod batch_update_environment;
pub mod batch_update_project;
pub mod cancel_request;
pub mod create_environment;
pub mod create_project;
pub mod create_workspace;
pub mod delete_environment;
pub mod delete_project;
mod describe_environment;
pub mod describe_project;
pub mod export_project;
pub mod import_project;
pub mod open_workspace;
pub mod stream_environments;
mod stream_project_environments;
pub mod stream_projects;
pub mod unarchive_project;
pub mod update_environment;
pub mod update_project;
pub mod update_workspace;
// FIXME: Should we have separate endpoints for operations on workspace/project environments?
// For now I'll go with adding an optional field for ProjectId in environment operations input
// If it's none it would be on the workspace, otherwise it would be on the project
