use std::collections::HashMap;
use std::path::PathBuf;
use std::mem;
use crate::{
    kdl::tokens::{BODY_LIT, RAW_STRING_INDENT, RAW_STRING_PREFIX, RAW_STRING_SUFFIX},
    models::types
};
use kdl::{KdlDocument, KdlEntry, KdlIdentifier, KdlNode};


#[derive(Clone, Debug, PartialEq)]
pub enum RequestBody {
    Raw(RawBodyType),
    FormData(HashMap<String, FormDataBody>),
    UrlEncoded(HashMap<String, UrlEncodedBody>),
    Binary(PathBuf)
}
#[rustfmt::skip]
impl From<types::request_types::RequestBody> for RequestBody {
    fn from(body: types::request_types::RequestBody) -> Self {
        match body {
            types::request_types::RequestBody::Raw(raw_body) => RequestBody::Raw(RawBodyType::from(raw_body)),
            types::request_types::RequestBody::Binary(path) => RequestBody::Binary(PathBuf::from(path)),
            types::request_types::RequestBody::FormData(form_data) => map_form_data_to_kdl(form_data),
            types::request_types::RequestBody::UrlEncoded(url_encoded) => map_url_encoded_to_kdl(url_encoded),
        }
    }
}

fn map_form_data_to_kdl(items: Vec<types::request_types::FormDataItem>) -> crate::kdl::body::RequestBody {
    let map =
        items
            .into_iter()
            .map(|mut item| (mem::take(&mut item.key), FormDataBody {
                value: match item.value {
                    types::request_types::FormDataValue::Text(s) => crate::kdl::body::FormDataValue::Text(s),
                    types::request_types::FormDataValue::File(s) => crate::kdl::body::FormDataValue::File(PathBuf::from(s))
                },
                desc: mem::take(&mut item.desc),
                order: mem::take(&mut item.order),
                disabled: mem::take(&mut item.disabled),
                options: FormDataOptions {
                    propagate: mem::take(&mut item.options.propagate)
                }
            }))
            .collect::<HashMap<_, _>>();
    crate::kdl::body::RequestBody::FormData(map)
}

fn map_url_encoded_to_kdl(items: Vec<types::request_types::UrlEncodedItem>) -> crate::kdl::body::RequestBody {
    let map =
        items
            .into_iter()
            .map(|mut item| (mem::take(&mut item.key), UrlEncodedBody {
                value: mem::take(&mut item.value),
                desc: mem::take(&mut item.desc),
                order: mem::take(&mut item.order),
                disabled: mem::take(&mut item.disabled),
                options: UrlEncodedOptions {
                    propagate: mem::take(&mut item.options.propagate)
                }
            }))
            .collect::<HashMap<_, _>>();
    crate::kdl::body::RequestBody::UrlEncoded(map)
}

#[derive(Clone, Debug, PartialEq)]
pub enum RawBodyType {
    Text(String),
    Json(String),
    Html(String),
    Xml(String),
}

#[rustfmt::skip]
impl From<types::request_types::RawBodyType> for RawBodyType {
    fn from(value: types::request_types::RawBodyType) -> Self {
        match value {
            types::request_types::RawBodyType::Text(text) => RawBodyType::Text(text),
            types::request_types::RawBodyType::Json(json) => RawBodyType::Json(json),
            types::request_types::RawBodyType::Html(html) => RawBodyType::Html(html),
            types::request_types::RawBodyType::Xml(xml) => RawBodyType::Xml(xml),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FormDataBody {
    pub value: FormDataValue,
    pub desc: Option<String>,
    pub order: Option<usize>,
    pub disabled: bool,
    pub options: FormDataOptions
}

impl Default for FormDataBody {
    fn default() -> Self {
        Self {
            value: FormDataValue::Text("".to_string()),
            desc: None,
            order: None,
            disabled: false,
            options: FormDataOptions::default()
        }
    }
}

impl Into<KdlDocument> for FormDataBody {
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
pub enum FormDataValue {
    Text(String),
    File(PathBuf),
}


#[derive(Clone, Debug, PartialEq)]
pub struct FormDataOptions {
    pub propagate: bool,
}

impl Default for FormDataOptions {
    fn default() -> Self {
        Self {
            propagate: false,
        }
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
pub struct UrlEncodedBody {
    pub value: String,
    pub desc: Option<String>,
    pub order: Option<usize>,
    pub disabled: bool,
    pub options: UrlEncodedOptions
}

impl Default for UrlEncodedBody {
    fn default() -> Self {
        Self {
            value: "".to_string(),
            desc: None,
            order: None,
            disabled: false,
            options: UrlEncodedOptions::default()
        }
    }
}

impl Into<KdlDocument> for UrlEncodedBody {
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
        Self {
            propagate: false,
        }
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

fn prepare_form_data_body_node(node: &mut KdlNode, form_data: HashMap<String, FormDataBody>) {
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

fn prepare_urlencoded_body_node(node: &mut KdlNode, url_encoded: HashMap<String, UrlEncodedBody>) {
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
    node.push(KdlEntry::new_prop("path", path.to_string_lossy().to_string()));
    node.autoformat();
}

impl Into<KdlNode> for RequestBody {
    fn into(self) -> KdlNode {
        let mut node = KdlNode::new(BODY_LIT);
        match self {
            RequestBody::Raw(raw_body) => {
                prepare_raw_body_node(&mut node, raw_body);
            }
            RequestBody::FormData(form_data) => {
                prepare_form_data_body_node(&mut node, form_data);
            }
            RequestBody::UrlEncoded(url_encoded) => {
                prepare_urlencoded_body_node(&mut node, url_encoded);
            }
            RequestBody::Binary(path) => {
                prepare_binary_body_node(&mut node, path);
            }
        }
        node
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;
    use super::*;
    // FIXME: When outputing raw body type, there won't be a space between the type property and `{`
    // It doesn't have any substantive difference, just that the look is a little bit inconsistent.
    // I'm not sure how to fix it without messing up the raw string parsing and writing
    #[test]
    fn test_writing_raw_text() {
        let expected = r###"body type=text{
    #"""
    Raw Text
    """#
}"###;

        let request_body = RequestBody::Raw(RawBodyType::Text("Raw Text".to_string()));
        let node: KdlNode = request_body.into();
        assert_eq!(node.to_string().trim(), expected.to_string().trim());
    }

    #[test]
    fn test_writing_form_data_node() {
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
        let body = FormDataBody {
            value: FormDataValue::Text("value".to_string()),
            ..Default::default()
        };
        form_data.insert("key".to_string(), body);
        let request_body = RequestBody::FormData(
            form_data
        );
        let node: KdlNode = request_body.into();
        assert_eq!(node.to_string().trim(), expected.to_string().trim());
    }

    #[test]
    fn test_writing_url_encoded_node() {
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
        let body = UrlEncodedBody {
            value: "value".to_string(),
            ..Default::default()
        };
        urlencoded.insert("key".to_string(), body);
        let request_body = RequestBody::UrlEncoded(urlencoded);
        let node: KdlNode = request_body.into();
        assert_eq!(node.to_string().trim(), expected.to_string().trim());
    }


    #[test]
    fn test_writing_binary_node() {
        let expected = r###"body type=binary path="path/to/file""###;
        let request_body = RequestBody::Binary(PathBuf::from("path/to/file"));
        let node: KdlNode = request_body.into();
        assert_eq!(node.to_string().trim(), expected.to_string().trim());
    }
}
