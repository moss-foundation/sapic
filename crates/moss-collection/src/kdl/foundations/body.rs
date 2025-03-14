use std::collections::HashMap;
use std::path::PathBuf;
use crate::kdl::tokens::{BODY_LIT, RAW_STRING_INDENT, RAW_STRING_PREFIX, RAW_STRING_SUFFIX};
use kdl::{KdlDocument, KdlEntry, KdlIdentifier, KdlNode};

#[derive(Clone, Debug, PartialEq)]
pub enum RequestBody {
    // TODO: Raw(RawType), Binary, Form, File ...
    Raw(RawBodyType),
    FormData(HashMap<String, FormDataBody>)
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
pub enum RawBodyType {
    Text(String),
    Json(String),
    Html(String),
    Xml(String),
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
        }
        node
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;
    use super::*;
    #[test]
    fn test_writing_form_data_body() {
        let form_data_body = FormDataBody {
            value: FormDataValue::File(PathBuf::from("path/to/file")),
            desc: Some("desc".to_string()),
            order: Some(1),
            disabled: false,
            options: Default::default(),
        };
        let document: KdlDocument = form_data_body.into();
        println!("{}", document.to_string());

    }

    #[test]
    fn test_writing_form_data_node() {
        let mut form_data = HashMap::new();
        let body1 = FormDataBody {
            value: FormDataValue::Text("value1".to_string()),
            ..Default::default()
        };
        let body2 = FormDataBody {
            value: FormDataValue::File(PathBuf::from("value2")),
            ..Default::default()
        };
        form_data.insert("key1".to_string(), body1);
        form_data.insert("key2".to_string(), body2);
        let request_body = RequestBody::FormData(
            form_data
        );
        let node: KdlNode = request_body.into();
        println!("{}", node.to_string());
    }

}
