use moss_applib::AppRuntime;

use crate::{environment::Environment, models::operations::ChangeVariableSetInput};

impl<R: AppRuntime> Environment<R> {
    pub async fn change_variable_set(
        &self,
        input: ChangeVariableSetInput,
    ) -> joinerror::Result<()> {
        Ok(())
    }
}
