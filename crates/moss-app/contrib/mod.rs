use moss_configuration::{ConfigurationDecl, ParameterDecl, models::primitives::ParameterType};
use moss_text::read_only_str as id;
use static_json::Value;

inventory::submit! {
    ConfigurationDecl {
        id: Some(id!("app")),
        parent_id: None,
        name: Some("App"),
        order: Some(1),
        description: None,
        parameters: &[
            ParameterDecl {
                id: id!("colorTheme"),
                default: Some(Value::Str("moss.sapic-theme.lightDefault")),
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
