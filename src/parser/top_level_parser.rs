use serde_json::map::Keys;

use crate::memory::TypeInfo;
use super::*;

pub fn parse_top_level(
    stream: &mut TokenStream
) -> Result<Option<TopLevelNode>,TokenError> {
    if let Ok(token) = stream.current() {
        let result = match token.kind {
            ShaderType => parse_shader_type(stream),
            RenderMode => parse_render_mode(stream),
            Const => parse_const(stream),
            Varying => parse_varying(stream),
            Uniform => parse_uniform(stream),
            Global => parse_uniform(stream),
            Instance => parse_uniform(stream),
            GroupUniforms => parse_group_uniforms(stream),
            Struct => parse_struct(stream),
            Void => parse_function(stream),
            Identifier => parse_function(stream),
            Semicolon => {
                stream.advance();
                Err(TokenError::SyntaxError)
            }
            _ => {
                stream.advance();
                Err(TokenError::SyntaxError)
            }
        };
        match result {
            Ok(declaration) => Ok(Some(declaration)),
            Err(TokenError::EofError) => {
                Ok(None)
            },
            _ => Ok(None)
        }
    } else {
        Err(TokenError::EofError)
    }
}


pub fn parse_shader_type(stream: &mut TokenStream) -> TopLevelResult {
    let keyword = stream.consume()?;

    stream.set_cursor_element(CompletionElement::ShaderType);
    let shader_type = parse_identifier(stream)?;

    _ = parse_kind(stream, Semicolon);
    Ok(TopLevelNode::ShaderType(ShaderTypeNode {
        keyword,
        shader_type
    }))
}

pub fn parse_render_mode(stream: &mut TokenStream) -> TopLevelResult {
    let keyword = stream.consume()?;
    let render_modes = parse_list(
        stream,
        Comma,
        Semicolon,
        Trailing::None,
        |s| {
            s.set_cursor_element(CompletionElement::RenderMode);
            parse_identifier(s) 
        }
    )?;

    _ = parse_kind(stream, Semicolon);
    Ok(TopLevelNode::RenderMode(RenderModeNode{
        keyword,
        render_modes 
    }))
}

pub fn parse_group_uniforms(stream: &mut TokenStream) -> TopLevelResult {
    let keyword = stream.consume()?;
    stream.set_cursor_element(CompletionElement::None);
    let group = parse_conditional(stream, Identifier);
    let subgroup = parse_conditional(stream, Dot)
        .map(|_| parse_identifier(stream))
        .transpose()?;

    _ = parse_kind(stream, Semicolon);
    Ok(TopLevelNode::GroupUniforms(GroupUniformsNode {
        keyword,
        group,
        subgroup 
    }))
}

pub fn parse_const(stream: &mut TokenStream) -> TopLevelResult {
    let keyword = stream.consume()?;

    stream.set_cursor_element(CompletionElement::Precision);
    let precision = parse_conditional(stream, Precision);
    if precision.is_some() { stream.set_cursor_element(CompletionElement::Type) }

    let value = parse_value_specifier(stream)?;
    parse_kind(stream, TokenKind::Equal)?;
    let expression: ExpressionNode = parse_expression(stream)?; // Parse Expression

    _ = parse_kind(stream, Semicolon);
    Ok(TopLevelNode::Const(ConstNode {
        keyword,
        precision,
        value: Box::new(value),
        expression: Box::new(expression)
    }))
}

pub fn parse_varying(stream: &mut TokenStream) -> TopLevelResult {
    let keyword = stream.consume()?;

    stream.set_cursor_element(CompletionElement::Interpolation);
    let interpolation = parse_conditional(stream, Interpolation);
    if interpolation.is_some() { stream.set_cursor_element(CompletionElement::Precision) }
    let precision = parse_conditional(stream, Precision);
    if precision.is_some() { stream.set_cursor_element(CompletionElement::Type) }

    let value = parse_value_specifier(stream)?;

    _ = parse_kind(stream, Semicolon);
    Ok(TopLevelNode::Varying(VaryingNode {
        keyword,
        interpolation,
        precision,
        value: Box::new(value)
    }))
}

