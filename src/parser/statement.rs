use super::*;

pub fn parse_block(stream: &mut TokenStream) -> StatementResult {
    let left = parse_kind(stream, LeftBrace)?;
    stream.queue_cursor_element(CompletionElement::Statement);
    let mut statements = vec![];
    loop {
        stream.parsing_statement = true;
        let current = stream.current()?;
        match current.kind {
            RightBrace => break,
            _ => match parse_statement(stream) {
                Ok(statement) => statements.push(statement),
                _ => stream.advance()
            }
        }
    }
    let right = parse_kind(stream, RightBrace)?;
    Ok(StatementNode::Block(BlockNode {
        statements,
        range: Range::new(left.range.start, right.range.end)
    }))
}

pub fn parse_statement(stream: &mut TokenStream) -> StatementResult {
    let current = stream.current()?;
    stream.queue_cursor_element(CompletionElement::Statement);
    stream.parsing_statement = true;
    match current.kind {
        Const => parse_var_declaration_statement(stream),
        If => parse_if_statement(stream),
        While => parse_while_statement(stream),
        For => parse_for_statement(stream),
        Switch => parse_switch_statement(stream),
        Continue => Ok(StatementNode::Continue(stream.consume()?)),
        Break => Ok(StatementNode::Break(stream.consume()?)),
        Return => parse_return_statement(stream), 
        Identifier => parse_identifier_statement(stream),
        LeftBrace => parse_block(stream),
        _ => parse_expression_statement(stream)
    }
}

pub fn parse_expression_statement(stream: &mut TokenStream) -> StatementResult {
    let expression = parse_expression(stream)?;
    _ = parse_semicolon(stream);
    Ok(StatementNode::Expression(Box::new(expression)))
}

pub fn parse_identifier_statement(stream: &mut TokenStream) -> StatementResult {
    let parsing_statement = stream.parsing_statement;
    stream.turn_off_errors();
    let old_idx = stream.current_idx();
    let maybe_type = parse_type(stream);
    let maybe_name = parse_identifier(stream);
    stream.force_change_position(old_idx);
    stream.turn_on_errors();
    stream.parsing_statement = parsing_statement;

    match maybe_type.is_ok() && maybe_name.is_ok() {
        true => parse_var_declaration_statement(stream),
        false => parse_expression_statement(stream)
    }
}

pub fn parse_var_declaration_statement(stream: &mut TokenStream) -> StatementResult {
    let (keyword, is_const) = parse_conditional(stream, Const)
         .map_or((None, false), |x| (Some(x), true));

    if is_const {
        stream.queue_cursor_element(CompletionElement::Type);
        stream.parsing_const = is_const;
    }
    let value = parse_value_specifier(stream);
    stream.parsing_const = false;
    let value = value?;

    let expression = parse_conditional(stream, Equal)
        .map(|_| parse_expression(stream))
        .transpose()?
        .map(|x| Box::new(x));
    _ = parse_semicolon(stream);
    
    Ok(StatementNode::VarDeclaration(VarDeclarationNode{
        keyword,
        value: Box::new(value),
        expression,
        is_const
    }))
}

pub fn parse_if_statement(stream: &mut TokenStream) -> StatementResult {
    let keyword = stream.consume()?;
    parse_kind(stream, LeftParen)?;
    stream.parsing_statement = false;
    let condition = Box::new(parse_expression(stream)?); 
    parse_kind(stream, RightParen)?;
    let action = Box::new(parse_statement(stream)?);
    stream.queue_cursor_element(CompletionElement::Identifier(stream.parsing_const));
    let alternate = match parse_conditional(stream, Else) {
        Some(token) => Some(ElseNode {
            keyword: token,
            action: Box::new(parse_statement(stream)?)
        }),
        None => None
    };
    Ok(StatementNode::If(IfNode {
        keyword,
        condition,
        action,
        alternate
    }))
}

pub fn parse_while_statement(stream: &mut TokenStream) -> StatementResult {
    let keyword = stream.consume()?;
    parse_kind(stream, LeftParen)?;
    let condition = Box::new(parse_expression(stream)?); 
    parse_kind(stream, RightParen)?;
    let action = Box::new(parse_statement(stream)?);

    Ok(StatementNode::While(WhileNode {
        keyword,
        condition,
        action
    }))
}

pub fn parse_for_statement(stream: &mut TokenStream) -> StatementResult {
    let keyword = stream.consume()?;
    parse_kind(stream, LeftParen)?;
    let initializer = Box::new(parse_statement(stream)?);
    let condition = Box::new(parse_expression(stream)?); 
    _ = parse_semicolon(stream);
    let update = Box::new(parse_expression(stream)?);
    parse_kind(stream, RightParen)?;
    let mut action = parse_statement(stream)?;
    action = if let StatementNode::Block(mut block) = action {
        block.range.start = keyword.range.start; 
        StatementNode::Block(block)
    } else {
        action
    };

    Ok(StatementNode::For(ForNode {
        keyword,
        initializer,
        condition,
        update,
        action: Box::new(action)
    }))

}

pub fn parse_switch_statement(stream: &mut TokenStream) -> StatementResult {
    let keyword = stream.consume()?;
    parse_kind(stream, LeftParen)?;
    let condition = Box::new(parse_expression(stream)?); 
    parse_kind(stream, RightParen)?;
    parse_kind(stream, LeftBrace)?;
    let mut cases = vec![];
    while stream.current()?.kind != RightBrace {
        cases.push(parse_switch_case(stream)?);
    }
    stream.advance();

    Ok(StatementNode::Switch(SwitchNode{
        keyword,
        condition,
        cases
    }))
}

pub fn parse_switch_case(stream: &mut TokenStream) -> Result<SwitchCaseNode, TokenError> {
    stream.queue_cursor_element(CompletionElement::SwitchCase);
    let keyword = match parse_conditional(stream, Default) {
        Some(keyword) => keyword,
        None => {
            let keyword = parse_kind(stream, Case)?;
            parse_int(stream)?;
            keyword
        }
    };
    parse_kind(stream, Colon)?;
    let mut statements = vec![];
    while ![RightBrace, Case, Default].contains(&stream.current()?.kind) {
        statements.push(parse_statement(stream)?)
    };

    Ok(SwitchCaseNode{
        keyword,
        statements
    })
}

pub fn parse_return_statement(stream: &mut TokenStream) -> StatementResult {
    let keyword = stream.consume()?;
    let expression = match stream.current()?.kind {
        Semicolon => None,
        _ => Some(Box::new(parse_expression(stream)?)),
    };
    _ = parse_semicolon(stream);
    Ok(StatementNode::Return(ReturnNode{
        keyword, 
        expression
    }))
}
