use std::collections::HashMap;
use lsp_types::*;

mod types;
mod functions;
mod scope;
mod hint;
mod variables;
mod render_modes;
pub use variables::*;
pub use types::*;
pub use functions::*;
pub use scope::*;
pub use hint::*;
pub use render_modes::*;

use crate::{get_byte_offset_from_position, interpreter::{evaluate_top_level_node, EvaluateError}, lexer::{Token, TokenStream}, nodes::TopLevelNode, parse_tokens, source_code::SourceDocument};

pub struct Memory {
    pub root_dir: Option<String>,
    pub shader_type: ShaderType,
    pub valid_render_modes: HashMap<String, String>,
    pub builtin_types: HashMap<String, BuiltinTypeInfo>,
    pub functions: HashMap<String, FunctionInfo>,
    pub hints: HashMap<String, HintInfo>,
    pub structs: HashMap<String, StructInfo>,
    pub scopes: ScopeList,

    source: SourceDocument
}
impl Memory {
    pub fn new(source_str: &str, root_dir: Option<String>) -> Self {
        let source = SourceDocument::new(source_str);
        let mut scopes = ScopeList::new();
        if root_dir.is_some() {
            scopes.extend(variable_builtins());
        }
        let (functions, hints) = if root_dir.is_none() {
            (HashMap::new(), HashMap::new())
        } else {
            (make_builtin_functions(), make_builtin_hints())
        };
        
        Memory {
            root_dir,
            shader_type: ShaderType::Spatial,
            valid_render_modes: HashMap::new(),
            builtin_types: make_builtin_types(),
            functions,
            hints,
            structs: HashMap::new(),
            scopes,
            source
        }
    }

    pub fn evaluate(&mut self, top_levels: Vec<TopLevelNode>) -> &Vec<Diagnostic> {
        let mut scopes = ScopeList::new();
        if self.root_dir.is_some() {
            scopes.extend(variable_builtins());
        }
        let functions = if self.root_dir.is_none() { HashMap::new() }
        else { make_builtin_functions() };
        self.scopes = scopes;
        self.functions = functions;

        for top_level in top_levels {
            _ = evaluate_top_level_node(top_level, self);
        }

        self.source.get_diagnostics()
    }

    pub fn evaluate_new(&mut self, cursor: Option<Position>) -> &Vec<Diagnostic> {
        let mut stream = TokenStream::new(self.source.get_code(), cursor);
        let tree = parse_tokens(&mut stream);

        let mut diagnostics = stream.get_source().get_diagnostics().clone();
        self.source.add_diagnostics(diagnostics);
        self.evaluate(tree)
    }


    pub fn get_builtin_types(&self, scope: usize) -> Vec<CompletionItem> {
        self.builtin_types
            .iter()
            .filter_map(|(name, info)| {
                if scope.clone() == 0 || info.used_anywhere {
                    Some(CompletionItem {
                        label: name.clone(),
                        kind: Some(CompletionItemKind::TYPE_PARAMETER),
                        ..Default::default()
                    })
                } else { None }
            })
            .collect()
    }

    pub fn get_hints(&self, ty: TypeInfo) -> Vec<CompletionItem> {
        self.hints.iter()
            .filter_map(|(name, info)| {
                let insert_text = if info.num_arguments.iter().any(|&x| x > 0) { "($0)" }
                else { "" };
                if info.type_info.contains(&ty) {
                    Some(CompletionItem {
                        label: name.clone(),
                        kind: None,
                        insert_text_format: Some(InsertTextFormat::SNIPPET),
                        insert_text: Some(format!("{}{}", &name, insert_text)),
                        ..Default::default()
                    })
                } else {
                    None
                }
            })
            .collect()


    }

    pub fn get_structs(&self) -> Vec<CompletionItem> {
        self.structs.keys().map(|x| CompletionItem {
            label: x.to_string(),
            kind: Some(CompletionItemKind::STRUCT),
            ..Default::default()
        }).collect()
    }
    
    pub fn get_functions(&self, cursor: Position, is_const: bool) -> Vec<CompletionItem> {
        self.functions
            .iter()
            .filter_map(|(name, info)| {
                if !is_const || info.is_const {
                    Some(CompletionItem {
                        label: format!("{}()", &name),
                        kind: Some(CompletionItemKind::FUNCTION),
                        insert_text_format: Some(InsertTextFormat::SNIPPET),
                        insert_text: Some(format!("{}($0)", &name)),
                        ..Default::default()
                    })
                } else { None }
            })
            .collect()
    }

    pub fn get_variables(&self, scope: usize, is_const: bool) -> Vec<CompletionItem>{
        self.scopes.collect_scopes_from(scope)
            .iter()
            .flat_map(|x| x.iter())
            .filter_map(|(name, info)| {
                if !is_const || info.is_const {
                    Some(CompletionItem {
                        label: name.clone(),
                        kind: Some(CompletionItemKind::VARIABLE),
                        ..Default::default()
                    })
                } else { None }
            })
            .collect()
    }

    pub fn is_id_in_use(&self, id: &str) -> bool {
        self.builtin_types.contains_key(id) 
        || self.functions.contains_key(id)
        || self.structs.contains_key(id)
        || self.scopes.collect_scopes().iter().any(|scope| scope.contains_key(id)) 
    }

    pub fn is_id_valid_type(&self, id: &str) -> bool {
        self.builtin_types.contains_key(id) 
        || self.structs.contains_key(id)
    }

    pub fn get_token_text(&self, token: Token) -> String {
        token.text(self.source.get_code())
    }

    pub fn get_source(&self) -> &SourceDocument {
        &self.source
    }
    
    pub fn alert_error(&mut self, msg: &str, range: Range) -> EvaluateError {
        let message = format!("Syntax Error: {}", msg); 
        self.source.push_error(msg, range, EvaluateError::SemanticsError)
    }

    pub fn fetch_gdshaderinc_files(&self, root_path: &str) -> Vec<String> {
        let mut result = Vec::new();

        let walker = walkdir::WalkDir::new(root_path).into_iter();
        for entry in walker.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() && path.extension().unwrap_or_default() == "gdshaderinc" {
                if let Some(path) = path.to_str() {
                    let better_path = path.replace(root_path, "res://");
                    result.push(better_path);
                }
            }
        }
    
        result
    }

    pub fn apply_change(&mut self, change: TextDocumentContentChangeEvent) {
        self.source.apply_change(change);
    }
}

#[derive(Clone, Debug)]
pub struct ValueInfo {
    pub ty: TypeInfo,
    pub editable: bool,
    pub is_const: bool,
    pub range: Option<Range>,
    pub description: Option<String>
}

#[derive(Debug)]
pub struct StructInfo {
    pub fields: Vec<StructField>,
    pub range: Range 
}

#[derive(Debug)]
pub struct StructField {
    pub name: String,
    pub ty: TypeInfo,
    pub range: Range, 
}

#[derive(Debug)]
pub enum ShaderType {
    Spatial,
    CanvasItem,
    Particles,
    Fog,
    Sky
}

