use lsp_types::Range;
use crate::{
    lexer::{MaybeOperator, OperationType, Token},
    memory::{Memory, Primitive, TypeInfo},
    nodes::*
};

use super::{eval_operation, eval_swizzle, EvaluateError};

pub struct ExpressionEvaluation {
    pub type_info: TypeInfo,
    pub is_const: bool,
    pub is_assignable: bool
}
impl ExpressionEvaluation {
    pub fn new(type_info: TypeInfo, is_const: bool, is_assignable: bool) -> Self {
        Self {
            type_info,
            is_const,
            is_assignable
        }
    }
}

pub type ExprEvalResult = Result<ExpressionEvaluation, EvaluateError>;

pub fn evaluate_expression(
    memory: &mut Memory,
    expr: ExpressionNode
) -> ExprEvalResult {
    let range = expr.range();
    match expr {
        ExpressionNode::Identifier(x) => eval_identifier_expr(memory, x, range),
        ExpressionNode::Unary(x) => eval_unary_expr(memory, x, range),
        ExpressionNode::Binary(x) => eval_binary_expr(memory, x, range),
        ExpressionNode::Paren(x) => evaluate_expression(memory, *x),
        ExpressionNode::Conditional(x) => eval_conditional(memory, x, range),
        ExpressionNode::Call(x) => eval_call_expr(memory, x, range),
        ExpressionNode::ArrayAccess(x) => eval_array_access(memory, x, range),
        ExpressionNode::MemberAccess(x) => eval_member_access(memory, x, range),
        ExpressionNode::ArrayLiteral(x) => eval_array_literal(memory, x, range),
        ExpressionNode::Assignment(x) => eval_assignment_expr(memory, x, range), 
        ExpressionNode::Increment(x) => eval_increment_expr(memory, x, range),
        ExpressionNode::Primitive(p) =>
            Ok(ExpressionEvaluation::new(TypeInfo::from_primitive(p), true, false)),
    }
}

fn eval_increment_expr(
    memory: &mut Memory,
    increment: IncrementNode,
    range: lsp_types::Range
) -> ExprEvalResult {
    let result = evaluate_expression(memory, *increment.arg)?;
    if !result.is_assignable || result.is_const {
        let message = "Cannot change a constant value.";
        return Err(memory.alert_error(message, range))
    }
    eval_operation(memory, OperationType::Number, result.type_info.clone(), range)?;
    Ok(ExpressionEvaluation::new(result.type_info, result.is_const, false))
}

fn eval_assignment_expr(
    memory: &mut Memory,
    assignment: AssignmentNode,
    range: lsp_types::Range
) -> ExprEvalResult {
    let left_range = assignment.left.range();
    let right_range = assignment.right.range();
    let left_result = evaluate_expression(memory, *assignment.left)?;
    let right_result = evaluate_expression(memory, *assignment.right)?;
    if !left_result.is_assignable || left_result.is_const {
        let message = "Cannot assign to a constant value.";
        return Err(memory.alert_error(message, left_range))
    }
    if left_result.type_info != right_result.type_info {
        let message = format!(
            "Type mismatch: '{}' and '{}'.",
            left_result.type_info.to_string(),
            right_result.type_info.to_string()
        );
        return Err(memory.alert_error(&message, range));
    }
    for ty in [left_result.type_info.clone(), right_result.type_info.clone()] {
        eval_operation(memory, assignment.op.to_assignment_op().unwrap(), ty, left_range)?;
    }
    Ok(ExpressionEvaluation::new(left_result.type_info, left_result.is_const, false))
}

