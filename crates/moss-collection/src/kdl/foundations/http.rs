use crate::kdl::tokens::{HEADERS_LIT, PARAMS_LIT, URL_LIT};
use kdl::{KdlDocument, KdlEntry, KdlNode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;

#[derive(Clone, Debug, Default)]
pub struct Url {
    pub raw: Option<String>,
    pub host: Option<String>,
}

impl Url {
    pub fn new(raw: String) -> Self {
        // TODO: implement this
        Self {
            raw: Some(raw),
            host: None,
        }
    }
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

#[derive(Clone, Debug)]
pub struct QueryParamBody {
    pub value: String,
    pub desc: Option<String>,
    pub order: Option<usize>,
    pub disabled: bool,
    pub options: QueryParamOptions,
}

impl Default for QueryParamBody {
    fn default() -> Self {
        Self {
            value: "".to_string(),
            desc: None,
            order: None,
            disabled: false,
            options: QueryParamOptions::default(),
        }
    }
}

impl Into<KdlDocument> for QueryParamBody {
    fn into(self) -> KdlDocument {
        let mut doc = KdlDocument::new();

        let mut value_node = KdlNode::new("value");
        value_node.push(KdlEntry::new(self.value));
        doc.nodes_mut().push(value_node);

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

#[derive(Clone, Debug)]
pub struct PathParamBody {
    pub value: String,
    pub desc: Option<String>,
    pub order: Option<usize>,
    pub disabled: bool,
    pub options: PathParamOptions,
}

impl Default for PathParamBody {
    fn default() -> Self {
        Self {
            value: "".to_string(),
            desc: None,
            order: None,
            disabled: false,
            options: PathParamOptions::default(),
        }
    }
}

impl Into<KdlDocument> for PathParamBody {
    fn into(self) -> KdlDocument {
        let mut doc = KdlDocument::new();

        let mut value_node = KdlNode::new("value");
        value_node.push(KdlEntry::new(self.value));
        doc.nodes_mut().push(value_node);

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

#[derive(Clone, Debug)]
pub struct HeaderParamBody {
    pub value: String,
    pub desc: Option<String>,
    pub order: Option<usize>,
    pub disabled: bool,
    pub options: HeaderOptions,
}

impl Default for HeaderParamBody {
    fn default() -> Self {
        Self {
            value: "".to_string(),
            desc: None,
            order: None,
            disabled: false,
            options: HeaderOptions::default(),
        }
    }
}

impl Into<KdlDocument> for HeaderParamBody {
    fn into(self) -> KdlDocument {
        let mut doc = KdlDocument::new();

        let mut value_node = KdlNode::new("value");
        value_node.push(KdlEntry::new(self.value));
        doc.nodes_mut().push(value_node);

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
pub struct HttpRequestFile {
    pub url: Url,
    pub query_params: HashMap<String, QueryParamBody>,
    pub path_params: HashMap<String, PathParamBody>,
    pub headers: HashMap<String, HeaderParamBody>,
}

impl ToString for HttpRequestFile {
    fn to_string(&self) -> String {
        let mut document = KdlDocument::new();
        let nodes = document.nodes_mut();

        let url_node: KdlNode = self.url.clone().into();
        nodes.push(url_node);

        let mut query_params_node = KdlNode::new(PARAMS_LIT);
        query_params_node.push(KdlEntry::new_prop("type", "query"));
        let mut children = KdlDocument::new();
        for (name, body) in &self.query_params {
            let mut param_node = KdlNode::new(name.to_string());
            param_node.set_children(body.clone().into());
            children.nodes_mut().push(param_node);
        }
        query_params_node.set_children(children);
        nodes.push(query_params_node);

        let mut path_params_node = KdlNode::new(PARAMS_LIT);
        path_params_node.push(KdlEntry::new_prop("type", "path"));
        let mut children = KdlDocument::new();
        for (name, body) in &self.path_params {
            let mut param_node = KdlNode::new(name.clone());
            param_node.set_children(body.clone().into());
            children.nodes_mut().push(param_node);
        }
        path_params_node.set_children(children);
        nodes.push(path_params_node);

        let mut headers_node = KdlNode::new(HEADERS_LIT);
        let mut children = KdlDocument::new();
        for (name, body) in &self.headers {
            let mut header_node = KdlNode::new(name.clone());
            header_node.set_children(body.clone().into());
            children.nodes_mut().push(header_node);
        }
        headers_node.set_children(children);
        nodes.push(headers_node);

        document.autoformat();
        document
            .into_iter()
            .map(|node| node.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
