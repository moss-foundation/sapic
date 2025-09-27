pub mod expression;
pub mod heredoc;
pub mod object;
pub mod util;

pub use crate::{
    expression::{
        deserialize_expression, deserialize_optional_expression, serialize_expression,
        serialize_optional_expression,
    },
    object::Object,
    util::{hcl_to_json, json_to_hcl},
};

pub use hcl::ser::{Block, LabeledBlock};
use joinerror::error::ErrorMarker;

pub trait HclResultExt<T> {
    fn join_err<E: ErrorMarker>(self, details: impl Into<String>) -> joinerror::Result<T>;

    fn join_err_with<E: ErrorMarker>(
        self,
        details: impl FnOnce() -> String,
    ) -> joinerror::Result<T>;
}

impl<T> HclResultExt<T> for hcl::Result<T> {
    fn join_err<E: ErrorMarker>(self, details: impl Into<String>) -> joinerror::Result<T> {
        self.map_err(|e| joinerror::Error::new::<()>(e.to_string()).join::<E>(details))
    }

    fn join_err_with<E: ErrorMarker>(
        self,
        details: impl FnOnce() -> String,
    ) -> joinerror::Result<T> {
        self.map_err(|e| joinerror::Error::new::<()>(e.to_string()).join_with::<E>(details))
    }
}
