use hcl::{Attribute, Block};
use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilterOp {
    Eq,       // = value
    Regex,    // ~ pattern
    Contains, // ~= substring
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabelIndex {
    pub label: String,
    pub index: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PointerToken {
    Block {
        block_type: String,
        label_indices: Vec<LabelIndex>,
    },
    Attribute(String),
    Wildcard,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pointer {
    pub tokens: Vec<PointerToken>,
}

impl Pointer {
    pub fn new() -> Self {
        Pointer { tokens: Vec::new() }
    }
    pub fn tokens(&self) -> &[PointerToken] {
        &self.tokens
    }
}

#[derive(Parser)]
#[grammar = "hcl/pointer.pest"]
pub struct PointerParser;

pub type ParseError = pest::error::Error<Rule>;

pub fn parse_pointer(input: &str) -> Result<Pointer, ParseError> {
    let mut pointer = Pointer::new();
    let pairs = PointerParser::parse(Rule::pointer, input)?;
    for pair in pairs {
        match pair.as_rule() {
            Rule::token => pointer.tokens.push(parse_token(pair)),
            Rule::EOI | Rule::slash => {}
            _ => unreachable!("Unexpected rule: {:?}", pair.as_rule()),
        }
    }
    Ok(pointer)
}

fn parse_token(pair: Pair<Rule>) -> PointerToken {
    let mut inner = pair.into_inner();
    let first = inner.next().unwrap();

    if first.as_str() == "*" {
        return PointerToken::Wildcard;
    }
    if first.as_rule() == Rule::attr {
        return PointerToken::Attribute(first.as_str().to_string());
    }
    let block_type = first.as_str().to_string();
    let mut label_indices = Vec::new();
    for suffix in inner {
        match suffix.as_rule() {
            Rule::label => {
                let quoted = suffix.into_inner().next().unwrap().as_str();
                let label = &quoted[1..quoted.len() - 1];
                label_indices.push(LabelIndex {
                    label: label.to_string(),
                    index: None,
                });
            }
            Rule::index => {
                let digits = &suffix.as_str()[1..suffix.as_str().len() - 1];
                let idx = digits.parse().unwrap_or(0);
                if let Some(last) = label_indices.last_mut() {
                    last.index = Some(idx);
                }
            }
            _ => {}
        }
    }
    PointerToken::Block {
        block_type,
        label_indices,
    }
}

pub enum Action {
    AddBlock { path: String, block: Block },
    AddAttribute { path: String, attribute: Attribute },
}
