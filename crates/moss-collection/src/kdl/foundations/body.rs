use kdl::{KdlDocument, KdlEntry, KdlIdentifier, KdlNode};
use std::{collections::HashMap, mem, path::PathBuf};

use crate::{
    kdl::tokens::{BODY_LIT, RAW_STRING_INDENT, RAW_STRING_PREFIX, RAW_STRING_SUFFIX},
    models::types::{FormDataItem, FormDataValue, RawBodyType, RequestBody, UrlEncodedItem},
};

#[derive(Clone, Debug, PartialEq)]
pub enum RequestBodyBlock {
    Raw(RawBodyType),
    FormData(HashMap<String, FormDataBodyItem>),
    UrlEncoded(HashMap<String, UrlEncodedBodyItem>),
    Binary(PathBuf),
}

#[rustfmt::skip]
impl From<RequestBody> for RequestBodyBlock {
    fn from(body: RequestBody) -> Self {
        match body {
            RequestBody::Raw(raw_body) => RequestBodyBlock::Raw(RawBodyType::from(raw_body)),
            RequestBody::Binary(path) => RequestBodyBlock::Binary(PathBuf::from(path)),
            RequestBody::FormData(form_data) => map_form_data_to_kdl(form_data),
            RequestBody::UrlEncoded(url_encoded) => map_url_encoded_to_kdl(url_encoded),
        }
    }
}

fn map_form_data_to_kdl(items: Vec<FormDataItem>) -> crate::kdl::body::RequestBodyBlock {
    let map = items
        .into_iter()
        .map(|mut item| {
            (
                mem::take(&mut item.key),
                FormDataBodyItem {
                    value: match item.value {
                        FormDataValue::Text(s) => crate::kdl::body::FormDataValue::Text(s),
                        FormDataValue::File(s) => {
                            crate::kdl::body::FormDataValue::File(PathBuf::from(s))
                        }
                    },
                    desc: mem::take(&mut item.desc),
                    order: mem::take(&mut item.order),
                    disabled: mem::take(&mut item.disabled),
                    options: FormDataOptions {
                        propagate: mem::take(&mut item.options.propagate),
                    },
                },
            )
        })
        .collect::<HashMap<_, _>>();
    crate::kdl::body::RequestBodyBlock::FormData(map)
}

fn map_url_encoded_to_kdl(items: Vec<UrlEncodedItem>) -> crate::kdl::body::RequestBodyBlock {
    let map = items
        .into_iter()
        .map(|mut item| {
            (
                mem::take(&mut item.key),
                UrlEncodedBodyItem {
                    value: mem::take(&mut item.value),
                    desc: mem::take(&mut item.desc),
                    order: mem::take(&mut item.order),
                    disabled: mem::take(&mut item.disabled),
                    options: UrlEncodedOptions {
                        propagate: mem::take(&mut item.options.propagate),
                    },
                },
            )
        })
        .collect::<HashMap<_, _>>();
    crate::kdl::body::RequestBodyBlock::UrlEncoded(map)
}

#[derive(Clone, Debug, PartialEq)]
pub struct FormDataBodyItem {
    pub value: FormDataValue,
    pub desc: Option<String>,
    pub order: Option<usize>,
    pub disabled: bool,
    pub options: FormDataOptions,
}

impl Default for FormDataBodyItem {
    fn default() -> Self {
        Self {
            value: FormDataValue::Text("".to_string()),
            desc: None,
            order: None,
            disabled: false,
            options: FormDataOptions::default(),
        }
    }
}

