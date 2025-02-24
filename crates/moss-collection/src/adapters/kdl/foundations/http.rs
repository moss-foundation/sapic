use crate::adapters::kdl::tokens::{HEADERS_LIT, PARAMS_LIT, URL_LIT};
use kdl::{KdlDocument, KdlEntry, KdlNode, KdlValue};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Default)]
pub enum HttpMethod {
    Post,
    Put,
    #[default]
    Get,
    Delete,
}

// #[derive(Debug)]
// pub struct Metadata {
//     pub order: Option<usize>,
//     pub method: HttpMethod,
// }

#[derive(Clone, Debug)]
pub struct Url {
    pub raw: Option<String>,
    pub host: Option<String>,
}

impl Into<KdlNode> for Url {
    fn into(self) -> KdlNode {
        let mut node = KdlNode::new(URL_LIT);
        let mut children = KdlDocument::new();
        if let Some(raw) = self.raw {
            let mut raw_node = KdlNode::new("raw");
            raw_node.push(KdlEntry::new(raw.as_str()));
            children.nodes_mut().push(raw_node);
        }
        if let Some(host) = self.host {
            let mut host_node = KdlNode::new("host");
            host_node.push(KdlEntry::new(host.as_str()));
            children.nodes_mut().push(host_node);
        }
        node.set_children(children);
        node
    }
}

#[derive(Clone, Debug, Default)]
pub struct QueryParamBody {
    pub value: Option<KdlValue>,
    pub desc: Option<String>,
    pub order: Option<usize>,
    pub disabled: bool,
    pub options: QueryParamOptions,
}

impl Into<KdlDocument> for QueryParamBody {
    fn into(self) -> KdlDocument {
        let mut doc = KdlDocument::new();
        if let Some(value) = self.value {
            let mut value_node = KdlNode::new("value");
            value_node.push(KdlEntry::new(value));
            doc.nodes_mut().push(value_node);
        }
        if let Some(desc) = self.desc {
            let mut desc_node = KdlNode::new("desc");
            desc_node.push(KdlEntry::new(desc));
            doc.nodes_mut().push(desc_node);
        }
        if let Some(order) = self.order {
            let mut order_node = KdlNode::new("order");
            order_node.push(KdlEntry::new(order as i128));
            doc.nodes_mut().push(order_node);
        }
        let mut disabled_node = KdlNode::new("disabled");
        disabled_node.push(KdlEntry::new(self.disabled));
        doc.nodes_mut().push(disabled_node);
        let options_node: KdlNode = self.options.into();
        doc.nodes_mut().push(options_node);
        doc
    }
}
#[derive(Clone, Debug, Default)]
pub struct QueryParamOptions {
    pub propagate: bool,
}

impl Into<KdlNode> for QueryParamOptions {
    fn into(self) -> KdlNode {
        let mut node = KdlNode::new("options");
        let mut children = KdlDocument::new();
        let mut propagate_node = KdlNode::new("propagate");
        propagate_node.push(KdlEntry::new(self.propagate));
        children.nodes_mut().push(propagate_node);
        node.set_children(children);
        node
    }
}

#[derive(Clone, Debug, Default)]
pub struct PathParamBody {
    pub value: Option<KdlValue>,
    pub desc: Option<String>,
    pub order: Option<usize>,
    pub disabled: bool,
    pub options: PathParamOptions,
}

impl Into<KdlDocument> for PathParamBody {
    fn into(self) -> KdlDocument {
        let mut doc = KdlDocument::new();
        if let Some(value) = self.value {
            let mut value_node = KdlNode::new("value");
            value_node.push(KdlEntry::new(value));
            doc.nodes_mut().push(value_node);
        }
        if let Some(desc) = self.desc {
            let mut desc_node = KdlNode::new("desc");
            desc_node.push(KdlEntry::new(desc));
            doc.nodes_mut().push(desc_node);
        }
        if let Some(order) = self.order {
            let mut order_node = KdlNode::new("order");
            order_node.push(KdlEntry::new(order as i128));
            doc.nodes_mut().push(order_node);
        }
        let mut disabled_node = KdlNode::new("disabled");
        disabled_node.push(KdlEntry::new(self.disabled));
        doc.nodes_mut().push(disabled_node);
        let options_node: KdlNode = self.options.into();
        doc.nodes_mut().push(options_node);
        doc
    }
}

