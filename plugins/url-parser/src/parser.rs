use joinerror::{OptionExt, Result, bail};
use pest::{Parser, iterators::Pair};
use pest_derive::Parser;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::models::types::{ParsedUrl, ParsedValue};

#[derive(Parser)]
#[grammar = "url_grammar.pest"] // relative to src
pub struct UrlParser;

impl UrlParser {
    pub fn parse_url(url: &str) -> Result<ParsedUrl> {
        let url = UrlParser::parse(Rule::url, url).unwrap().next().unwrap();

        let mut scheme_part: Option<ParsedValue> = None;
        let mut host_part = Vec::new();
        let mut path_part = Vec::new();
        let mut query_part = Vec::new();
        let mut fragment_part = None::<ParsedValue>;

        for part in url.into_inner() {
            let rule = part.as_rule();
            match rule {
                Rule::scheme_part => {
                    scheme_part = Some(parse_scheme_part(part)?);
                }
                Rule::host_part => {
                    host_part = parse_host_part(part)?;
                }
                Rule::path_part => {
                    path_part = parse_path_part(part)?;
                }
                Rule::query_part => {
                    query_part = parse_query_part(part)?;
                }
                Rule::fragment_part => {
                    fragment_part = parse_fragment_part(part)?;
                }
                _ => {}
            }
        }

        Ok(ParsedUrl {
            scheme_part,
            host_part,
            path_part,
            query_part,
            fragment_part,
        })
    }
}

fn parse_var(var: Pair<Rule>) -> Result<ParsedValue> {
    let var_ident = var
        .into_inner()
        .next()
        .ok_or_join_err::<()>("failed to parse variable")?
        .as_str();
    Ok(ParsedValue::Variable(var_ident.to_owned()))
}

fn parse_scheme_part(scheme: Pair<Rule>) -> Result<ParsedValue> {
    let scheme_inner = scheme
        .into_inner()
        .next()
        .ok_or_join_err::<()>("failed to parse scheme")?;
    match scheme_inner.as_rule() {
        Rule::scheme_literal => Ok(ParsedValue::String(scheme_inner.as_str().to_owned())),
        Rule::var => parse_var(scheme_inner),
        _ => bail!("Invalid scheme: `{}`", scheme_inner),
    }
}

fn parse_host_piece(piece: Pair<Rule>) -> Result<ParsedValue> {
    let host_piece_inner = piece
        .into_inner()
        .next()
        .ok_or_join_err::<()>("failed to parse host piece")?;
    match host_piece_inner.as_rule() {
        Rule::host_literal => Ok(ParsedValue::String(host_piece_inner.as_str().to_owned())),
        Rule::var => parse_var(host_piece_inner),
        _ => {
            bail!("Invalid host piece: `{}`", host_piece_inner);
        }
    }
}

fn parse_host_part(host: Pair<Rule>) -> Result<Vec<ParsedValue>> {
    let mut host_parts = Vec::new();
    for pair in host.into_inner() {
        host_parts.push(parse_host_piece(pair)?);
    }
    Ok(host_parts)
}

fn parse_path_segment(segment: Pair<Rule>) -> Result<ParsedValue> {
    let path_segment_inner = segment
        .into_inner()
        .next()
        .ok_or_join_err::<()>("failed to parse path segment")?;
    match path_segment_inner.as_rule() {
        Rule::path_text => Ok(ParsedValue::String(path_segment_inner.as_str().to_owned())),
        Rule::var => parse_var(path_segment_inner),
        _ => bail!("Invalid path segment: `{}`", path_segment_inner),
    }
}

fn parse_path_part(path: Pair<Rule>) -> Result<Vec<ParsedValue>> {
    let mut path_segments = Vec::new();
    for path_segment in path.into_inner() {
        path_segments.push(parse_path_segment(path_segment)?);
    }
    Ok(path_segments)
}

fn parse_query_param(param: Pair<Rule>) -> Result<(String, ParsedValue)> {
    let mut param_parts = param.into_inner().collect::<Vec<_>>();

    let query_key_pair = param_parts
        .get(0)
        .ok_or_join_err::<()>("failed to parse query parameter key")?;
    let query_key = query_key_pair.as_str().to_owned();

    let query_value_inner = param_parts
        .get(1)
        .and_then(|p| p.clone().into_inner().next())
        .ok_or_join_err::<()>("failed to parse query parameter value")?;

    let query_value = match query_value_inner.as_rule() {
        Rule::query_text => ParsedValue::String(query_value_inner.as_str().to_owned()),
        Rule::var => parse_var(query_value_inner)?,
        _ => bail!("Invalid query parameter value: `{}`", query_value_inner),
    };

    Ok((query_key, query_value))
}

fn parse_query_part(part: Pair<Rule>) -> Result<Vec<(String, ParsedValue)>> {
    let mut query_params = Vec::new();
    for query_param in part.into_inner() {
        query_params.push(parse_query_param(query_param)?);
    }
    Ok(query_params)
}

fn parse_fragment_part(fragment: Pair<Rule>) -> Result<Option<ParsedValue>> {
    let fragment_part_inner = fragment
        .into_inner()
        .next()
        .ok_or_join_err::<()>("failed to parse fragment part")?;
    match fragment_part_inner.as_rule() {
        Rule::fragment_text => Ok(Some(ParsedValue::String(
            fragment_part_inner.as_str().to_owned(),
        ))),
        Rule::var => Ok(Some(parse_var(fragment_part_inner)?)),
        _ => bail!("Invalid fragment part: `{}`", fragment_part_inner),
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        models::types::ParsedUrl,
        parser::{ParsedValue, UrlParser},
    };

    #[test]
    fn it_works() {
        let input =
            "{{test0}}://test1-example.com/{{test2}}/api/users?limit=10&sort={{test3}}#{{test4}}";

        let result = UrlParser::parse_url(input).unwrap();

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
                ParsedValue::Variable("var2".to_string()),
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

        assert_eq!(UrlParser::parse_url(input).unwrap(), output);
    }
}
