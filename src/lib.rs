use interpreter::evaluate_top_level_node;
use lexer::TokenStream;
use lsp_types::{Position, TextDocumentContentChangeEvent};
use memory::Memory;
use nodes::TopLevelNode;
use parser::parse_top_level;
use std::path::Path;
use std::fs;

pub mod lexer;
pub mod source_code;
pub mod nodes;
pub mod parser;
pub mod interpreter;
pub mod memory;
pub mod completion;

pub fn parse_tokens(
    stream: &mut TokenStream,
) -> Vec<TopLevelNode> {
    let mut nodes = vec![];
    loop {
        match parse_top_level(stream) {
            Ok(maybe_node) => {
                if let Some(node) = maybe_node {
                    nodes.push(node);
                }
            }
            _ => break
        }
    }
    nodes
}

pub fn evaluate_tree(
    memory: &mut Memory,
    top_levels: Vec<TopLevelNode>
) {
    for top_level in top_levels {
        _ = evaluate_top_level_node(top_level, memory);
    }
}
pub fn apply_change(source: &mut String, change: &TextDocumentContentChangeEvent){
    if let Some(range) = change.range {
        let start_byte = get_byte_offset_from_position(source, range.start);
        let end_byte = get_byte_offset_from_position(source, range.end);
        let new_text = &change.text;
        if start_byte <= source.len() && end_byte <= source.len() && start_byte <= end_byte {
            source.replace_range(start_byte..end_byte, new_text);
        }
    }
}

pub fn get_byte_offset_from_position(source: &String, position: Position) -> usize {
    let mut byte_offset = 0;
    for (i, line) in source.lines().enumerate() {
        if i as u32 == position.line {
            byte_offset += position.character as usize;
            break;
        }
        byte_offset += line.len() + 1;
    }
    byte_offset
}

pub fn calculate_new_end_position(start: Position, new_text: &str) -> Position {
    let new_lines = new_text.lines().collect::<Vec<&str>>();
    let line_count = new_lines.len();

    let new_end_line = if line_count > 1 {
        start.line + line_count as u32 - 1
    } else {
        start.line
    };
    let new_end_character = if line_count > 1 {
        new_lines.last().unwrap().len() as u32
    } else {
        start.character + new_text.len() as u32
    };

    Position {
        line: new_end_line,
        character: new_end_character,
    }
}

pub const DID_OPEN: &'static str = "textDocument/didOpen";
pub const DID_CHANGE: &'static str = "textDocument/didChange";
pub const DID_CLOSE: &'static str = "textDocument/didClose";
pub const DID_SAVE: &'static str = "textDocument/didSave";
pub const DID_MOUSE_MOVE: &'static str = "textDocument/didSave";
pub const PUBLISH_DIAGNOSTICS: &'static str = "textDocument/publishDiagnostics";
pub const INITIALIZE: &'static str = "initialize";
pub const SHUTDOWN: &'static str = "shutdown";
pub const EXIT: &'static str = "exit";
pub const COMPLETION: &'static str = "textDocument/completion";
pub const HOVER: &'static str = "textDocument/hover";
pub const DEFINITION: &'static str = "textDocument/definition";