pub fn parse_uniform(stream: &mut TokenStream) -> TopLevelResult {
    let (global_instance, keyword) = match parse_conditional(stream, Uniform) {
        Some(x) => (None, x),
        None => {
            let global_instance = stream.consume_if(|x| [Global, Instance].contains(&x.kind));
            stream.set_cursor_element(CompletionElement::Uniform);
            let keyword = stream.consume()?;
            if keyword.kind != Uniform {
                let err = stream.alert_error("Expected 'Uniform' keyword", keyword.range);
                return Err(err);
            }
            (global_instance, keyword)
        }
    };
    stream.set_cursor_element(CompletionElement::Precision);
    let precision = parse_conditional(stream, Precision);

    if precision.is_some() { stream.set_cursor_element(CompletionElement::Type) }

    let value = parse_value_specifier(stream)?;
    stream.set_cursor_element(CompletionElement::Hint(value.type_node.info.clone()));
    let hint = if parse_conditional(stream, Colon).is_some() {
        stream.set_cursor_element(CompletionElement::Hint(value.type_node.info.clone()));
        let identifier = parse_identifier(stream)?;
        let params = if parse_conditional(stream, LeftParen).is_some() {
            let list = parse_list(
                stream,
                Comma,
                RightParen,
                Trailing::None,
                |s| parse_number(s)
            )?;
            stream.advance();
            Some(list)
        } else { None };
        Some(HintNode{ identifier, params})
    } else { None };

    let expression = parse_conditional(stream, Equal)
        .map(|_| parse_expression(stream))
        .transpose()?;

    _ = parse_kind(stream, Semicolon);
    Ok(TopLevelNode::Uniform(UniformNode{
        global_instance,
        keyword,
        precision,
        value: Box::new(value),
        hint,
        expression: expression.map(|x| Box::new(x))
    }))
    
}

pub fn parse_struct(stream: &mut TokenStream) -> TopLevelResult {
    let keyword = stream.consume()?;
    stream.set_cursor_element(CompletionElement::None);
    let identifier = parse_identifier(stream)?;

    stream.set_cursor_element(CompletionElement::Type);
    parse_kind(stream, LeftBrace)?;
    let fields = parse_list(
        stream,
        Semicolon,
        RightBrace,
        Trailing::Enforced,
        |s| {
            s.set_cursor_element(CompletionElement::Type);
            parse_value_specifier(s) 
        }
    )?;
    parse_kind(stream, RightBrace)?;

    _ = parse_kind(stream, Semicolon);
    Ok(TopLevelNode::Struct(StructNode{
        keyword,
        identifier,
        fields
    }))
}

pub fn parse_function_arg(stream: &mut TokenStream) -> Result<ParamNode, TokenError> {
    let qualifier = parse_qualifier(stream);
    let value_node = parse_value_specifier(stream)?;
    Ok(ParamNode {
        qualifier,
        value_node
    })
}

pub fn parse_function(stream: &mut TokenStream) -> TopLevelResult {
    stream.set_cursor_element(CompletionElement::TopLevelKeyword);
    let mut is_void = false;
    let type_node = match parse_conditional(stream, Void) {
        Some(x) => {
            is_void = true;
            let text = x.text(stream.get_source().get_lines());
            TypeNode { info: TypeInfo::from_str(&text), range: x.range}
        }
        None => parse_type(stream)?,
    };
    if is_void {
        stream.set_cursor_element(CompletionElement::FunctionName);
    } else {
        stream.set_cursor_element(CompletionElement::None);
    }
    let identifier = parse_identifier(stream)?;

    parse_kind(stream, LeftParen)?;
    let params = parse_list(
        stream,
        Comma,
        RightParen,
        Trailing::Optional,
        |s| parse_function_arg(s)
    )?;

    parse_kind(stream, RightParen)?;
    let block = match parse_block(stream)? {
        StatementNode::Block(block) => block,
        _ => unreachable!()
    };
    stream.parsing_statement = true;
    Ok(TopLevelNode::Function(FunctionNode{
        type_node,
        identifier,
        params,
        block
    }))
}




