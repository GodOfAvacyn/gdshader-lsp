use lsp_types::Range;

use crate::lexer::{Token, TokenError};

pub type ExpressionResult = Result<ExpressionNode, TokenError>;

#[derive(Clone, Debug)]
pub enum ExpressionNode {
    Primitive(Token),
    Identifier(Token),
    Unary(UnaryNode),
    Binary(BinaryNode),
    Assignment(AssignmentNode),
    Increment(IncrementNode),
    Paren(Box<ExpressionNode>),
    Conditional(ConditionalNode),
    Call(CallNode),
    ArrayAccess(ArrayAccessNode),
    MemberAccess(MemberAccessNode),
    ArrayLiteral(Vec<ExpressionNode>)
}
impl ExpressionNode {
    pub fn range(&self) -> Range {
        use ExpressionNode::*;
        match self {
            Primitive(x) => x.range,
            Identifier(x) => x.range,
            Paren(x) => x.range(),
            Unary(x) =>
                Range::new(x.op.range.start, x.arg.range().end),
            Binary(x) =>
                Range::new(x.left.range().start, x.right.range().end),
            Assignment(x) =>
                Range::new(x.left.range().start, x.right.range().end),
            MemberAccess(x) =>
                Range::new(x.argument.range().start, x.member.range.end),
            ArrayAccess(x) =>
                Range::new(x.argument.range().start, x.index.range().end),
            Conditional(x) =>
                Range::new(x.condition.range().start, x.alternate.range().end),
            Increment(x) => match x.is_prefix {
                true => Range::new(x.op.range.start, x.arg.range().end),
                false => Range::new(x.arg.range().start, x.op.range.end),
            }
            ArrayLiteral(x) => {
                let start = x.first().unwrap().range().start;
                let end = x.last().unwrap().range().end;
                Range::new(start, end)
            }
            Call(x) => {
                let start = x.identifier.range.start;
                match x.args.last() {
                    Some(last_arg) =>
                        Range::new(start, last_arg.expression.range().end),
                    _ => x.identifier.range
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct UnaryNode {
    pub op: Token,
    pub arg: Box<ExpressionNode>,
}

#[derive(Clone, Debug)]
pub struct BinaryNode {
    pub op: Token,
    pub left: Box<ExpressionNode>,
    pub right: Box<ExpressionNode>,
}

#[derive(Clone, Debug)]
pub struct AssignmentNode {
    pub op: Token,
    pub left: Box<ExpressionNode>,
    pub right: Box<ExpressionNode>,
}

#[derive(Clone, Debug)]
pub struct IncrementNode {
    pub op: Token,
    pub is_prefix: bool,
    pub arg: Box<ExpressionNode>,
}

#[derive(Clone, Debug)]
pub struct ConditionalNode {
    pub condition: Box<ExpressionNode>,
    pub action: Box<ExpressionNode>,
    pub alternate: Box<ExpressionNode>,
}

#[derive(Clone, Debug)]
pub struct CallNode {
    pub identifier: Token,
    pub args: Vec<CallArgumentNode>
}

#[derive(Clone, Debug)]
pub struct CallArgumentNode {
    pub qualifier: Option<Token>,
    pub expression: ExpressionNode
}

#[derive(Clone, Debug)]
pub struct ArrayAccessNode {
    pub argument: Box<ExpressionNode>,
    pub index: Box<ExpressionNode>,
}

#[derive(Clone, Debug)]
pub struct MemberAccessNode {
    pub argument: Box<ExpressionNode>,
    pub member: Token 
}

