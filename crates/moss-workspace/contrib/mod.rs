use moss_configuration::{ConfigurationDecl, ParameterDecl, models::primitives::ParameterType};
use moss_text::read_only_str as id;
use static_json::Value;

use crate::models::primitives::ActivitybarPosition;

inventory::submit! {
    ConfigurationDecl {
        id: Some(id!("workspace")),
        parent_id: None,
        name: Some("Workspace"),
        order: Some(1),
        description: None,
        parameters: &[
            ParameterDecl {
                id: id!("workspace.activityBarPosition"),
                default: Some(Value::Str(ActivitybarPosition::Default.as_str())),
                typ: ParameterType::String,
                description: None,
                maximum: None,
                minimum: None,
                excluded: false,
                protected: false,
                order: Some(1),
                tags: &[],
            },
            ParameterDecl {
                id: id!("locale"),
                default: Some(Value::Str("moss.sapic-locale.en")),
                typ: ParameterType::String,
                description: None,
                maximum: None,
                minimum: None,
                excluded: false,
                protected: false,
                order: Some(2),
                tags: &[],
            },
        ],
    }
}
