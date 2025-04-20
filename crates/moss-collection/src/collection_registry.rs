use moss_common::leased_slotmap::{LeasedSlotMap, ResourceKey};
use moss_models::collection::types::{HttpMethod, RequestProtocol};
use std::path::PathBuf;
use tokio::sync::RwLock;

use crate::constants::*;

/// Data for a request node
///
/// This data is used to store the request node in the collection registry.
pub struct CollectionRequestData {
    /// The name of the request node
    ///
    /// This is the decoded name of the request node, like `MyRequest`.
    pub name: String,
    /// The path of the request node
    ///
    /// This is the relative path of the request node from
    /// the collection root directory, like `/requests/.../MyRequest.request`.
    pub path: PathBuf,
    /// The order of the request node
    ///
    /// This data we get when we restore the node data from the database
    /// of saved states. If there is no data saved in the previous session,
    /// this value will be `None`.
    pub order: Option<usize>,
    /// The name of the spec file
    ///
    /// This is the name of the spec file, like `get.spec`.
    pub spec_file_name: String,
}

impl CollectionRequestData {
    pub fn protocol(&self) -> RequestProtocol {
        match self.spec_file_name.as_str() {
            GET_ENTRY_SPEC_FILE => RequestProtocol::Http(HttpMethod::Get),
            POST_ENTRY_SPEC_FILE => RequestProtocol::Http(HttpMethod::Post),
            PUT_ENTRY_SPEC_FILE => RequestProtocol::Http(HttpMethod::Put),
            DELETE_ENTRY_SPEC_FILE => RequestProtocol::Http(HttpMethod::Delete),
            GRAPHQL_ENTRY_SPEC_FILE => RequestProtocol::GraphQL,
            GRPC_ENTRY_SPEC_FILE => RequestProtocol::Grpc,
            _ => RequestProtocol::Http(HttpMethod::Get),
        }
    }
}

/// Data for a request group node
///
/// This data is used to store the request group node in the collection registry.
pub struct CollectionRequestGroupData {
    /// The name of the request group node
    ///
    /// This is the decoded name of the request group node, like `MySubfolder`.
    pub name: String,
    /// The path of the request group node
    ///
    /// This is the relative path of the request group node from
    /// the collection root directory, like `/requests/.../MySubfolder`.
    pub path: PathBuf,
    /// The order of the request group node
    ///
    /// This data we get when we restore the node data from the database
    /// of saved states. If there is no data saved in the previous session,
    /// this value will be `None`.
    pub order: Option<usize>,
    /// The name of the spec file
    ///
    /// This is the name of the spec file, like `folder.spec`.
    ///
    /// This value is optional because the request group node
    /// may not have a spec file.
    pub spec_file_name: Option<String>,
}

pub enum RequestNode {
    Request(CollectionRequestData),
    Group(CollectionRequestGroupData),
}

impl RequestNode {
    pub fn name(&self) -> &str {
        match self {
            RequestNode::Request(data) => &data.name,
            RequestNode::Group(data) => &data.name,
        }
    }

    pub fn set_name(&mut self, new_name: String) {
        match self {
            RequestNode::Request(data) => data.name = new_name,
            RequestNode::Group(data) => data.name = new_name,
        }
    }

    pub fn path(&self) -> &PathBuf {
        match self {
            RequestNode::Request(data) => &data.path,
            RequestNode::Group(data) => &data.path,
        }
    }

    pub fn set_path(&mut self, new_path: PathBuf) {
        match self {
            RequestNode::Request(data) => data.path = new_path,
            RequestNode::Group(data) => data.path = new_path,
        }
    }

    pub fn order(&self) -> Option<usize> {
        match self {
            RequestNode::Request(data) => data.order,
            RequestNode::Group(data) => data.order,
        }
    }

    pub fn set_order(&mut self, new_order: Option<usize>) {
        match self {
            RequestNode::Request(data) => data.order = new_order,
            RequestNode::Group(data) => data.order = new_order,
        }
    }

    pub fn is_request(&self) -> bool {
        match self {
            RequestNode::Request(_) => true,
            _ => false,
        }
    }

    pub fn is_request_group(&self) -> bool {
        match self {
            RequestNode::Group(_) => true,
            _ => false,
        }
    }
}

pub struct EndpointData {}
pub struct EndpointGroupData {}

pub enum EndpointNode {
    Endpoint(EndpointData),
    Group(EndpointGroupData),
}

pub struct SchemaData {}
pub struct SchemaGroupData {}

pub enum SchemaNode {
    Schema(SchemaData),
    Group(SchemaGroupData),
}

pub struct ComponentData {}
pub struct ComponentGroupData {}

pub enum ComponentNode {
    Component(ComponentData),
    Group(ComponentGroupData),
}

type RequestNodeMap = LeasedSlotMap<ResourceKey, RequestNode>;
type EndpointNodeMap = LeasedSlotMap<ResourceKey, EndpointNode>;
type SchemaNodeMap = LeasedSlotMap<ResourceKey, SchemaNode>;
type ComponentNodeMap = LeasedSlotMap<ResourceKey, ComponentNode>;

pub struct CollectionRegistry {
    requests_nodes: RwLock<RequestNodeMap>,
    endpoints_nodes: RwLock<EndpointNodeMap>,
    schemas_nodes: RwLock<SchemaNodeMap>,
    components_nodes: RwLock<ComponentNodeMap>,
}

impl CollectionRegistry {
    pub fn new(
        requests_nodes: RequestNodeMap,
        endpoints_nodes: EndpointNodeMap,
        schemas_nodes: SchemaNodeMap,
        components_nodes: ComponentNodeMap,
    ) -> Self {
        Self {
            requests_nodes: RwLock::new(requests_nodes),
            endpoints_nodes: RwLock::new(endpoints_nodes),
            schemas_nodes: RwLock::new(schemas_nodes),
            components_nodes: RwLock::new(components_nodes),
        }
    }

    pub fn requests_nodes(&self) -> &RwLock<RequestNodeMap> {
        &self.requests_nodes
    }

    pub fn endpoints_nodes(&self) -> &RwLock<EndpointNodeMap> {
        &self.endpoints_nodes
    }

    pub fn schemas_nodes(&self) -> &RwLock<SchemaNodeMap> {
        &self.schemas_nodes
    }

    pub fn components_nodes(&self) -> &RwLock<ComponentNodeMap> {
        &self.components_nodes
    }
}
