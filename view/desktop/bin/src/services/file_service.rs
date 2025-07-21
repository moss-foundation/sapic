#![allow(dead_code)]

use moss_fs::FileSystem;
use std::sync::Arc;

use ropey::Rope;
use tree_sitter::{Node, Parser, Query, QueryCursor, StreamingIterator};

pub struct FileService {
    fs: Arc<dyn FileSystem>,
}

impl FileService {
    pub fn new(fs: Arc<dyn FileSystem>) -> Self {
        Self { fs }
    }
}

#[derive(Debug)]
struct Edit {
    start: usize,
    end: usize,
    old: String,
    new: String,
}

struct EditHistory {
    applied: Vec<Edit>,
    undone: Vec<Edit>,
}

impl EditHistory {
    fn new() -> Self {
        EditHistory {
            applied: Vec::new(),
            undone: Vec::new(),
        }
    }

    fn apply(&mut self, rope: &mut Rope, edit: Edit) {
        rope.remove(edit.start..edit.end);
        rope.insert(edit.start, &edit.new);
        self.applied.push(edit);
        self.undone.clear();
    }

    fn undo(&mut self, rope: &mut Rope) {
        if let Some(edit) = self.applied.pop() {
            let start = edit.start;
            let new_end = start + edit.new.chars().count();

            rope.remove(start..new_end);
            rope.insert(start, &edit.old);
            self.undone.push(edit);
        }
    }

    fn redo(&mut self, rope: &mut Rope) {
        if let Some(edit) = self.undone.pop() {
            let start = edit.start;
            let old_end = start + edit.old.chars().count();

            rope.remove(start..old_end);
            rope.insert(start, &edit.new);
            self.applied.push(edit);
        }
    }
}

fn find_value_range(rope: &Rope, key: &str) -> Option<(usize, usize)> {
    let source = rope.to_string();
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_json::LANGUAGE.into())
        .expect("Error loading JSON grammar");
    let tree = parser.parse(&source, None)?;

    let query_source = r#"
    (pair
      key: (string) @key
      value: (_) @value
    )
    "#;
    let query =
        Query::new(&tree_sitter_json::LANGUAGE.into(), query_source).expect("Error parsing query");
    let mut cursor = QueryCursor::new();

    let mut matches = cursor.matches(&query, tree.root_node(), source.as_bytes());
    while let Some(m) = matches.next() {
        let mut key_node: Option<Node> = None;
        let mut value_node: Option<Node> = None;
        for capture in m.captures {
            let name = &query.capture_names()[capture.index as usize];
            match *name {
                "key" => key_node = Some(capture.node),
                "value" => value_node = Some(capture.node),
                _ => {}
            }
        }
        if let (Some(k), Some(v)) = (key_node, value_node) {
            // Get text of key, without quotes
            let text = k.utf8_text(source.as_bytes()).ok()?;
            let trimmed = text.trim_matches('"');
            if trimmed == key {
                // Byte range
                let start_byte = v.start_byte();
                let end_byte = v.end_byte();

                let start_char = rope.byte_to_char(start_byte);
                let end_char = rope.byte_to_char(end_byte);
                return Some((start_char, end_char));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_value_range() {
        let text = r#"
        {
            "foo": "value",
            "key2": "value2"
        }
        "#;

        let mut rope = Rope::from_str(text);
        let mut history = EditHistory::new();

        if let Some((start, end)) = find_value_range(&rope, "foo") {
            let old_val = rope.slice(start..end).to_string();
            let new_val = "\"bar\"".to_string();
            let edit = Edit {
                start,
                end,
                old: old_val,
                new: new_val,
            };
            history.apply(&mut rope, edit);
        }

        println!("After edit:\n{}", rope.to_string());
        history.undo(&mut rope);
        println!("After undo:\n{}", rope.to_string());
        history.redo(&mut rope);
        println!("After redo:\n{}", rope.to_string());
    }
}
