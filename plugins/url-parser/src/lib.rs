mod models;

use pest::{
    Parser,
    iterators::{Pair, Pairs},
};
use pest_derive::Parser;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// TODO: Cleanup grammar comments
#[derive(Parser)]
#[grammar = "url_grammar.pest"] // relative to src
struct MyParser;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ParsedValue {
    String(String),
    Variable(String),
    // TODO: Implement this after updating the grammar for composite segment
    Composite(Vec<ParsedValue>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedUrl {
    pub scheme_part: Option<ParsedValue>,
    pub host_part: Vec<ParsedValue>,
    pub path_part: Vec<ParsedValue>,
    pub query_part: Vec<(String, ParsedValue)>,
    pub fragment_part: Option<ParsedValue>,
}

// {{variable_ident}}
fn parse_var(var: Pair<Rule>) -> ParsedValue {
    let var_ident = var.into_inner().next().unwrap().as_str();
    ParsedValue::Variable(var_ident.to_owned())
}

fn parse_scheme_part(scheme: Pair<Rule>) -> ParsedValue {
    let scheme_inner = scheme.into_inner().next().unwrap();
    match scheme_inner.as_rule() {
        Rule::scheme_literal => ParsedValue::String(scheme_inner.as_str().to_owned()),
        Rule::var => parse_var(scheme_inner),
        _ => unreachable!(),
    }
}

fn parse_host_piece(piece: Pair<Rule>) -> ParsedValue {
    let host_piece_inner = piece.into_inner().next().unwrap();
    match host_piece_inner.as_rule() {
        Rule::host_literal => ParsedValue::String(host_piece_inner.as_str().to_owned()),
        Rule::var => parse_var(host_piece_inner),
        _ => {
            unreachable!()
        }
    }
}

fn parse_host_part(host: Pair<Rule>) -> Vec<ParsedValue> {
    let mut host_parts = Vec::new();
    for pair in host.into_inner() {
        host_parts.push(parse_host_piece(pair));
    }
    host_parts
}

fn parse_path_segment(segment: Pair<Rule>) -> ParsedValue {
    let path_segment_inner = segment.into_inner().next().unwrap();
    match path_segment_inner.as_rule() {
        Rule::path_text => ParsedValue::String(path_segment_inner.as_str().to_owned()),
        Rule::var => parse_var(path_segment_inner),
        _ => unreachable!(),
    }
}

fn parse_path_part(path: Pair<Rule>) -> Vec<ParsedValue> {
    let mut path_segments = Vec::new();
    for path_segment in path.into_inner() {
        path_segments.push(parse_path_segment(path_segment));
    }
    path_segments
}

fn parse_query_param(param: Pair<Rule>) -> (String, ParsedValue) {
    let mut param_parts = param.into_inner();
    let query_key_pair = param_parts.next().unwrap();
    let query_key = query_key_pair.as_str().to_owned();
    let query_value_inner = param_parts.next().unwrap().into_inner().next().unwrap();
    let query_value = match query_value_inner.as_rule() {
        Rule::query_text => ParsedValue::String(query_value_inner.as_str().to_owned()),
        Rule::var => parse_var(query_value_inner),
        _ => unreachable!(),
    };

    (query_key, query_value)
}

fn parse_query_part(part: Pair<Rule>) -> Vec<(String, ParsedValue)> {
    let mut query_params = Vec::new();
    for query_param in part.into_inner() {
        query_params.push(parse_query_param(query_param));
    }
    query_params
}

fn parse_fragment_part(fragment: Pair<Rule>) -> Option<ParsedValue> {
    let fragment_part_inner = fragment.into_inner().next().unwrap();
    match fragment_part_inner.as_rule() {
        Rule::fragment_text => Some(ParsedValue::String(fragment_part_inner.as_str().to_owned())),
        Rule::var => Some(parse_var(fragment_part_inner)),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser;

    use crate::{
        MyParser, ParsedUrl, ParsedValue, Rule, parse_fragment_part, parse_host_part,
        parse_path_part, parse_query_part, parse_scheme_part,
    };

    #[test]
    fn it_works() {
        let input =
            "{{test0}}://test1-example.com/{{test2}}/api/users?limit=10&sort={{test3}}#{{test4}}";

        let url = MyParser::parse(Rule::url, input).unwrap().next().unwrap();

        let mut scheme_part: Option<ParsedValue> = None;
        let mut host_part = Vec::new();
        let mut path_part = Vec::new();
        let mut query_part = Vec::new();
        let mut fragment_part = None::<ParsedValue>;

        for part in url.into_inner() {
            let rule = part.as_rule();
            match rule {
                Rule::scheme_part => {
                    scheme_part = Some(parse_scheme_part(part));
                }
                Rule::host_part => {
                    host_part = parse_host_part(part);
                }
                Rule::path_part => {
                    path_part = parse_path_part(part);
                }
                Rule::query_part => {
                    query_part = parse_query_part(part);
                }
                Rule::fragment_part => {
                    fragment_part = parse_fragment_part(part);
                }
                _ => {}
            }
        }

        let result = ParsedUrl {
            scheme_part,
            host_part,
            path_part,
            query_part,
            fragment_part,
        };

        dbg!(result);
    }

    #[test]
    fn example() {
        let input = "https://{{var1}}.example.com/{{var2}}/api/users?limit=10&sort={{var3}}#top";

        let output = ParsedUrl {
            scheme_part: Some(ParsedValue::String("https".to_string())),
            host_part: vec![
                ParsedValue::Variable("var1".to_string()),
                ParsedValue::String("example".to_string()),
                ParsedValue::String("com".to_string()),
            ],
            path_part: vec![
                ParsedValue::Variable("test2".to_string()),
                ParsedValue::String("api".to_string()),
                ParsedValue::String("users".to_string()),
            ],
            query_part: vec![
                ("limit".to_string(), ParsedValue::String("10".to_string())),
                (
                    "sort".to_string(),
                    ParsedValue::Variable("var3".to_string()),
                ),
            ],
            fragment_part: Some(ParsedValue::String("top".to_string())),
        };

        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    }
}
