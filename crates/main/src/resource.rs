use sapic_base::resource::{manifest::EntryModel, types::ResourceId};
use sapic_system::resource::resource_edit_service::ResourceEditService;

pub struct RuntimeResource {
    pub id: ResourceId,
    pub manifest: EntryModel,

    pub(crate) edit: ResourceEditService,
}
