use std::collections::HashMap;
use lsp_types::{Position, Range};

use crate::lexer::ExtraRange;

use super::{TypeInfo, ValueInfo};

#[derive(Clone, PartialEq)]
pub struct FunctionReturn {
    pub expected: Option<TypeInfo>,
    pub returned: Option<TypeInfo>
}

#[derive(Clone, PartialEq)]
pub enum ScopeType {
    TopLevel,
    Block,
    Loop,
    Function(Box<FunctionReturn>)
}

pub struct Scope {
    pub scope_type: ScopeType,
    pub parent: usize,
    pub range: Range,
    values: HashMap<String, ValueInfo>
} 

pub struct ScopeList {
    pub scopes: Vec<Scope>,
    current: usize
}

impl ScopeList {
    pub fn new() -> Self {
        Self {
            scopes: vec![
                Scope {
                    parent: 0,
                    range: Range::default(),
                    scope_type: ScopeType::TopLevel,
                    values: HashMap::new(),
                }
            ],
            current: 0
        }
    }
    pub fn insert(&mut self, k: String, v: ValueInfo) {
        self.scopes[self.current].values.insert(k, v);
    }
    pub fn extend(&mut self, extra: Vec<(String, ValueInfo)>) {
        self.scopes[self.current].values.extend(extra.into_iter())
    }
    pub fn force_scope(&mut self, scope: usize) {
        self.current = scope;
    }
    pub fn scope_type(&mut self) -> ScopeType {
        self.scopes[self.current].scope_type.clone()
    }
    pub fn enter_scope(&mut self, scope_type: ScopeType, range: Range) {
        self.scopes.push(Scope {
            parent: self.current,
            range,
            scope_type,
            values: HashMap::new()
        });
        self.current = self.scopes.len()-1;
    }
    /// True means we succeeded.
    pub fn leave_scope(&mut self) -> bool {
        if self.current == 0 {
            false 
        } else {
            self.current = self.scopes[self.current].parent;
            true
        }
    }

    pub fn get_expected_return_type(&self) -> Option<TypeInfo> {
        let mut current = self.current;
        while current != 0 {
            if let ScopeType::Function(x) = &self.scopes[current].scope_type {
                return x.expected.clone();
            }
            current = self.scopes[current].parent;
        }
        None
    }

    pub fn set_actual_return_type(&mut self, return_type: Option<TypeInfo>) {
        let mut current = self.current;
        while current != 0 {
            if let ScopeType::Function(function_return) = &self.scopes[current].scope_type {
                let new_function_return = Box::new(FunctionReturn {
                    expected: function_return.expected.clone(),
                    returned: return_type.clone() 
                });
                self.scopes[current].scope_type = ScopeType::Function(new_function_return);
            }
            current = self.scopes[current].parent;
        }
    }

    pub fn assert_returned(&mut self) -> bool {
        let mut current = self.current;
        while current != 0 {
            if let ScopeType::Function(function_return) = &self.scopes[current].scope_type {
                return function_return.expected.is_some() && function_return.returned.is_some()
                    || function_return.returned.is_none() && function_return.expected.is_none()
            }
            current = self.scopes[current].parent;
        }
        false
    }

    pub fn collect_scopes(&self) -> Vec<&HashMap<String, ValueInfo>> {
        self.collect_scopes_from(self.current)
    }

    pub fn find_scope_from_position(&self, position: Position) -> usize {
        self.scopes.iter().enumerate().find_map(|(i,scope)| {
            if scope.range.contains_position(position) {
                Some(i)
            } else { None }
        }).map_or(0, |x| x)

    }
    
    pub fn collect_scopes_from(
        &self,
        location: usize
    ) -> Vec<&HashMap<String, ValueInfo>> {
        let mut indices = vec![0];
        let mut current = location;

        while current != 0 {
            indices.push(current);
            if let Some(idx) = self.scopes.get(current) {
                current = idx.parent;
            } else {
                break;
            }
        };
        self.scopes.iter().enumerate().filter_map(move |(idx, scope)| {
            if indices.contains(&idx) { Some(&scope.values) } else { None }
        }).collect()
    }
}
