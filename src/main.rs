use lsp::{
    completion::{get_completion_items, get_hover_description},
    lexer::TokenStream,
    memory::Memory,
    source_code::send_errors,
    *
};
use lsp_server::{Connection, Message, Response, Request};
use lsp_types::*;

const TEST : bool = false;

fn main() {
    if TEST { test(); return; }

    let mut source = String::new();
    let mut memory = Memory::new("");

    let (connection, io_threads) = Connection::stdio();

    loop {
        match connection.receiver.recv() {
            Ok(msg) => match msg {
                Message::Notification(notif) => {
                    if notif.method == DID_OPEN {
                        let did_open_params: Result<DidOpenTextDocumentParams, _> =
                            notif.extract(DID_OPEN);
                        if let Ok(params) = did_open_params {
                            source = params.text_document.text.clone();
                            memory = Memory::new(&source);

                            let mut stream = TokenStream::new(&source, None);
                            let tree = parse_tokens(&mut stream);
                            evaluate_tree(&mut memory, tree);

                            let mut diagnostics = stream.get_source().get_diagnostics().clone();
                            diagnostics.extend(memory.get_source().get_diagnostics().clone());
                            send_errors(&connection, &params.text_document.uri, diagnostics)
                        }

                    } else if notif.method == DID_CHANGE {
                        let did_change_params: Result<DidChangeTextDocumentParams, _> =
                            notif.extract(DID_CHANGE);
                        if let Ok(params) = did_change_params {
                            for change in params.content_changes {
                                apply_change(&mut source, &change);
                            }
                        }

                    } else if notif.method == DID_SAVE {
                        let did_save_params: Result<DidSaveTextDocumentParams, _> =
                            notif.extract(DID_SAVE);
                        if let Ok(param) = did_save_params {
                            memory = Memory::new(&source);

                            let mut stream = TokenStream::new(&source, None);
                            let tree = parse_tokens(&mut stream);
                            evaluate_tree(&mut memory, tree);

                            let mut diagnostics = stream.get_source().get_diagnostics().clone();
                            diagnostics.extend(memory.get_source().get_diagnostics().clone());
                            send_errors(&connection, &param.text_document.uri, diagnostics)
                        }
                    }
                },
                Message::Request(req) => {
                    if connection.handle_shutdown(&req).unwrap() {
                        io_threads.join().unwrap();
                        return;
                    }
                    match handle_request(req, &mut source, &mut memory) {
                        Some(response) =>
                            connection.sender.send(Message::Response(response)).unwrap(),
                        None => ()
                    }
                },
                Message::Response(_) => {}
            }
            Err(_) => {
                io_threads.join().unwrap();
                return
            }
        }
    }
}

pub fn handle_request(
    req: Request,
    source: &String,
    memory: &mut Memory
) -> Option<Response> {
    match req.method.as_str() {
        INITIALIZE => {
            Some(Response::new_ok(req.id, serde_json::to_value(
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
                                    vec![".".to_string(), ":".to_string()]
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
            let position_json = req.params.get("position").unwrap();
            let char: u32 = position_json.get("character").unwrap().as_u64().unwrap() as u32; 
            let line: u32 = position_json.get("line").unwrap().as_u64().unwrap() as u32; 
            let cursor = Position{character: char, line};

            let mut stream = TokenStream::new(&source, Some(cursor));
            let tree = parse_tokens(&mut stream);
            evaluate_tree(memory, tree);
            
            let names = get_completion_items(memory, cursor, &stream.cursor_element);

            Some(Response::new_ok(
                req.id,
                serde_json::to_value(CompletionResponse::Array(names)).unwrap()
            ))
        }
        HOVER => {
            let position_json = req.params.get("position").unwrap();
            let char: u32 = position_json.get("character").unwrap().as_u64().unwrap() as u32; 
            let line: u32 = position_json.get("line").unwrap().as_u64().unwrap() as u32; 
            let cursor = Position{character: char, line};

            let (text, scope) = {
                let mut stream = TokenStream::new(&source, Some(cursor));
                let text = stream.find_cursor_text().map_or("".to_string(), |x| x);
                let scope = memory.scopes.find_scope_from_position(cursor);
                (text, scope)
            };
    
            if let Some(contents) = get_hover_description(memory, cursor, &text){
                Some(Response::new_ok(
                    req.id,
                    serde_json::to_value(lsp_types::Hover{
                        contents, 
                        range: None
                    }).unwrap()
                ))
            } else { None }
        },
        DEFINITION => {
            panic!()
        },
        EXIT => panic!(),
        SHUTDOWN => panic!(),
        _ => None
    }
}

fn test() {
    let source = String::from(
"shader_type spatial;

void light() {


}"
    );
    let mut memory = Memory::new(&source);
    let mut stream = TokenStream::new(&source, Some(Position::new(1,0)));
    let tree = parse_tokens(&mut stream);
    evaluate_tree(&mut memory, tree);

    let mut one = 0;
    for a in memory.scopes.scopes.iter().map(|x| x.range) {
        eprintln!("{}, {:?}", one, a);
        one += 1;
    }
}






