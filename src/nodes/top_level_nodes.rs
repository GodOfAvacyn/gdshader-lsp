use crate::lexer::{Token, TokenError};
use self::expression_node::ExpressionNode;

use super::{*, statement_node::BlockNode};

pub type TopLevelResult = Result<TopLevelNode, TokenError>;

#[derive(Clone, Debug)]
pub enum TopLevelNode {
    ShaderType(ShaderTypeNode),
    RenderMode(RenderModeNode),
    GroupUniforms(GroupUniformsNode),
    Const(ConstNode),
    Varying(VaryingNode),
    Uniform(UniformNode),
    Struct(StructNode),
    Function(FunctionNode)
}

#[derive(Clone, Debug)]
pub struct ShaderTypeNode {
    pub keyword: Token,
    pub shader_type: Token,
}
#[derive(Clone, Debug)]
pub struct RenderModeNode {
    pub keyword: Token,
    pub render_modes: Vec<Token>,
}
#[derive(Clone, Debug)]
pub struct GroupUniformsNode {
    pub keyword: Token,
    pub group: Option<Token>,
    pub subgroup: Option<Token>
}
#[derive(Clone, Debug)]
pub struct ConstNode {
    pub keyword: Token,
    pub precision: Option<Token>,
    pub value: Box<ValueNode>,
    pub expression: Box<ExpressionNode>,
}
#[derive(Clone, Debug)]
pub struct VaryingNode {
    pub keyword: Token,
    pub interpolation: Option<Token>,
    pub precision: Option<Token>,
    pub value: Box<ValueNode>,
}
#[derive(Clone, Debug)]
pub struct UniformNode {
    pub global_instance: Option<Token>,
    pub keyword: Token,
    pub precision: Option<Token>,
    pub value: Box<ValueNode>,
    pub hint: Option<HintNode>,
    pub expression: Option<Box<ExpressionNode>>,
}
#[derive(Clone, Debug)]
pub struct HintNode {
    pub identifier: Token,
    pub params: Option<Vec<Token>> 
}
#[derive(Clone, Debug)]
pub struct StructNode {
    pub keyword: Token,
    pub identifier: Token,
    pub fields: Vec<ValueNode>,
}
#[derive(Clone, Debug)]
pub struct FunctionNode {
    pub type_node: TypeNode,
    pub identifier: Token,
    pub params: Vec<ParamNode>,
    pub block: BlockNode
}
#[derive(Clone, Debug)]
pub struct ParamNode {
    pub qualifier: Option<Token>,
    pub value_node: ValueNode,
}

