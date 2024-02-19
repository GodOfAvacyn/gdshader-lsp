use std::{any::Any, collections::{HashMap, HashSet}};
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

use crate::{interpreter::EvaluateError, lexer::Token, source_code::SourceDocument};

pub struct Memory {
    pub shader_type: ShaderType,
    pub valid_render_modes: HashMap<String, String>,
    /// 'true' means 'this can be used inside functions'.;
    pub builtin_types: HashMap<String, BuiltinTypeInfo>,
    pub functions: HashMap<String, FunctionInfo>,
    pub hints: HashMap<String, HintInfo>,
    pub structs: HashMap<String, StructInfo>,
    pub scopes: ScopeList,
    source: SourceDocument
}
impl Memory {
    pub fn new(source_str: &str) -> Self {
        let source = SourceDocument::new(source_str);
        let mut scopes = ScopeList::new();
        scopes.extend(variable_builtins());
        Memory {
            shader_type: ShaderType::Spatial,
            valid_render_modes: HashMap::new(),
            builtin_types: make_builtin_types(),
            functions: make_builtin_functions(),
            hints: make_builtin_hints(),
            structs: HashMap::new(),
            scopes,
            source
        }
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
        eprintln!("you got variables at {}", scope);
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
        token.text(self.source.get_lines())
    }

    pub fn get_source(&self) -> &SourceDocument {
        &self.source
    }
    
    pub fn alert_error(&mut self, msg: &str, range: Range) -> EvaluateError {
        let message = format!("Syntax Error: {}", msg); 
        self.source.push_error(msg, range, EvaluateError::SemanticsError)
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

