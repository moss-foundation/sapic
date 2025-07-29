use crate::models::{
    primitives::VariableId,
    types::{AddVariableParams, UpdateVariableParams},
};

pub struct ChangeVariableSetInput {
    pub vars_to_add: Vec<AddVariableParams>,
    pub vars_to_update: Vec<UpdateVariableParams>,
    pub vars_to_delete: Vec<VariableId>,
}
