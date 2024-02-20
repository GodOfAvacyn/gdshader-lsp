use super::*;

pub fn parse_expression(
    stream: &mut TokenStream,
) -> ExpressionResult {
    let mut original = parse_expression_priority(stream)?;
    loop {
        if let Some(op) = parse_binary_operation(stream) {
            original = parse_binary_expression(stream, op, original)?
        } else if let Some(op) = parse_assignment_operation(stream) {
            original = parse_assignment_expression(stream, op, original)?
        } else {
            break Ok(original)
        }
    }
}

pub fn parse_expression_priority(
    stream: &mut TokenStream,
) -> ExpressionResult {
    if !stream.parsing_statement {
        stream.queue_cursor_element(CompletionElement::Identifier(stream.parsing_const));
    }

    let mut original = if let Some(primitive) = parse_primitive(stream) {
        ExpressionNode::Primitive(primitive)
    } else if let Some(op) = parse_unary_operation(stream){
        parse_unary_expression(stream, op)?
    } else {
        let current = stream.current()?;
        match current.kind {
            Identifier => parse_identifier_expression(stream)?,
            LeftParen => parse_parenthetical_expression(stream)?,
            LeftBrace => parse_array_literal_expression(stream)?,
            Increment | Decrement => parse_increment_expression(stream, None)?,
            _ => {
                return Err(stream.alert_error("Expected Expression", current.range))
            }
        }
    };
    loop {
        let current = stream.current()?;
        match current.kind {
            LeftBracket => original = parse_array_access_expression(stream, original)?,
            Dot => original = parse_member_access_expression(stream, original)?,
            Question => original = parse_conditional_expression(stream, original)?,
            Increment | Decrement =>
                original = parse_increment_expression(stream, Some(original))?,
            _ => break Ok(original)
        }
    }
}

pub fn parse_identifier_expression(stream: &mut TokenStream) -> ExpressionResult {
    let identifier = parse_identifier(stream)?;
    match stream.current()?.kind {
        LeftParen => parse_call_expression(stream, identifier),
        _ => Ok(ExpressionNode::Identifier(identifier))
    }
}

pub fn parse_call_expression(
    stream: &mut TokenStream,
    identifier: Token
) -> ExpressionResult {
    stream.advance();
    let args = parse_list(
        stream,
        Comma,
        RightParen,
        Trailing::Optional,
        |s| parse_expression(s)
            .map(|e| CallArgumentNode{qualifier: None, expression: e})
    )?;
    stream.advance();
    Ok(ExpressionNode::Call(CallNode {
        identifier,
        args
    }))
}

pub fn parse_unary_expression(stream: &mut TokenStream, op: Token) -> ExpressionResult {
    let arg = Box::new(parse_expression_priority(stream)?);
    Ok(ExpressionNode::Unary(UnaryNode{ arg, op }))
}

pub fn parse_binary_expression(
    stream: &mut TokenStream,
    op: Token,
    original: ExpressionNode
) -> ExpressionResult {
    let right = parse_expression_priority(stream)?; 

    Ok(ExpressionNode::Binary(BinaryNode{
        left: Box::new(original),
        right: Box::new(right),
        op
    }))
}

pub fn parse_assignment_expression(
    stream: &mut TokenStream,
    op: Token,
    original: ExpressionNode
) -> ExpressionResult {
    let right = parse_expression(stream)?;

    Ok(ExpressionNode::Assignment(AssignmentNode{
        left: Box::new(original),
        right: Box::new(right),
        op
    }))
}

pub fn parse_increment_expression(
    stream: &mut TokenStream,
    original: Option<ExpressionNode>,
) -> ExpressionResult {
    let op = stream.consume()?;
    let (arg, is_prefix) = match original {
        Some(arg) => (arg, false),
        None => (parse_expression(stream)?, true)
    };

    Ok(ExpressionNode::Increment(IncrementNode{
        op,
        is_prefix,
        arg: Box::new(arg)
    }))
}

pub fn parse_parenthetical_expression(stream: &mut TokenStream) -> ExpressionResult {
    stream.advance();
    let expr = Box::new(parse_expression(stream)?);
    parse_kind(stream, RightParen)?;

    Ok(ExpressionNode::Paren(expr))
}

pub fn parse_conditional_expression(
    stream: &mut TokenStream,
    originial: ExpressionNode
) -> ExpressionResult {
    stream.advance();
    let action = parse_expression(stream)?;
    parse_kind(stream, Colon)?;
    let alternate = parse_expression(stream)?;

    Ok(ExpressionNode::Conditional(ConditionalNode{
        condition: Box::new(originial),
        action: Box::new(action),
        alternate: Box::new(alternate)
    }))
}

pub fn parse_array_access_expression(
    stream: &mut TokenStream,
    original: ExpressionNode
) -> ExpressionResult {
    stream.advance();
    let index = parse_expression(stream)?;
    parse_kind(stream, RightBracket)?;

    Ok(ExpressionNode::ArrayAccess(ArrayAccessNode{
        argument: Box::new(original),
        index: Box::new(index)
    }))
}

pub fn parse_member_access_expression(
    stream: &mut TokenStream,
    original: ExpressionNode
) -> ExpressionResult {
    stream.queue_cursor_element(CompletionElement::Member(Box::new(original.clone())));
    stream.advance();
    let member = parse_identifier(stream)?;

    Ok(ExpressionNode::MemberAccess(MemberAccessNode{
        argument: Box::new(original),
        member
    }))
}


pub fn parse_array_literal_expression(stream: &mut TokenStream) -> ExpressionResult {
    stream.advance();
    let vec = parse_list(
        stream,
        Comma,
        RightBrace,
        Trailing::Optional,
        |s| parse_expression(s) 
    )?;

    Ok(ExpressionNode::ArrayLiteral(vec))
}




