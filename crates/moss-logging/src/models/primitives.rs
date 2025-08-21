use derive_more::Deref;
use moss_id_macro::generate_id_type;
use serde::{Deserialize, Serialize};

generate_id_type!(LogEntryId);
