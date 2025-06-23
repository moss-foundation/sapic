pub mod common;
pub mod dir;
pub mod item;

pub use common::*;
pub use dir::*;
use hcl::Body;
pub use item::*;

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum ConfigurationModel {
    Item(CompositeItemConfigurationModel),
    Dir(CompositeDirConfigurationModel),
}

impl ConfigurationModel {
    pub fn to_hcl(&self) -> Body {
        match self {
            ConfigurationModel::Item(item) => item.to_hcl(),
            ConfigurationModel::Dir(dir) => unimplemented!(),
        }
    }
}

impl ConfigurationModel {
    pub fn id(&self) -> Uuid {
        match self {
            ConfigurationModel::Item(item) => item.metadata.id,
            ConfigurationModel::Dir(dir) => dir.metadata.id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_configuration_model_item_hcl_serialization() {
        let model = ConfigurationModel::Item(CompositeItemConfigurationModel {
            metadata: common::ConfigurationMetadata {
                id: uuid::Uuid::new_v4(),
            },
            inner: item::ItemConfigurationModel::Request(
                item::RequestItemConfigurationModel::Http(item::HttpRequestItemConfiguration {
                    request_parts: item::HttpRequestParts {
                        method: crate::models::primitives::HttpMethod::Get,
                    },
                }),
            ),
        });

        let hcl_body = model.to_hcl();
        let hcl_string = hcl::to_string(&hcl_body).expect("Failed to serialize to HCL");

        println!("ConfigurationModel (Item) serialized to HCL:");
        println!("{}", hcl_string);

        // let _deserialized: ConfigurationModel =
        //     hcl::from_str(&hcl_string).expect("Failed to deserialize from HCL");
    }

    // #[test]
    // fn test_configuration_model_dir_hcl_serialization() {
    //     let model = ConfigurationModel::Dir(CompositeDirConfigurationModel::default());

    //     let hcl_string = hcl::to_string(&model).expect("Failed to serialize to HCL");

    //     println!("ConfigurationModel (Dir) serialized to HCL:");
    //     println!("{}", hcl_string);

    //     let _deserialized: ConfigurationModel =
    //         hcl::from_str(&hcl_string).expect("Failed to deserialize from HCL");
    // }

    // #[test]
    // fn test_configuration_model_item_hcl_serialization() {
    //     let model = ConfigurationModel::Item(CompositeItemConfigurationModel {
    //         metadata: common::ConfigurationMetadata {
    //             id: uuid::Uuid::new_v4(),
    //         },
    //         inner: item::ItemConfigurationModel::Request(
    //             item::RequestItemConfigurationModel::Http(item::HttpRequestItemConfiguration {
    //                 request_parts: item::HttpRequestParts {
    //                     method: crate::models::primitives::HttpMethod::Get,
    //                 },
    //             }),
    //         ),
    //     });

    //     let hcl_string = hcl::to_string(&model).expect("Failed to serialize to HCL");

    //     println!("ConfigurationModel (Item) serialized to HCL:");
    //     println!("{}", hcl_string);

    //     let _deserialized: ConfigurationModel =
    //         hcl::from_str(&hcl_string).expect("Failed to deserialize from HCL");
    // }

    // #[test]
    // fn test_configuration_model_hcl_block_serialization() {
    //     let body = Body::builder()
    //         .add_attribute((
    //             "metadata",
    //             Expression::from_iter([(
    //                 "id",
    //                 Expression::String("c1498caa-fbd9-443b-bda3-e84eb2398dde".to_string()),
    //             )]),
    //         ))
    //         .add_block(
    //             Block::builder("request")
    //                 .add_label("http")
    //                 .add_attribute(("method", "GET"))
    //                 .build(),
    //         )
    //         .build();

    //     let hcl_string = hcl::to_string(&body).expect("Failed to serialize to HCL");

    //     println!("HCL with labels:");
    //     println!("{}", hcl_string);
    // }

    // #[test]
    // fn test_configuration_model_custom_hcl_serialization() {
    //     let model = ConfigurationModel::Item(CompositeItemConfigurationModel {
    //         metadata: common::ConfigurationMetadata {
    //             id: uuid::Uuid::parse_str("c1498caa-fbd9-443b-bda3-e84eb2398dde").unwrap(),
    //         },
    //         inner: item::ItemConfigurationModel::Request(
    //             item::RequestItemConfigurationModel::Http(item::HttpRequestItemConfiguration {
    //                 request_parts: item::HttpRequestParts {
    //                     method: crate::models::primitives::HttpMethod::Get,
    //                 },
    //             }),
    //         ),
    //     });

    //     let body = Body::builder()
    //         .add_attribute((
    //             "metadata",
    //             Expression::from_iter([("id", Expression::String(model.id().to_string()))]),
    //         ))
    //         .add_block(
    //             Block::builder("request")
    //                 .add_label("http")
    //                 .add_attribute(("method", "GET"))
    //                 .build(),
    //         )
    //         .build();

    //     let hcl_string = hcl::to_string(&body).expect("Failed to serialize to HCL");

    //     println!("ConfigurationModel custom HCL format:");
    //     println!("{}", hcl_string);
    // }

    // #[test]
    // fn test_configuration_model_direct_hcl_serialization() {
    //     let model = ConfigurationModel::Item(CompositeItemConfigurationModel {
    //         metadata: common::ConfigurationMetadata {
    //             id: uuid::Uuid::parse_str("c1498caa-fbd9-443b-bda3-e84eb2398dde").unwrap(),
    //         },
    //         inner: item::ItemConfigurationModel::Request(
    //             item::RequestItemConfigurationModel::Http(item::HttpRequestItemConfiguration {
    //                 request_parts: item::HttpRequestParts {
    //                     method: crate::models::primitives::HttpMethod::Get,
    //                 },
    //             }),
    //         ),
    //     });

    //     let hcl_string =
    //         hcl::to_string(&model).expect("Failed to serialize ConfigurationModel to HCL");

    //     println!("ConfigurationModel direct HCL serialization:");
    //     println!("{}", hcl_string);

    //     let deserialized_model: ConfigurationModel =
    //         hcl::from_str(&hcl_string).expect("Failed to deserialize ConfigurationModel from HCL");

    //     assert_eq!(model.id(), deserialized_model.id());
    //     println!("Successfully round-trip serialized/deserialized ConfigurationModel!");
    // }
}
