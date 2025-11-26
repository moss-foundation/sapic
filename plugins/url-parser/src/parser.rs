use joinerror::{Error, OptionExt, Result, bail};
use pest::{Parser, iterators::Pair};
use pest_derive::Parser;
use std::fmt::Debug;

use crate::models::types::{ParsedUrl, ParsedValue, QueryParam, ValueList};

#[derive(Parser)]
#[grammar = "url_grammar.pest"] // relative to src
pub struct UrlParser;

impl UrlParser {
    pub fn parse_url(url: &str) -> Result<ParsedUrl> {
        let url = UrlParser::parse(Rule::url, url)
            .map_err(|e| Error::new::<()>(format!("failed to part url: {}", e.to_string())))?
            .next()
            .ok_or_join_err::<()>("no url is matched")?;

        let mut scheme_part = Vec::new();
        let mut host_part = Vec::new();
        let mut path_part = Vec::new();
        let mut query_part = Vec::new();
        let mut fragment_part = None::<ValueList>;

        for part in url.into_inner() {
            let rule = part.as_rule();
            match rule {
                Rule::scheme_part => {
                    scheme_part = parse_scheme_part(part)?;
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

        // Construct raw url
        let mut raw = Vec::new();
        raw.extend(scheme_part.iter().cloned());
        raw.push(ParsedValue::String("://".to_string()));
        raw.extend(host_part.iter().cloned());

        if path_part.len() > 0 {
            raw.push(ParsedValue::String("/".to_string()));
            raw.extend(path_part.iter().cloned());
        }

        if query_part.len() > 0 {
            raw.push(ParsedValue::String("?".to_string()));

            // The first query param doesn't need a `&` prefix
            // e.g. ?query1=1&query2=2
            let mut should_add_separator = false;
            for query_param in query_part.iter().cloned() {
                if should_add_separator {
                    raw.push(ParsedValue::String("&".to_string()));
                } else {
                    should_add_separator = true;
                }
                raw.extend(query_param.key);
                if let Some(value) = query_param.value {
                    raw.push(ParsedValue::String("=".to_string()));
                    raw.extend(value.into_iter());
                }
            }
        }

        if let Some(fragment) = &fragment_part {
            raw.push(ParsedValue::String("#".to_string()));
            raw.extend(fragment.iter().cloned());
        }

        Ok(ParsedUrl {
            scheme_part,
            host_part,
            path_part,
            query_part,
            fragment_part,
            raw,
        })
    }
}

// {{ident}}
fn parse_var(var: Pair<Rule>) -> Result<ParsedValue> {
    let var_ident = var
        .into_inner()
        .next()
        .ok_or_join_err::<()>("failed to parse variable")?
        .as_str();
    Ok(ParsedValue::Variable(var_ident.to_owned()))
}

// :ident
fn parse_path_var(var: Pair<Rule>) -> Result<ParsedValue> {
    let var_ident = var
        .into_inner()
        .next()
        .ok_or_join_err::<()>("failed to parse variable")?
        .as_str();
    Ok(ParsedValue::PathVariable(var_ident.to_owned()))
}

fn parse_scheme_part(scheme: Pair<Rule>) -> Result<ValueList> {
    let mut scheme_parts = Vec::new();
    for pair in scheme.into_inner() {
        let part = match pair.as_rule() {
            Rule::var => parse_var(pair)?,
            Rule::scheme_literal => ParsedValue::String(pair.as_str().to_owned()),
            _ => bail!("Invalid scheme part: `{}`", pair.as_str()),
        };
        scheme_parts.push(part);
    }
    Ok(scheme_parts)
}

fn parse_host_part(host: Pair<Rule>) -> Result<ValueList> {
    let mut host_parts = Vec::new();
    for pair in host.into_inner() {
        let part = match pair.as_rule() {
            Rule::var => parse_var(pair)?,
            Rule::host_raw => ParsedValue::String(pair.as_str().to_string()),
            _ => bail!("Invalid host part: `{}`", pair.as_str()),
        };
        host_parts.push(part);
    }

    Ok(host_parts)
}

fn parse_path_part(part: Pair<Rule>) -> Result<ValueList> {
    let mut path_parts = Vec::new();
    for pair in part.into_inner() {
        let part = match pair.as_rule() {
            Rule::path_var => parse_path_var(pair)?,
            Rule::path_raw => ParsedValue::String(pair.as_str().to_string()),
            _ => bail!("Invalid path part: `{}`", pair.as_str()),
        };
        path_parts.push(part);
    }
    Ok(path_parts)
}

fn parse_query_param(param: Pair<Rule>) -> Result<QueryParam> {
    let param_parts = param.into_inner().collect::<Vec<_>>();

    let query_key_pair = param_parts
        .get(0)
        .ok_or_join_err::<()>("failed to parse query parameter key")?;

    let mut key = Vec::new();
    for pair in query_key_pair.clone().into_inner() {
        let part = match pair.as_rule() {
            Rule::var => parse_var(pair)?,
            Rule::query_raw => ParsedValue::String(pair.as_str().to_string()),
            _ => bail!("Invalid query parameter key part: `{}`", pair.as_str()),
        };
        key.push(part);
    }

    let query_value_pair = param_parts.get(1);
    if query_value_pair.is_none() {
        return Ok(QueryParam { key, value: None });
    }

    let mut value = Vec::new();
    for pair in query_value_pair.unwrap().clone().into_inner() {
        let part = match pair.as_rule() {
            Rule::var => parse_var(pair)?,
            Rule::query_raw => ParsedValue::String(pair.as_str().to_string()),
            _ => bail!("Invalid query parameter value part: `{}`", pair.as_str()),
        };
        value.push(part);
    }

    if value.is_empty() {
        // Handle cases where query value is an empty string, like ?q=
        value.push(ParsedValue::String("".to_string()));
    }

    Ok(QueryParam {
        key,
        value: Some(value),
    })
}

fn parse_query_part(part: Pair<Rule>) -> Result<Vec<QueryParam>> {
    let mut params = Vec::new();
    for query_param in part.into_inner() {
        params.push(parse_query_param(query_param)?);
    }

    Ok(params)
}

fn parse_fragment_part(part: Pair<Rule>) -> Result<Option<ValueList>> {
    let mut fragment_parts: ValueList = Vec::new();
    for pair in part.into_inner() {
        let part = match pair.as_rule() {
            Rule::var => parse_var(pair)?,
            Rule::fragment_raw => ParsedValue::String(pair.as_str().to_string()),
            _ => bail!("Invalid fragment part: `{}`", pair.as_str()),
        };
        fragment_parts.push(part);
    }
    if fragment_parts.is_empty() {
        Ok(None)
    } else {
        Ok(Some(fragment_parts))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::UrlParser;

    #[test]
    fn it_works() {
        let input = "{{test0}}://api-{{env}}.sapic.dev/path/:test1/api/users?limit=10&sort=prefix-{{test2}}#";

        let result = UrlParser::parse_url(input).unwrap();
        dbg!(&result);
    }

    // #[test]
    // fn example() {
    //     let input = "https://{{var1}}.example.com/:var2/api/users?limit=10&sort={{var3}}#top";
    //
    //     let output = ParsedUrl {
    //         scheme_part: Some(ParsedValue::String("https".to_string())),
    //         host_part: vec![
    //             ParsedValue::Variable("var1".to_string()),
    //             ParsedValue::String("example".to_string()),
    //             ParsedValue::String("com".to_string()),
    //         ],
    //         path_part: vec![
    //             ParsedValue::Variable("var2".to_string()),
    //             ParsedValue::String("api".to_string()),
    //             ParsedValue::String("users".to_string()),
    //         ],
    //         query_part: vec![
    //             (
    //                 "limit".to_string(),
    //                 Some(ParsedValue::String("10".to_string())),
    //             ),
    //             (
    //                 "sort".to_string(),
    //                 Some(ParsedValue::Variable("var3".to_string())),
    //             ),
    //         ],
    //         fragment_part: Some(ParsedValue::String("top".to_string())),
    //     };
    //
    //     assert_eq!(UrlParser::parse_url(input).unwrap(), output);
    // }
}
