use lsp_types::Range;
use regex::Regex;

use crate::{
    lexer::{OperationType, Token},
    memory::{Memory, Primitive, TypeInfo},
    nodes::{ExpressionNode, TypeNode, ValueNode}
};

mod top_level_interpreter;
mod expression_interpreter;
mod statement_interpreter;
pub use top_level_interpreter::*;
pub use expression_interpreter::*;
pub use statement_interpreter::*;


#[derive(Debug)]
pub enum EvaluateError { 
    Warning,
    SemanticsError
}

pub type EvaluateResult = Result<(), EvaluateError>;

pub fn ensure_valid_id( memory: &mut Memory, id: Token,) -> Result<(), EvaluateError> {
    let id_text = memory.get_token_text(id);
    if memory.is_id_in_use(&id_text) {
        let message = format!("Identifier '{}' is in use.", id_text);
        Err(memory.alert_error(&message, id.range))
    } else {
        Ok(())
    }
}

pub fn ensure_valid_type(
    memory: &mut Memory,
    ty: &TypeNode
) -> Result<(), EvaluateError> {
    if !memory.is_id_valid_type(&ty.info.base) {
        let message = format!("Invalid type '{}'.", ty.info.to_string());
        Err(memory.alert_error(&message, ty.range))
    } else {
        Ok(())
    }
}


pub fn ensure_valid_value(
    memory: &mut Memory,
    value: &ValueNode,
    expression: Option<ExpressionNode>,
    is_const: bool,
) -> EvaluateResult {
    ensure_valid_id(memory, value.identifier)?;
    ensure_valid_type(memory, &value.type_node)?;
    if let Some(expr) = expression {
        let expr_range = expr.range();
        if let Ok(result) = evaluate_expression(memory, expr) {
            if result.type_info != value.type_node.info {
                let message = format!(
                    "Type mismatch: '{}' and '{}'.",
                    value.type_node.info.to_string(),
                    result.type_info.to_string()
                );
                memory.alert_error(&message, Range::new(value.range.start, expr_range.end));
            } else if is_const && !result.is_const {
                let message = "Invalid constant expression.";
                memory.alert_error(&message, expr_range);
            }
        }
    }
    Ok(())
}

pub fn eval_swizzle(
    swizzle: &str,
    base_type: Primitive,
    base_length: usize,
) -> Option<TypeInfo> {
    let vec2_regex: Regex = Regex::new(r"^(?:[xy]{1,4}|[rg]{1,4}|[st]{1,4})$").unwrap();
    let vec3_regex: Regex = Regex::new(r"^(?:[xyz]{1,4}|[rgb]{1,4}|[stp]{1,4})$").unwrap();
    let vec4_regex: Regex = Regex::new(r"^(?:[xyzw]{1,4}|[rgba]{1,4}|[stpq]{1,4})$").unwrap();

    if !match base_length {
        2 => vec2_regex.is_match(swizzle),
        3 => vec3_regex.is_match(swizzle),
        4 => vec4_regex.is_match(swizzle),
        _ => unreachable!()
    } {
        return None
    }

    let length = swizzle.len();
    Some(match base_type {
        Primitive::Bool=> match length {
            1 => TypeInfo::from_str("bool"),
            2 => TypeInfo::from_str("bvec2"),
            3 => TypeInfo::from_str("bvec3"),
            4 => TypeInfo::from_str("bvec4"),
            _ => unreachable!()
        }
        Primitive::Uint => match length {
            1 => TypeInfo::from_str("uint"),
            2 => TypeInfo::from_str("uvec2"),
            3 => TypeInfo::from_str("uvec3"),
            4 => TypeInfo::from_str("uvec4"),
            _ => unreachable!()
        }
        Primitive::Int => match length {
            1 => TypeInfo::from_str("int"),
            2 => TypeInfo::from_str("ivec2"),
            3 => TypeInfo::from_str("ivec3"),
            4 => TypeInfo::from_str("ivec4"),
            _ => unreachable!()
        }
        Primitive::Float => match length {
            1 => TypeInfo::from_str("float"),
            2 => TypeInfo::from_str("vec2"),
            3 => TypeInfo::from_str("vec3"),
            4 => TypeInfo::from_str("vec4"),
            _ => unreachable!()
        }
    })
}


pub fn eval_operation(
    memory: &mut Memory,
    op: OperationType,
    ty: TypeInfo,
    range: Range
) -> Result<TypeInfo, EvaluateError> {
    match op {
        OperationType::Number => {
            if ty == TypeInfo::from_str("vec_type")
            || ty == TypeInfo::from_str("ivec_type")
            || ty == TypeInfo::from_str("uvec_type")
            || ty == TypeInfo::from_str("bvec_type")
            || ty == TypeInfo::from_str("mat_type") {
                Ok(ty)
            } else {
                let message = format!("Invalid type for operation: {}", ty.to_string());
                Err(memory.alert_error(&message, range))
            }
        }
        OperationType::Int => {
            if ty == TypeInfo::from_str("ivec_type")
            || ty == TypeInfo::from_str("uvec_type") {
                Ok(ty)
            } else {
                let message = format!("Invalid type for operation: {}", ty.to_string());
                Err(memory.alert_error(&message, range))
            }
        }
        OperationType::Bool => {
            match ty.base.as_str() {
                "bool" => Ok(TypeInfo::from_str("bool")),
                _ => {
                    let message = format!("Invalid type for operation: {}", ty.to_string());
                    Err(memory.alert_error(&message, range))
                }
            }
        }
        OperationType::Comparison => {
            match ty.base.as_str() {
                 "int" => Ok(TypeInfo::from_str("bool")), 
                 "uint" => Ok(TypeInfo::from_str("bool")), 
                 "float" => Ok(TypeInfo::from_str("bool")), 
                _ => {
                    let message = format!("Invalid type for operation: {}", ty.to_string());
                    Err(memory.alert_error(&message, range))
                }
            }
        }
        OperationType::Equal => {
            Ok(TypeInfo::from_str("bool"))
        }
    }
}

