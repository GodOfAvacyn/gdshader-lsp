use lsp_server::{Connection, Message, Notification};
use lsp_types::*;

use crate::lexer::{Token, TokenError, TokenKind};

pub struct SourceDocument {
    lines: Vec<String>,
    diagnostics: Vec<Diagnostic>
}
impl SourceDocument {
    pub fn new(source: &str) -> Self {
        let lines = source.split("\n").map(|x| x.to_string()).collect();
        let diagnostics = vec![];
        Self {lines, diagnostics}
    }

    pub fn push_error<T>(&mut self, msg: &str, range: Range, error: T) -> T{
        self.diagnostics.push(create_diagnostic(range, msg));
        error
    }
    
    pub fn push_eof_error(&mut self, range: Range) -> TokenError {
        self.diagnostics.push(create_diagnostic(range, "Unexpected EOF"));
        TokenError::EofError
    }

    pub fn force_change_diagnostics(&mut self, diagnostics: Vec<Diagnostic>) {
        self.diagnostics = diagnostics;
    }

    pub fn get_lines(&self) -> &Vec<String> {
        &self.lines
    }

    pub fn get_diagnostics(&self) -> &Vec<Diagnostic> {
        &self.diagnostics
    }
}

fn create_unused_warning(range: Range, id_name: &str) -> Diagnostic {
    Diagnostic {
        range,
        severity: Some(DiagnosticSeverity::WARNING),
        code: None,
        code_description: None,
        source: Some("GDShaderServer".to_string()),
        message: format!("'{}' is unused.", id_name),
        related_information: None,
        tags: Some(vec![DiagnosticTag::UNNECESSARY]),
        data: None,
    }
}

fn create_diagnostic(range: Range, message: &str) -> Diagnostic {
    Diagnostic {
        range,
        severity: Some(DiagnosticSeverity::ERROR),
        code: None,
        code_description: None,
        source: Some("GDShaderServer".to_string()),
        message: message.to_string(),
        related_information: None,
        tags: None,
        data: None,
    }
}

pub fn send_errors(connection: &Connection, uri: &Url, diagnostics: Vec<Diagnostic>) {
    let params = PublishDiagnosticsParams {
        uri: uri.clone(),
        diagnostics: diagnostics.clone(),
        version: None,
    };

    connection.sender.send(Message::Notification(Notification {
        method: "textDocument/publishDiagnostics".to_string(),
        params: serde_json::to_value(params).unwrap(),
    })).unwrap();
}

