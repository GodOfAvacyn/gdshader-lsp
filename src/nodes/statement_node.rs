use lsp_types::Range;

use crate::lexer::{Token, TokenError};
use super::{expression_node::ExpressionNode, ValueNode};

pub type StatementResult = Result<StatementNode, TokenError>;

#[derive(Clone, Debug)]
pub enum StatementNode {
    VarDeclaration(VarDeclarationNode),
    If(IfNode),
    While(WhileNode),
    For(ForNode),
    Switch(SwitchNode),
    Expression(Box<ExpressionNode>),
    Continue(Token),
    Break(Token),
    Return(ReturnNode),
    Block(BlockNode),
}

#[derive(Clone, Debug)]
pub struct BlockNode {
    pub range: Range,
    pub statements: Vec<StatementNode>
}

#[derive(Clone, Debug)]
pub struct VarDeclarationNode {
    pub keyword: Option<Token>,
    pub value: Box<ValueNode>,
    pub expression: Option<Box<ExpressionNode>>,
    pub is_const: bool
}

#[derive(Clone, Debug)]
pub struct IfNode {
    pub keyword: Token,
    pub condition: Box<ExpressionNode>,
    pub action: Box<StatementNode>,
    pub alternate: Option<ElseNode>
}

#[derive(Clone, Debug)]
pub struct ElseNode {
    pub keyword: Token,
    pub action: Box<StatementNode>
}

#[derive(Clone, Debug)]
pub struct WhileNode {
    pub keyword: Token,
    pub condition: Box<ExpressionNode>,
    pub action: Box<StatementNode>
}

#[derive(Clone, Debug)]
pub struct ForNode {
    pub keyword: Token,
    pub initializer: Box<StatementNode>,
    pub condition: Box<ExpressionNode>,
    pub update: Box<ExpressionNode>,
    pub action: Box<StatementNode>
}

#[derive(Clone, Debug)]
pub struct SwitchNode {
    pub keyword: Token,
    pub condition: Box<ExpressionNode>,
    pub cases: Vec<SwitchCaseNode>
}

#[derive(Clone, Debug)]
pub struct SwitchCaseNode {
    pub keyword: Token,
    pub statements: Vec<StatementNode>
}

#[derive(Clone, Debug)]
pub struct ReturnNode {
    pub keyword: Token,
    pub expression: Option<Box<ExpressionNode>>
}



