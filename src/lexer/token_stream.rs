use logos::{Logos, Lexer};
use lsp_types::{Position, Range};
use crate::{completion::CompletionElement, source_code::SourceDocument};
use super::{ExtraRange, Token, TokenKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenError {
    EofError,
    SyntaxError
}

pub type TokenResult = Result<Token, TokenError>;

pub struct TokenStream {
    current_idx: usize,
    pub tokens: Vec<Token>,
    source: SourceDocument,
    cursor: Option<Position>,
    pub cursor_element: CompletionElement,
    cursor_element_queue: CompletionElement,
    pub parsing_const: bool,
    pub parsing_statement: bool,
    record_errors: bool,
}

impl TokenStream {
    pub fn new(source_str: &str, cursor: Option<Position>) -> Self {
        let mut lexer = <TokenKind as Logos>::lexer(&source_str); 
        let mut tokens = vec![];
        let mut next_kind = lexer.next();
        let mut range = get_lexer_current_range(&lexer);

        while let Some(maybe_kind) = next_kind {
            match maybe_kind {
                Ok(kind) => tokens.push(Token { kind, range }),
                Err(_) => tokens.push(Token { kind: TokenKind::Error, range }),
            }
            next_kind = lexer.next();
            range = get_lexer_current_range(&lexer);
        };

        let source = SourceDocument::new(source_str);

        Self {
            current_idx: 0,
            tokens,
            source,
            cursor,
            cursor_element: CompletionElement::TopLevelKeyword,
            cursor_element_queue: CompletionElement::TopLevelKeyword,
            parsing_const: false,
            parsing_statement: false,
            record_errors: true,
        }
    }

    pub fn destroy(self) -> (Vec<Token>, SourceDocument) {
        (self.tokens, self.source)
    }

    pub fn get_source(&self) -> &SourceDocument {
        &self.source
    }

    pub fn current(&self) -> TokenResult {
        match self.tokens.get(self.current_idx) {
            Some(&x) => Ok(x),
            _ => Err(TokenError::EofError)
        }
    }

    pub fn current_idx(&self) -> usize {
        self.current_idx
    }

    pub fn turn_off_errors(&mut self) {
        self.record_errors = false;
    }
    
    pub fn turn_on_errors(&mut self) {
        self.record_errors = true;
    }

    pub fn force_change_position(&mut self, idx: usize) {
        self.current_idx = idx;
    }

    pub fn queue_cursor_element(
        &mut self,
        element: CompletionElement
    ) {
        self.cursor_element_queue = element;
    }

    pub fn advance(&mut self) {
        if let Some(cursor) = self.cursor {
            if let Ok(current) = self.current() {
                if current.range.contains_position(cursor) {
                    self.cursor_element = self.cursor_element_queue.clone();
                }
            };
        }
        self.current_idx += 1;
        self.parsing_statement = false;
    }

    pub fn retreat(&mut self) {
        self.current_idx -= 1;
    }

    pub fn advance_with<T>(&mut self, data: T) -> T {
        let current = self.current(); 
        self.advance();
        data
    }

    pub fn consume(&mut self) -> TokenResult {
        let current = self.current();
        self.advance();
        current 
    }

    pub fn consume_if<F>(&mut self, f: F) -> Option<Token>
    where
        F: Fn(Token) -> bool,
    {
        match self.current() {
            Ok(x) => if f(x) { self.advance(); Some(x) } else { None },
            _ => None,
        }
    }

    pub fn consume_token_kind(&mut self, expected: TokenKind) -> TokenResult {
        let message = format!("Expected {}", expected.as_ref()); 
        match self.current()? {
            e if e.kind == expected => self.consume(),
            e => Err(self.alert_error(&message, e.range))
        }
    }

    pub fn alert_error(&mut self, msg: &str, range: Range) -> TokenError {
        if !self.record_errors {
            return TokenError::SyntaxError;
        }
        let message = format!("Syntax Error: {}", msg); 
        self.source.push_error(msg, range, TokenError::SyntaxError)
    }

    pub fn alert_error_with<T>(&mut self, msg: &str, range: Range, data: T) -> T {
        if !self.record_errors {
            return data;
        }
        let message = format!("Syntax Error: {}", msg); 
        self.source.push_error(msg, range, TokenError::SyntaxError);
        data
    }

    pub fn find_cursor_text(&self) -> Option<String> {
        if let Some(cursor) = self.cursor {
            for token in &self.tokens {
                if token.range.contains_position(cursor) {
                    return Some(token.text(&self.source.get_code()))
                }
            }
        }
        None
    }
}

fn get_lexer_current_range<'a>(lexer: &Lexer<'a, TokenKind>) -> Range {
    let span = lexer.span();
    let lines = &lexer.extras;
    let current_line = lines.iter().filter(|&&c| c <= span.start).count();
    let last_line_end = if current_line == 0 { 0 } else { lines[current_line - 1] };
    let current_character = span.start - last_line_end;
    let last_character = current_character + span.len();

    Range::new(
        Position::new(current_line as u32, current_character as u32),
        Position::new(current_line as u32, last_character as u32)
    )
}
