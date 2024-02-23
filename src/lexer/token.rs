use std::usize;

use lsp_types::{Position, Range};
use strum_macros::AsRefStr;
use logos::{skip, Logos, Skip, Source};

#[derive(Clone, Copy, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub range: Range
}

impl Token {
    pub fn as_ref(&self) -> &str {
        self.kind.as_ref()
    } 

    pub fn text(&self, source: &String) -> String {
        let line = &source.lines().nth(self.range.start.line as usize);
        if let Some(line) = line {
            let start = self.range.start.character as usize;
            let end = self.range.end.character as usize;
            let span = start .. end;

            line.slice(span).map_or("".to_string(), |x| x.to_string())
        } else {
            "".to_string()
        }
    }
}

pub trait ExtraRange {
    fn contains_position(&self, position: Position) -> bool;
    fn preceeds_position(&self, position: Position) -> bool;
}
impl ExtraRange for Range {
    fn contains_position(&self, position: Position) -> bool {
        if position.line > self.start.line &&
            position.line < self.end.line { return true }
    
        let after_start = position.character >= self.start.character;
        let before_end = position.character <= self.end.character;
        let on_start_line = position.line == self.start.line;
        let on_end_line = position.line == self.end.line;
        
        if on_start_line && on_end_line  {
            return after_start && before_end
        }

        if on_start_line {
            return after_start;
        }
        if on_end_line {
            return before_end;
        }

        false
    }

    fn preceeds_position(&self, position: Position) -> bool {
        if position.character == 0 { return false }
        position.line > self.end.line ||
            (position.line == self.end.line &&
             position.character - 1 >= self.end.character)
    }
}

#[derive(AsRefStr, Copy, Clone, Debug, Eq, Logos, PartialEq)]
#[logos(extras=Vec<usize>, skip r"[ \t\f]+")]
pub enum TokenKind {
    // Preprocessors
    #[token("#include")] Include,
    // Top Level Keywords
    #[token("shader_type")] ShaderType,
    #[token("render_mode")] RenderMode,
    #[token("group_uniforms")] GroupUniforms,
    #[token("global")] Global,
    #[token("instance")] Instance,
    #[token("const")] Const,
    #[token("uniform")] Uniform,
    #[token("varying")] Varying,
    #[token("struct")] Struct,
    #[token("void")] Void,
    // Keywords
    #[token("break")] Break,
    #[token("continue")] Continue,
    #[token("while")] While,
    #[token("do")] Do,
    #[token("else")] Else,
    #[token("for")] For,
    #[token("if")] If,
    #[token("discard")] Discard,
    #[token("return")] Return,
    #[token("switch")] Switch,
    #[token("case")] Case,
    #[token("default")] Default,
    #[token("in")] In,
    #[token("out")] Out,
    #[token("inout")] InOut,
    #[regex("flat|smooth")] Interpolation,
    #[regex("lowp|mediump|highp")] Precision,
    // Double Operations 
    #[token("+=")] AddAssign,
    #[token("-=")] SubAssign,
    #[token("*=")] MulAssign,
    #[token("/=")] DivAssign,
    #[token("%=")] ModAssign,
    #[token("++")] Increment,
    #[token("--")] Decrement,
    #[token("&&")] And,
    #[token("||")] Or,
    #[token("<<")] LeftOp,
    #[token(">>")] RightOp,
    #[token("<=")] LeqOp,
    #[token(">=")] GeqOp,
    #[token("==")] EqOp,
    #[token("!=")] NeqOp,
    // Single Operations
    #[token("=")] Equal,
    #[token("!")] Bang,
    #[token("-")] Dash,
    #[token("~")] Tilde,
    #[token("+")] Plus,
    #[token("*")] Star,
    #[token("/")] Slash,
    #[token("%")] Percent,
    #[token("<")] LeftAngle,
    #[token(">")] RightAngle,
    #[token("|")] VerticalBar,
    #[token("^")] Caret,
    #[token("&")] Ampersand,
    #[token("?")] Question,
    // Punctiation
    #[token("(")] LeftParen,
    #[token(")")] RightParen,
    #[token("[")] LeftBracket,
    #[token("]")] RightBracket,
    #[token("{")] LeftBrace,
    #[token("}")] RightBrace,
    #[token(".")] Dot,
    #[token(",")] Comma,
    #[token(":")] Colon,
    #[token(";")] Semicolon,
    // Other
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
    #[regex("[0-9]+")]
    IntConstant,
    #[regex("[0-9]+u")]
    UintConstant,
    #[regex("[0-9]*\\.[0-9]+|[0-9]+\\.[0-9]*(f)?")]
    FloatConstant,
    #[regex("true|false")]
    BoolConstant,
    #[regex("\"(?:[^\"]|\\\\\")*\"")]
    String,
    #[regex("//[^\n]*", skip)]
    #[regex("/\\*(?:[^*]|\\*[^/])*\\*/", skip)]
    Comment,
    #[regex(r"\n", newline_callback)]
    Newline,
    Error
}

fn newline_callback(lex: &mut logos::Lexer<TokenKind>) -> Skip {
    lex.extras.push(lex.span().end);
    Skip
}