#[derive(Clone, Debug, Default)]
pub struct PathParamOptions {
    pub propagate: bool,
}

impl Into<KdlNode> for PathParamOptions {
    fn into(self) -> KdlNode {
        let mut node = KdlNode::new("options");
        let mut children = KdlDocument::new();
        let mut propagate_node = KdlNode::new("propagate");
        propagate_node.push(KdlEntry::new(self.propagate));
        children.nodes_mut().push(propagate_node);
        node.set_children(children);
        node
    }
}

#[derive(Clone, Debug, Default)]
pub struct HeaderBody {
    pub value: Option<KdlValue>,
    pub desc: Option<String>,
    pub order: Option<usize>,
    pub disabled: bool,
    pub options: HeaderOptions,
}

impl Into<KdlDocument> for HeaderBody {
    fn into(self) -> KdlDocument {
        let mut doc = KdlDocument::new();
        if let Some(value) = self.value {
            let mut value_node = KdlNode::new("value");
            value_node.push(KdlEntry::new(value));
            doc.nodes_mut().push(value_node);
        }
        if let Some(desc) = self.desc {
            let mut desc_node = KdlNode::new("desc");
            desc_node.push(KdlEntry::new(desc));
            doc.nodes_mut().push(desc_node);
        }
        if let Some(order) = self.order {
            let mut order_node = KdlNode::new("order");
            order_node.push(KdlEntry::new(order as i128));
            doc.nodes_mut().push(order_node);
        }
        let mut disabled_node = KdlNode::new("disabled");
        disabled_node.push(KdlEntry::new(self.disabled));
        doc.nodes_mut().push(disabled_node);
        let options_node: KdlNode = self.options.into();
        doc.nodes_mut().push(options_node);
        doc
    }
}

#[derive(Clone, Debug, Default)]
pub struct HeaderOptions {
    pub propagate: bool,
}

impl Into<KdlNode> for HeaderOptions {
    fn into(self) -> KdlNode {
        let mut node = KdlNode::new("options");
        let mut children = KdlDocument::new();
        let mut propagate_node = KdlNode::new("propagate");
        propagate_node.push(KdlEntry::new(self.propagate));
        children.nodes_mut().push(propagate_node);
        node.set_children(children);
        node
    }
}

#[derive(Clone, Debug, Default)]
pub struct Request {
    // pub metadata: Option<Metadata>,
    pub url: Option<Url>,
    pub query_params: Option<HashMap<String, QueryParamBody>>,
    pub path_params: Option<HashMap<String, PathParamBody>>,
    pub headers: Option<HashMap<String, HeaderBody>>,
}

impl ToString for Request {
    fn to_string(&self) -> String {
        let mut document = KdlDocument::new();
        let mut nodes = document.nodes_mut();
        // let metadata_node: KdlNode = self.metadata.into();
        // nodes.push(metadata_node);
        if let Some(url) = &self.url {
            let url_node: KdlNode = url.clone().into();
            nodes.push(url_node);
        }
        if let Some(query_params) = &self.query_params {
            let mut query_params_node = KdlNode::new(PARAMS_LIT);
            query_params_node.push(KdlEntry::new_prop("type", "query"));
            let mut children = KdlDocument::new();
            for (name, body) in query_params {
                let mut param_node = KdlNode::new(name.to_string());
                param_node.set_children(body.clone().into());
                children.nodes_mut().push(param_node);
            }
            query_params_node.set_children(children);
            nodes.push(query_params_node);
        }
        if let Some(path_params) = &self.path_params {
            let mut path_params_node = KdlNode::new(PARAMS_LIT);
            path_params_node.push(KdlEntry::new_prop("type", "path"));
            let mut children = KdlDocument::new();
            for (name, body) in path_params {
                let mut param_node = KdlNode::new(name.clone());
                param_node.set_children(body.clone().into());
                children.nodes_mut().push(param_node);
            }
            path_params_node.set_children(children);
            nodes.push(path_params_node);
        }
        if let Some(headers) = &self.headers {
            let mut headers_node = KdlNode::new(HEADERS_LIT);
            let mut children = KdlDocument::new();
            for (name, body) in headers {
                let mut header_node = KdlNode::new(name.clone());
                header_node.set_children(body.clone().into());
                children.nodes_mut().push(header_node);
            }
            headers_node.set_children(children);
            nodes.push(headers_node);
        }
        document.autoformat();
        document.to_string()
    }
}
