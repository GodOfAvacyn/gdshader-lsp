use crate::{completion::CompletionElement, lexer::*, memory::TypeInfo, nodes::*};
use lsp_types::Range;
use TokenKind::*;

mod top_level_parser;
mod expression;
mod statement;
pub use top_level_parser::*;
pub use expression::*;
pub use statement::*;


pub fn parse_kind(stream: &mut TokenStream, kind: TokenKind) -> TokenResult {
    stream.set_cursor_element(CompletionElement::None);
    stream.consume_token_kind(kind)
}

pub fn parse_conditional(stream: &mut TokenStream, kind: TokenKind) -> Option<Token> {
    stream.consume_if(|x| x.kind == kind)
}

pub fn parse_semicolon(stream: &mut TokenStream) -> TokenResult {
    stream.consume_token_kind(Semicolon)
}

pub fn parse_identifier(stream: &mut TokenStream) -> TokenResult {
    stream.consume_token_kind(Identifier)
}

pub fn parse_primitive(stream: &mut TokenStream) -> Option<Token> {
    stream.consume_if(|x| match x.kind {
        BoolConstant | IntConstant | UintConstant | FloatConstant => true,
        _ => false
    })
}

pub fn parse_qualifier(stream: &mut TokenStream) -> Option<Token> {
    stream.set_cursor_element(CompletionElement::FunctionQualifier);
    stream.consume_if(|x| match x.kind {
        In | Out | InOut => {
            true
        },
        _ => false
    })
}

pub fn parse_binary_operation(stream: &mut TokenStream) -> Option<Token> {
    stream.consume_if(|t| t.to_binary_op().is_some())
}

pub fn parse_unary_operation(stream: &mut TokenStream) -> Option<Token> {
    stream.consume_if(|t| t.to_unary_op().is_some())
}

pub fn parse_assignment_operation(stream: &mut TokenStream) -> Option<Token> {
    stream.consume_if(|t| t.to_assignment_op().is_some())
}

pub fn parse_increment_operation(stream: &mut TokenStream) -> Option<Token> {
    stream.consume_if(|t| match t.kind {
        Increment | Decrement => true,
        _ => false
    })
}

pub fn parse_int(stream: &mut TokenStream) -> Result<Token, TokenError> {
    let message = "Expected integer constant.";
    let current = stream.current()?;
    match current.kind {
        IntConstant | UintConstant => stream.consume(),
        _ => Err(stream.alert_error(message, current.range))
    }
}

pub fn parse_number(stream: &mut TokenStream) -> Result<Token, TokenError> {
    let message = "Expected number";
    let current = stream.current()?;
    match current.kind {
        IntConstant | UintConstant | FloatConstant => stream.consume(),
        _ => Err(stream.alert_error(message, current.range))
    }
}

pub fn parse_positive_int(stream: &mut TokenStream) -> Option<u32> {
    let lines = stream.get_source().get_lines();
    if stream.current().is_err() { return None }

    let current = stream.current().unwrap();
    match current.kind {
        IntConstant => match current.text(lines).parse::<u32>() {
            Ok(x) => stream.advance_with(Some(x)),
            _ => None
        },
        UintConstant => {
            let text = current.text(lines);
            let without_u = &text[..text.len() - 1];
            match without_u.parse::<u32>() {
                Ok(x) => stream.advance_with(Some(x)),
                _ => None
            }
        }
        _ => None
    }
}

pub fn parse_size(stream: &mut TokenStream) -> Result<(u32, Option<Range>), TokenError> {
    let message = "Expected positive integer constant.";
    if stream.consume_if(|x| x.kind == LeftBracket).is_some() {
        let range = stream.current()?.range;
        let size = match parse_positive_int(stream) {
            None => Err(stream.alert_error(message, range)),
            Some(x) if x == 0 => Err(stream.alert_error(message, range)),
            Some(x) => Ok(x)
        }?;
        Ok((size, Some(range)))
    } else {
        Ok((0, None))
    }
}

pub fn parse_type(stream: &mut TokenStream) -> Result<TypeNode, TokenError> {
    let id = parse_identifier(stream)?;

    let (size, size_range) = parse_size(stream)?;
    let range = Range::new(
        id.range.start,
        size_range.map_or(id.range.end, |x| x.end)
    );
    let base = id.text(stream.get_source().get_lines());
    Ok(TypeNode { info: TypeInfo {base, size}, range })

}

pub fn parse_value_specifier(stream: &mut TokenStream) -> Result<ValueNode, TokenError> {
    let mut type_node = parse_type(stream)?;

    stream.set_cursor_element(CompletionElement::None);
    let identifier = parse_identifier(stream)?;

    let (other_size, other_size_range) = parse_size(stream)?;
    
    let range = Range::new(
        type_node.range.start,
        other_size_range.map_or(identifier.range.end, |x| x.end)
    );
    
    if other_size != 0 {
        if type_node.info.size != 0 {
            let message = "Array size cannot be defined twice.";
            return Err(stream.alert_error(message, range));
        } else {
            type_node.info.size = other_size;
        }
    }

    Ok(ValueNode { identifier, type_node, range })
}

#[derive(PartialEq, Eq)]
enum Trailing{
    Enforced,
    Optional,
    None
}

/// Designed to keep pushing forward as much as possible. If the ending is not
/// reached it will parse the whole document.
pub fn parse_list<F, T>(
    stream: &mut TokenStream,
    separator: TokenKind,
    stop: TokenKind,
    trailing: Trailing,
    content: F
) -> Result<Vec<T>, TokenError>
where
    F: Fn(&mut TokenStream) -> Result<T, TokenError> 
{
    use Trailing::*;
    let mut parsing_separator = false;
    let mut vec = vec![];
    let bad_stop_err = format!("Unexpected {}", stop.as_ref());
    let sep_error = match trailing {
        Optional | None => format!("Expected {} or {}", separator.as_ref(), stop.as_ref()),
        Enforced => format!("Expected {}", separator.as_ref()),
    };

    loop { match parsing_separator {
        true => match stream.current()? {
            t if t.kind == stop => match trailing {
                Optional | None => break Ok(vec),
                Enforced => break stream.alert_error_with(&sep_error, t.range, Ok(vec)),
            }
            t if t.kind == separator => parsing_separator = stream.advance_with(false),
            t => parsing_separator = stream.alert_error_with(&sep_error, t.range, false),
        },
        false => match stream.current()? {
            t if t.kind == stop => match trailing {
                Optional | Enforced => break Ok(vec),
                None => break stream.alert_error_with(&bad_stop_err, t.range, Ok(vec)),
            },
            _ => parsing_separator = match content(stream) {
                Ok(ok) => {vec.push(ok); true},
                Err(TokenError::EofError) => break Err(TokenError::EofError),
                _ => stream.advance_with(true)
            }
        }
    }}
}