fn eval_array_literal(
    memory: &mut Memory,
    mut array_literal: Vec<ExpressionNode>,
    range: Range
) -> ExprEvalResult {
    let len = array_literal.len();
    let mut is_const = true;
    if len == 0 {
        let message = "Empty array literal.";
        return Err(memory.alert_error(message, range))
    }

    let first = array_literal.swap_remove(0);
    let first_range = first.range();
    let first_result = evaluate_expression(memory, first)?;
    if first_result.type_info.size != 0 {
        let message = "Nested array types are not allowed.";
        return Err(memory.alert_error(message, first_range))
    }
    if !first_result.is_const {
        is_const = false;
    }

    for expr in array_literal {
        let result = evaluate_expression(memory, expr)?;
        if result.type_info != first_result.type_info{
            let message = format!(
                "Mismatched types: {} and {}",
                first_result.type_info.to_string(),
                result.type_info.to_string()
            );
            return Err(memory.alert_error(&message, range))
        }
        if !result.is_const {
            is_const = false;
        }
    };
    Ok(ExpressionEvaluation::new(
        TypeInfo { base: first_result.type_info.base, size: len as u32 },
        is_const,
        false
    ))
}

fn eval_member_access(
    memory: &mut Memory,
    member_access: MemberAccessNode,
    range: lsp_types::Range
) -> ExprEvalResult {
    let member_access_str = memory.get_token_text(member_access.member);
    let argument_result = evaluate_expression(memory, *member_access.argument)?;
    if argument_result.type_info.size != 0 {
        let message = format!(
            "Cannot access member of array type {}",
            argument_result.type_info.to_string()
        );
        return Err(memory.alert_error(&message, range))
    }
    let message = format!(
        "Type {} has no member {}",
        argument_result.type_info.to_string(),
        member_access_str
    );
    let member = &member_access_str;
    
    if let Some(swizzle) = match argument_result.type_info.base.as_str() {
        "vec2" => Some(eval_swizzle(member, Primitive::Float, 2)),
        "vec3" => Some(eval_swizzle(member, Primitive::Float, 3)),
        "vec4" => Some(eval_swizzle(member, Primitive::Float, 4)),
        "ivec2" => Some(eval_swizzle(member, Primitive::Int, 2)),
        "ivec3" => Some(eval_swizzle(member, Primitive::Int, 3)),
        "ivec4" => Some(eval_swizzle(member, Primitive::Int, 4)),
        "uvec2" => Some(eval_swizzle(member, Primitive::Uint, 2)),
        "uvec3" => Some(eval_swizzle(member, Primitive::Uint, 3)),
        "uvec4" => Some(eval_swizzle(member, Primitive::Uint, 4)),
        "bvec2" => Some(eval_swizzle(member, Primitive::Bool, 2)),
        "bvec3" => Some(eval_swizzle(member, Primitive::Bool, 3)),
        "bvec4" => Some(eval_swizzle(member, Primitive::Bool, 4)),
        _ => None
    } {
        if let Some(ty) = swizzle {
            return Ok(ExpressionEvaluation::new(
                ty,
                argument_result.is_const,
                argument_result.is_assignable
            ))
        } else {
            return Err(memory.alert_error(&message, range))
        }
    }
    memory.structs
        .get(&argument_result.type_info.base)
        .and_then(|x| x.fields.iter().find(|x| x.name.as_str() == member).map(|x| x.ty.clone()))
        .ok_or_else(|| memory.alert_error(&message, range))
        .map(|ty| ExpressionEvaluation::new(
            ty,
            argument_result.is_const,
            argument_result.is_assignable
        ))
}

