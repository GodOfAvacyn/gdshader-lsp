use std::collections::HashMap;

use gdshader_lsp::{
    completion::{get_completion_items, get_hover_description},
    lexer::TokenStream,
    memory::Memory,
    source_code::send_errors,
    *
};
use lsp_server::{Connection, Message, Request, Response};
use lsp_types::*;
use serde_json::Value;

const TEST : bool = false;

pub struct Server<'a> {
    memories: HashMap<String, Memory>,
    root_dir: Option<String>,
    connection: &'a Connection
}
impl<'a> Server<'a> {
    pub fn open_document(&mut self, params: DidOpenTextDocumentParams) {
        let mut memory = Memory::new(&params.text_document.text, self.root_dir.clone());
        let diagnostics = memory.evaluate_new(None).clone();
        send_errors(&self.connection, &params.text_document.uri, diagnostics);
        
        self.memories.insert(params.text_document.uri.to_string(), memory);
    } 

    pub fn change_document(&mut self, params: DidChangeTextDocumentParams) {
        let maybe_memory = self.memories.get_mut(&params.text_document.uri.to_string()); 
        if let Some(memory) = maybe_memory {
            memory.root_dir = self.root_dir.clone();
            for change in params.content_changes {
                memory.apply_change(change);
            }
        }
    }

    pub fn save_document(&mut self, params: DidSaveTextDocumentParams) {
        let maybe_memory = self.memories.get_mut(&params.text_document.uri.to_string()); 
        if let Some(old_memory) = maybe_memory {
            *old_memory = Memory::new(&old_memory.get_source().get_code(), self.root_dir.clone());
            old_memory.root_dir = self.root_dir.clone();
            let diagnostics = old_memory.evaluate_new(None).clone();
            send_errors(&self.connection, &params.text_document.uri, diagnostics);
        }
    }

    pub fn get_memory_from_uri(&mut self, req: &Request) -> Option<&mut Memory> {
        let uri = req.params.get("textDocument")
            .unwrap()
            .get("uri")
            .unwrap()
            .to_string()
            .trim_matches(|x| x == '\"')
            .to_string();
        self.memories.get_mut(&uri)
    }
}

fn main() {
    if TEST { test(); return; }

    let (connection, io_threads) = Connection::stdio();

    let mut server = Server {
        memories: HashMap::new(),
        root_dir: Some("donkey".to_string()),
        connection: &connection
    };

    loop {
        match connection.receiver.recv() {
            Ok(msg) => match msg {
                Message::Notification(notif) => {
                    if notif.method == DID_OPEN {
                        let did_open_params: Result<DidOpenTextDocumentParams, _> =
                            notif.extract(DID_OPEN);
                        if let Ok(params) = did_open_params {
                            server.open_document(params);
                        }

                    } else if notif.method == DID_CHANGE {
                        let did_change_params: Result<DidChangeTextDocumentParams, _> =
                            notif.extract(DID_CHANGE);
                        if let Ok(params) = did_change_params {
                            server.change_document(params);
                        }

                    } else if notif.method == DID_SAVE {
                        let did_save_params: Result<DidSaveTextDocumentParams, _> =
                            notif.extract(DID_SAVE);
                        if let Ok(param) = did_save_params {
                            server.save_document(param)
                        }
                    }
                },
                Message::Request(req) => {
                    if connection.handle_shutdown(&req).unwrap() {
                        io_threads.join().unwrap();
                        return;
                    }
                    match handle_request(req, &mut server) {
                        Ok(response) =>
                            connection.sender.send(Message::Response(response)).unwrap(),
                        Err(ResponseError::Shutdown)=> {
                            io_threads.join().unwrap();
                            return;
                        }
                        _ => {}
                    }
                },
                Message::Response(x) => { }
            }
            Err(_) => {
                io_threads.join().unwrap();
                return
            }
        }
    }
}

pub enum ResponseError {
    DoNothing,
    Shutdown
}

pub fn handle_request(
    req: Request,
    server: &mut Server,
) -> Result<Response, ResponseError> {
    match req.method.as_str() {
        INITIALIZE => {
            server.root_dir = req.params.get("rootPath")
                .map(|x| x
                     .to_string()
                     .trim_matches(|x| x == '"')
                     .to_string()
                );
            for x in server.memories.iter_mut() {
                x.1.root_dir = server.root_dir.clone();
            }
            Ok(Response::new_ok(req.id, serde_json::to_value(
                InitializeResult {
                    capabilities: ServerCapabilities {
                        text_document_sync: Some(
                            TextDocumentSyncCapability::Kind(
                                TextDocumentSyncKind::INCREMENTAL
                            )
                        ),
                        completion_provider: Some(
                            lsp_types::CompletionOptions {
                                trigger_characters: Some(
                                    vec![
                                        "#".to_string(),
                                        ".".to_string(),
                                        ":".to_string(),
                                        "\"".to_string(),
                                    ]
                                ),
                                ..Default::default()
                            }
                        ),
                        hover_provider: Some(
                            lsp_types::HoverProviderCapability::Simple(true)
                        ),
                        ..Default::default()
                    },
                ..Default::default()
            }).unwrap()))
        }
        COMPLETION => {
            let cursor = get_cursor(&req.params);
            eprintln!("the hover cursor is {:?}", cursor);
            let maybe_memory = server.get_memory_from_uri(&req);

            if let Some(memory) = maybe_memory {
                let mut stream = TokenStream::new(&memory.get_source().get_code(), Some(cursor));
                eprintln!("and the stuff was{:?}", stream.tokens);
                let tree = parse_tokens(&mut stream);
                evaluate_tree(memory, tree);
                let names = get_completion_items(memory, cursor, &stream.cursor_element);

                Ok(Response::new_ok(
                    req.id,
                    serde_json::to_value(CompletionResponse::Array(names)).unwrap()
                ))
            } else {
                Err(ResponseError::DoNothing)
            } 
        }
        HOVER => {
            let cursor = get_cursor(&req.params);
            let maybe_memory = server.get_memory_from_uri(&req);

            if let Some(memory) = maybe_memory {
                let text = {
                    let stream = TokenStream::new(&memory.get_source().get_code(), Some(cursor));
                    let text = stream.find_cursor_text().map_or("".to_string(), |x| x);
                    text
                };
                if let Some(contents) = get_hover_description(memory, cursor, &text){
                    Ok(Response::new_ok(
                        req.id,
                        serde_json::to_value(lsp_types::Hover{
                            contents, 
                            range: None
                        }).unwrap()
                    ))
                } else { Err(ResponseError::DoNothing) }
            } else {  Err(ResponseError::DoNothing) }
        },
        DEFINITION => Err(ResponseError::DoNothing),
        EXIT => Err(ResponseError::Shutdown),
        SHUTDOWN => Err(ResponseError::Shutdown),
        _ =>Err(ResponseError::DoNothing) 
    }
}

//fn pos(a:u32, b:u32) -> Position{
//    Position::new(a,b)
//}
//
//fn range(a:u32, b:u32, c:u32, d:u32) -> Range {
//    Range::new(pos(a,b), pos(c,d))
//}
//
pub fn get_cursor(val: &Value) -> Position {
    let position_json = val.get("position").unwrap();
    let char: u32 = position_json.get("character").unwrap().as_u64().unwrap() as u32; 
    let line: u32 = position_json.get("line").unwrap().as_u64().unwrap() as u32; 
    Position{character: char, line}
}

fn test() {
}






