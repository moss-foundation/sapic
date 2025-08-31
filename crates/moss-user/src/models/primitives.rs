#[cfg(any(test, feature = "integration-tests"))]
use moss_applib::context::ContextValue;
use moss_id_macro::ids;

ids!([AccountId]);

#[cfg(any(test, feature = "integration-tests"))]
impl ContextValue for AccountId {}