fn eval_array_access(
    memory: &mut Memory,
    array_access: ArrayAccessNode,
    range: lsp_types::Range
) -> ExprEvalResult {
    let argument_result = evaluate_expression(memory, *array_access.argument)?;
    let index_result = evaluate_expression(memory, *array_access.index)?;
    let is_const = argument_result.is_const && index_result.is_const;
    let is_assignable = argument_result.is_assignable;
    match index_result.type_info.base.as_str() {
        "int" | "uint" => {},
        _ => {
            let message = format!(
                "Cannot index with type {}",
                index_result.type_info.to_string()
            );
            return Err(memory.alert_error(&message, range))
        }
    }
    let new_eval = ExpressionEvaluation::new;
    match argument_result.type_info.base.as_str() {
        "vec2" => Ok(new_eval(TypeInfo::from_str("float"), is_const, is_assignable)),
        "vec3" => Ok(new_eval(TypeInfo::from_str("float"), is_const, is_assignable)),
        "vec4" => Ok(new_eval(TypeInfo::from_str("float"), is_const, is_assignable)),
        "mat2" => Ok(new_eval(TypeInfo::from_str("vec2"), is_const, is_assignable)),
        "mat3" => Ok(new_eval(TypeInfo::from_str("vec3"), is_const, is_assignable)),
        "mat4" => Ok(new_eval(TypeInfo::from_str("vec4"), is_const, is_assignable)),
        _ if argument_result.type_info.size != 0 => {
            Ok(new_eval(
                TypeInfo { base: argument_result.type_info.base, size: 0 },
                is_const,
                is_assignable
            ))
        },
        _ => {
            let message = format!(
                "Type {} cannot be indexed.",
                argument_result.type_info.to_string()
            );
            Err(memory.alert_error(&message, range))
        }
    }
}

fn eval_call_expr(
    memory: &mut Memory,
    call: CallNode,
    range: lsp_types::Range
) -> ExprEvalResult {
    let call_name = memory.get_token_text(call.identifier);
    let mut is_const = true;
    let mut arg_types = vec![];

    for arg in call.args {
        let result = evaluate_expression(memory, arg.expression)?;
        if !result.is_const{
            is_const = false
        }
        arg_types.push((arg.qualifier.map(|x| x.kind), result.type_info));
    }

    let maybe_struct = memory.structs.get(&call_name);
    if let Some(struct_info) = maybe_struct {
        let correct_types = struct_info.fields
            .iter()
            .map(|x| &x.ty)
            .zip(arg_types.iter())
            .collect::<Vec<_>>();
        for (info, (_, arg_info)) in correct_types {
            if *info != *arg_info {
                let message = format!("Invalid arguments for function '{}'", call_name);
                return Err(memory.alert_error(&message, range));
            }
        }
        return Ok(ExpressionEvaluation::new(
            TypeInfo::from_str(&call_name),
            is_const,
            false
        ));
    }

    let maybe_function = memory.functions.get(&call_name);
    if let Some(function) = maybe_function {
        if !function.is_const {
            is_const = false;
        }
        for signature in &function.signatures {
            let correct_types = signature.params
                .iter()
                .map(|x| (x.qualifier.clone().map(|y| y.kind()), x.ty.clone()))
                .collect::<Vec<_>>();
            if arg_types != correct_types {
                continue;
            }
            let old_return_type = &signature.return_type;

            if let Some(primitive_type) = old_return_type.is_generically_sized() {
                let generic_args: Vec<_> = correct_types
                    .iter()
                    .zip(arg_types.iter())
                    .filter_map(|((_, generic), arg)| {
                        if generic.is_generically_sized().is_some() { Some(arg) }
                        else { None }
                    })
                    .collect();

                let generic_size = generic_args[0].1.get_generic_size();
                if generic_args.iter().all(|x| x.1.get_generic_size() == generic_size) {
                    let ty = TypeInfo::from_pieces(primitive_type, generic_size.unwrap());
                    return Ok(ExpressionEvaluation::new(ty, is_const, false));
                }
            } else if let Some(generic_size) = old_return_type.is_generically_typed() {
                let generic_args: Vec<_> = correct_types
                    .iter()
                    .zip(arg_types.iter())
                    .filter_map(|((_, generic), arg)| {
                        if generic.is_generically_typed().is_some() { Some(arg) }
                        else { None }
                    })
                    .collect();

                let generic_type = generic_args[0].1.get_generic_type();
                if generic_args.iter().all(|x| x.1.get_generic_type() == generic_type) {
                    let ty = TypeInfo::from_pieces(generic_type.unwrap(), generic_size);
                    return Ok(ExpressionEvaluation::new(ty, is_const, false));
                }
            } else {
                return Ok(ExpressionEvaluation::new(old_return_type.clone(), is_const, false));
            }
        }
        let message = format!("Invalid arguments for function '{}'", call_name);
        Err(memory.alert_error(&message, range))
    } else {
        let message = format!("Function '{}' does not exist.", call_name);
        Err(memory.alert_error(&message, range))
    }
}