impl Into<KdlDocument> for FormDataBodyItem {
    fn into(self) -> KdlDocument {
        let (typ, value) = match self.value {
            FormDataValue::Text(s) => ("text", s),
            FormDataValue::File(s) => ("file", s.to_string_lossy().to_string()),
        };

        let mut doc = KdlDocument::new();

        let mut type_node = KdlNode::new("type");
        type_node.push(KdlEntry::new(typ));
        doc.nodes_mut().push(type_node);

        let mut value_node = KdlNode::new("value");
        value_node.push(KdlEntry::new(value));
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

#[derive(Clone, Debug, PartialEq)]
pub struct FormDataOptions {
    pub propagate: bool,
}

impl Default for FormDataOptions {
    fn default() -> Self {
        Self { propagate: false }
    }
}

impl Into<KdlNode> for FormDataOptions {
    fn into(self) -> KdlNode {
        let mut node = KdlNode::new("options");
        let mut children = KdlDocument::new();
        let mut propagate_node = KdlNode::new("propagate");
        propagate_node.push(KdlEntry::new(self.propagate));
        children.nodes_mut().push(propagate_node);
        node.set_children(children);
        node.autoformat();
        node
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct UrlEncodedBodyItem {
    pub value: String,
    pub desc: Option<String>,
    pub order: Option<usize>,
    pub disabled: bool,
    pub options: UrlEncodedOptions,
}

impl Default for UrlEncodedBodyItem {
    fn default() -> Self {
        Self {
            value: "".to_string(),
            desc: None,
            order: None,
            disabled: false,
            options: UrlEncodedOptions::default(),
        }
    }
}

impl Into<KdlDocument> for UrlEncodedBodyItem {
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

#[derive(Clone, Debug, PartialEq)]
pub struct UrlEncodedOptions {
    pub propagate: bool,
}

impl Default for UrlEncodedOptions {
    fn default() -> Self {
        Self { propagate: false }
    }
}

impl Into<KdlNode> for UrlEncodedOptions {
    fn into(self) -> KdlNode {
        let mut node = KdlNode::new("options");
        let mut children = KdlDocument::new();
        let mut propagate_node = KdlNode::new("propagate");
        propagate_node.push(KdlEntry::new(self.propagate));
        children.nodes_mut().push(propagate_node);
        node.set_children(children);
        node.autoformat();
        node
    }
}

fn format_raw_string(content: &str) -> String {
    format!(
        "{RAW_STRING_PREFIX}\n\
        {content}\n\
        {RAW_STRING_SUFFIX}"
    )
    .lines()
    // Indent the content
    .map(|line| format!("{RAW_STRING_INDENT}{line}"))
    .collect::<Vec<_>>()
    .join("\n")
}

fn prepare_raw_body_node(node: &mut KdlNode, raw_body: RawBodyType) {
    let (typ, content) = match raw_body {
        RawBodyType::Text(s) => ("text", s),
        RawBodyType::Json(s) => ("json", s),
        RawBodyType::Html(s) => ("html", s),
        RawBodyType::Xml(s) => ("xml", s),
    };
    node.push(KdlEntry::new_prop("type", typ));
    let mut children = KdlDocument::new();
    // We have to manually format the output
    // Since the provided `autoformat` method will mess up with escape characters
    let formatted_content = format_raw_string(&content);
    let mut raw_content = KdlIdentifier::from(formatted_content.clone());
    // If we don't do this, the raw string quotes will be incorrectly escaped
    raw_content.set_repr(formatted_content);

    children.nodes_mut().push(KdlNode::new(raw_content));
    node.set_children(children);
}

fn prepare_form_data_body_node(node: &mut KdlNode, form_data: HashMap<String, FormDataBodyItem>) {
    node.push(KdlEntry::new_prop("type", "form-data"));
    let mut children = KdlDocument::new();
    for (key, body) in form_data {
        let mut param_node = KdlNode::new(key.clone());
        param_node.set_children(body.clone().into());
        children.nodes_mut().push(param_node);
    }
    node.set_children(children);
    node.autoformat();
}

fn prepare_urlencoded_body_node(
    node: &mut KdlNode,
    url_encoded: HashMap<String, UrlEncodedBodyItem>,
) {
    node.push(KdlEntry::new_prop("type", "urlencoded"));
    let mut children = KdlDocument::new();
    for (key, body) in url_encoded {
        let mut param_node = KdlNode::new(key.clone());
        param_node.set_children(body.clone().into());
        children.nodes_mut().push(param_node);
    }
    node.set_children(children);
    node.autoformat();
}

fn prepare_binary_body_node(node: &mut KdlNode, path: PathBuf) {
    node.push(KdlEntry::new_prop("type", "binary"));
    node.push(KdlEntry::new_prop(
        "path",
        path.to_string_lossy().to_string(),
    ));
    node.autoformat();
}

impl Into<KdlNode> for RequestBodyBlock {
    fn into(self) -> KdlNode {
        let mut node = KdlNode::new(BODY_LIT);
        match self {
            RequestBodyBlock::Raw(raw_body) => {
                prepare_raw_body_node(&mut node, raw_body);
            }
            RequestBodyBlock::FormData(form_data) => {
                prepare_form_data_body_node(&mut node, form_data);
            }
            RequestBodyBlock::UrlEncoded(url_encoded) => {
                prepare_urlencoded_body_node(&mut node, url_encoded);
            }
            RequestBodyBlock::Binary(path) => {
                prepare_binary_body_node(&mut node, path);
            }
        }
        node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // FIXME: When outputting raw body type, there won't be a space between the type property and `{`
    // It doesn't have any substantive difference, just that the look is a little bit inconsistent.
    // I'm not sure how to fix it without messing up the raw string parsing and writing
    #[test]
    fn writing_raw_text() {
        let expected = r###"body type=text{
    #"""
    Raw Text
    """#
}"###;

        let request_body = RequestBodyBlock::Raw(RawBodyType::Text("Raw Text".to_string()));
        let node: KdlNode = request_body.into();
        assert_eq!(node.to_string().trim(), expected.to_string().trim());
    }

    #[test]
    fn writing_form_data_node() {
        let expected = r###"body type=form-data {
    key {
        type text
        value value
        disabled #false
        options {
            propagate #false
        }
    }
}"###;
        let mut form_data = HashMap::new();
        let body = FormDataBodyItem {
            value: FormDataValue::Text("value".to_string()),
            ..Default::default()
        };
        form_data.insert("key".to_string(), body);
        let request_body = RequestBodyBlock::FormData(form_data);
        let node: KdlNode = request_body.into();
        assert_eq!(node.to_string().trim(), expected.to_string().trim());
    }

    #[test]
    fn writing_url_encoded_node() {
        let expected = r###"body type=urlencoded {
    key {
        value value
        disabled #false
        options {
            propagate #false
        }
    }
}"###;
        let mut urlencoded = HashMap::new();
        let body = UrlEncodedBodyItem {
            value: "value".to_string(),
            ..Default::default()
        };
        urlencoded.insert("key".to_string(), body);
        let request_body = RequestBodyBlock::UrlEncoded(urlencoded);
        let node: KdlNode = request_body.into();
        assert_eq!(node.to_string().trim(), expected.to_string().trim());
    }

    #[test]
    fn writing_binary_node() {
        let expected = r###"body type=binary path="path/to/file""###;
        let request_body = RequestBodyBlock::Binary(PathBuf::from("path/to/file"));
        let node: KdlNode = request_body.into();
        assert_eq!(node.to_string().trim(), expected.to_string().trim());
    }
}
