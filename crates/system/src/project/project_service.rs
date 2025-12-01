use moss_storage2::KvStorage;
use sapic_base::project::types::primitives::ProjectId;
use std::sync::Arc;

pub struct CreateProjectParams {}

pub struct ProjectItem {}

pub struct ProjectService {
    storage: Arc<dyn KvStorage>,
}

impl ProjectService {
    pub fn new(storage: Arc<dyn KvStorage>) -> Self {
        Self { storage }
    }

    pub async fn create_project(&self, params: CreateProjectParams) -> joinerror::Result<()> {
        todo!()
    }

    pub async fn delete_project(&self, id: &ProjectId) -> joinerror::Result<()> {
        todo!()
    }

    pub async fn projects(&self) -> joinerror::Result<Vec<ProjectItem>> {
        todo!()
    }
}