fn eval_conditional(
    memory: &mut Memory,
    conditional: ConditionalNode,
    range: lsp_types::Range
) -> ExprEvalResult {
    let condition_result = evaluate_expression(memory, *conditional.condition)?;
    if condition_result.type_info.base.as_str() != "bool" {
        let message = format!(
            "Condition type must be bool, not {}",
            condition_result.type_info.to_string()
        );
        return Err(memory.alert_error(&message, range));
    }
    let left_result = evaluate_expression(memory, *conditional.action)?;
    let right_result = evaluate_expression(memory, *conditional.alternate)?;
    let is_const = left_result.is_const && right_result.is_const && condition_result.is_const;

    if left_result.type_info!= right_result.type_info{
        let message = format!(
            "Type mismatch: {} & {}",
            left_result.type_info.to_string(),
            right_result.type_info.to_string()
        );
        Err(memory.alert_error(&message, range))
    } else {
        Ok(ExpressionEvaluation::new(left_result.type_info, is_const, false))
    }
}

fn eval_binary_expr(
    memory: &mut Memory,
    binary: BinaryNode,
    range: lsp_types::Range
) -> ExprEvalResult {
    let left = evaluate_expression(memory, *binary.left)?;
    let right = evaluate_expression(memory, *binary.right)?;
    let mut type_mismatch = || {
        let message = format!(
            "Type mismatch: {} & {}",
            left.type_info.to_string(),
            right.type_info.to_string()
        );
        Err(memory.alert_error(&message, range))
    };
    let is_const = left.is_const && right.is_const;

    let left_size = if let Some(x) =
        left.type_info.get_generic_size().map_or(None, |x| x.as_size()) { x }
        else { return type_mismatch() };
    
    let right_size = if let Some(x) =
        right.type_info.get_generic_size().map_or(None, |x| x.as_size()) { x }
        else { return type_mismatch() };

    let left_generic = if let Some(x) = left.type_info.get_generic_type() { x }
    else { return type_mismatch() };

    let right_generic = if let Some(x) = right.type_info.get_generic_type() { x }
    else { return type_mismatch() };

    let correct_type = if left_size == right_size {
        left.type_info.clone()
    } else {
        if left_size == 1 {
            right.type_info.clone()
        } else if right_size == 1 {
            left.type_info.clone()
        } else {
            return type_mismatch();
        }
    };

    if left_generic != right_generic {
        type_mismatch()
    } else {
        eval_operation(memory, binary.op.to_binary_op().unwrap(), correct_type, range)
            .map(|ty| ExpressionEvaluation::new(ty, is_const, false))
    }
}

fn eval_unary_expr(
    memory: &mut Memory,
    unary: UnaryNode,
    range: lsp_types::Range
) -> ExprEvalResult {
    let result = evaluate_expression(memory, *unary.arg)?;
    eval_operation(memory, unary.op.to_unary_op().unwrap(), result.type_info, range)
        .map(|ty| ExpressionEvaluation::new(ty, result.is_const, false))
}

fn eval_identifier_expr(
    memory: &mut Memory,
    value: Token,
    range: lsp_types::Range
) -> ExprEvalResult {
    let identifier = memory.get_token_text(value);
    for scope in memory.scopes.collect_scopes() {
        if let Some(value) = scope.get(&identifier) {
            return Ok(ExpressionEvaluation::new(
                value.ty.clone(),
                value.is_const,
                true
            ))
        }
    }
    let message = format!("Identifier '{}' is undefined.", identifier);
    Err(memory.alert_error(&message, range))
}





