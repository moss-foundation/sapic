use derive_more::Deref;
use moss_environment::Environment;
use sapic_base::{
    environment::types::primitives::EnvironmentId, project::types::primitives::ProjectId,
};
use sapic_system::environment::environment_edit_service::EnvironmentEditService;
use std::sync::Arc;

#[derive(Clone, Deref)]
pub struct RuntimeEnvironment {
    pub id: EnvironmentId,
    pub project_id: Option<ProjectId>,

    #[deref]
    pub handle: Arc<Environment>,
    pub(crate) edit: EnvironmentEditService,
    pub order: Option<isize>,
}
