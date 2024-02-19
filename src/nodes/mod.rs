use lsp_types::Range;
use crate::{lexer::{Token, TokenError}, memory::TypeInfo};

mod top_level_nodes;
mod expression_node;
mod statement_node;
pub use top_level_nodes::*;
pub use expression_node::*;
pub use statement_node::*;

pub type ValueResult = Result<ValueNode, TokenError>;

#[derive(Clone, Debug)]
pub struct ValueNode {
    pub identifier: Token,
    pub type_node: TypeNode, 
    pub range: Range
}

pub type TypeResult = Result<TypeNode, TokenError>;

#[derive(Clone, Debug)]
pub struct TypeNode {
    pub info: TypeInfo,
    pub range: Range
}

