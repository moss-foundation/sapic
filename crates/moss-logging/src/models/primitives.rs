use derive_more::Deref;
use moss_id_macro::generate_id_type;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

generate_id_type!(LogEntryId);
