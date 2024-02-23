use crate::{lexer::Token, memory::*, nodes::*};

use super::{ensure_valid_value, evaluate_expression, EvaluateError};

pub fn eval_block(
    memory: &mut Memory,
    block: BlockNode,
) {
    for statement in block.statements {
        _ = evaluate_statement(memory, statement);
    }
}

pub fn evaluate_statement(
    memory: &mut Memory,
    statement: StatementNode,
) -> Result<(), EvaluateError> {
    match statement {
        StatementNode::VarDeclaration(x) => evaluate_var_declaration(memory, x),
        StatementNode::If(x) => evaluate_if_statement(x, memory),
        StatementNode::While(x) => evaluate_while_statement(x, memory),
        StatementNode::For(x) => evaluate_for_statement(x, memory),
        StatementNode::Switch(x) => evaluate_switch_statement(x, memory),
        StatementNode::Expression(x) => evaluate_expression(memory, *x).map(|_| ()),
        StatementNode::Continue(x) => evaluate_continue_statement(memory, x),
        StatementNode::Break(x) => evaluate_break_statement(memory, x),
        StatementNode::Return(x) => evaluate_return_statement(memory, x),
        StatementNode::Block(x) => {
            eval_block(memory, x);
            Ok(())
        }
    }
}

fn evaluate_return_statement(
    memory: &mut Memory,
    node: ReturnNode
) -> Result<(), EvaluateError> {
    memory.scopes.set_actual_return_type(None);
    if let Some(expr) = node.expression {
        let result_range = expr.range();
        let result = evaluate_expression(memory, *expr)?;
        memory.scopes.set_actual_return_type(Some(result.type_info.clone()));
        if let Some(return_type) = memory.scopes.get_expected_return_type() {
            if return_type != result.type_info {
                let message = format!(
                    "Invalid return type, expected '{}'",
                    return_type.to_string()
                );
                return Err(memory.alert_error(&message, result_range))
            }
        }
    }
    Ok(())
}

fn evaluate_break_statement(
    memory: &mut Memory,
    node: Token
) -> Result<(), EvaluateError> {
    if memory.scopes.scope_type() != ScopeType::Loop {
        let message = "Cannot use 'break' outside a loop.";
        Err(memory.alert_error(message, node.range))
    } else {
        Ok(())
    }
}

fn evaluate_continue_statement(
    memory: &mut Memory,
    node: Token
) -> Result<(), EvaluateError> {
    if memory.scopes.scope_type() != ScopeType::Loop {
        let message = "Cannot use 'continue' outside a loop.";
        Err(memory.alert_error(message, node.range))
    } else {
        Ok(())
    }
}

fn evaluate_switch_statement(
    node: SwitchNode,
    memory: &mut Memory
) -> Result<(), EvaluateError> {
    let range = node.condition.range();
    if let Ok(expr) = evaluate_expression(memory, *node.condition) {
        if expr.type_info != TypeInfo::from_str("int") &&
            expr.type_info != TypeInfo::from_str("uint") {
                memory.alert_error("Switch condition must be an integer.", range);
            }
    }


    for case in node.cases {
        for statement in case.statements {
            _ = evaluate_statement(memory, statement);
        }
    }
    Ok(())
}

fn evaluate_for_statement(
    node: ForNode,
    memory: &mut Memory
) -> Result<(), EvaluateError> {
    let block_range = match *node.action {
        StatementNode::Block(ref b) => Some(b.range),
        _ => None
    };

    if let StatementNode::VarDeclaration(initializer) = *node.initializer {
        if let Some(range) = block_range {
            memory.scopes.enter_scope(ScopeType::Loop, range)
        }
        evaluate_var_declaration(memory, initializer)?;
    } else {
        let message = "Left expression of a for loop must be a variable declaration.";
        return Err(memory.alert_error(message, node.keyword.range));
    }

    let condition_range = node.condition.range();
    if let Ok(condition_result) = evaluate_expression(memory, *node.condition) {
        if condition_result.type_info != TypeInfo::from_str("bool") {
            let message = "For loop condition must be a boolean expression.";
            memory.alert_error(message, condition_range);
        }
    }
    _ = evaluate_expression(memory, *node.update)?;
    _ = evaluate_statement(memory, *node.action);
    if block_range.is_some() {
        memory.scopes.leave_scope();
    }
    
    Ok(())
}

fn evaluate_while_statement(
    node: WhileNode,
    memory: &mut Memory
) -> Result<(), EvaluateError> {
    let condition_range = node.condition.range();
    let condition_result = evaluate_expression(memory, *node.condition)?;
    if condition_result.type_info != TypeInfo::from_str("bool") {
        let message = "While loop condition must be a boolean expression.";
        memory.alert_error(message, condition_range);
    }
    let block_range = match *node.action {
        StatementNode::Block(ref b) => Some(b.range),
        _ => None
    };
    if let Some(range) = block_range {
        memory.scopes.enter_scope(ScopeType::Loop, range)
    }
    evaluate_statement(memory, *node.action)?;
    if block_range.is_some() {
        memory.scopes.leave_scope();
    }
    Ok(())
}

fn evaluate_if_statement(
    node: IfNode,
    memory: &mut Memory
) -> Result<(), EvaluateError> {
    let condition_range = node.condition.range();
    let condition_result = evaluate_expression(memory, *node.condition)?;
    if condition_result.type_info != TypeInfo::from_str("bool") {
        eprintln!("this is what it was {:?}", condition_result.type_info);
        let message = "If statement condition must be a boolean expression.";
        memory.alert_error(message, condition_range);
    } 
    let mut block_range = match *node.action {
        StatementNode::Block(ref b) => Some(b.range),
        _ => None
    };
    if let Some(range) = block_range {
        memory.scopes.enter_scope(ScopeType::Loop, range)
    }
    _ = evaluate_statement(memory, *node.action);
    if block_range.is_some() {
        memory.scopes.leave_scope();
    }
    
    if let Some(x) = node.alternate {
        block_range = match *x.action {
            StatementNode::Block(ref b) => Some(b.range),
            _ => None
        };
        if let Some(range) = block_range {
            memory.scopes.enter_scope(ScopeType::Loop, range)
        }
        _ = evaluate_statement(memory, *x.action);
        if block_range.is_some() {
            memory.scopes.leave_scope();
        }
    }
    Ok(())
}

fn evaluate_var_declaration(
    memory: &mut Memory,
    node: VarDeclarationNode,
) -> Result<(), EvaluateError> {
    let value = &node.value;
    
    let expression = node.expression;
    ensure_valid_value(memory, value, expression.map(|x| *x), node.is_const)?;

    let name = memory.get_token_text(value.identifier);
    let ty = value.type_node.info.clone();
    let range = value.range;
    let (is_const, editable) = (node.is_const, !node.is_const);
    memory.scopes.insert( name, ValueInfo {
        ty,
        is_const,
        editable,
        range: Some(range),
        description: None
    });
    Ok(())
}


