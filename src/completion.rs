use lsp_types::*;

use crate::{interpreter::evaluate_expression, memory::*, nodes::{ExpressionNode, TypeNode}};

fn cast_types() -> Vec<String> {
    [
        "float()",
        "int()",
        "uint()",
        "bool()",
        "vec2()",
        "vec3()",
        "vec4()",
        "ivec2()",
        "ivec3()",
        "ivec4()",
        "uvec2()",
        "uvec3()",
        "uvec4()",
        "bvec2()",
        "bvec3()",
        "bvec4()",
        "mat2()",
        "mat3()",
        "mat4()",
    ].map(|x| x.to_string()).to_vec()
}

#[derive(Clone, Debug)]
pub enum CompletionElement {
    TopLevelKeyword,
    Include,
    IncludeString,
    ShaderType,
    RenderMode,
    Uniform,
    Hint(TypeInfo),
    Precision,
    Interpolation,
    FunctionName,
    FunctionQualifier,
    Statement,
    SwitchCase,
    Type,
    Identifier(bool),
    Member(Box<ExpressionNode>),
    None
}
pub fn get_completion_items(
    memory: &mut Memory,
    cursor: Position,
    element: &CompletionElement
) -> Vec<CompletionItem>{
    let scope = memory.scopes.find_scope_from_position(cursor);
    match element {
        CompletionElement::TopLevelKeyword => {
            [
                "shader_type",
                "render_mode",
                "const",
                "varying",
                "uniform",
                "global",
                "instance",
                "group_uniforms",
                "struct",
                "void",
            ]
                .iter()
                .map(|x| CompletionItem {
                    label: x.to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    ..Default::default()
                })
                .chain(memory.get_builtin_types(1))
                .chain([CompletionItem{
                    label: "#include".to_string(),
                    kind: Some(CompletionItemKind::VALUE),
                    insert_text: Some("include".to_string()),
                    ..Default::default()
                }].into_iter())
                .collect()
        },
        CompletionElement::ShaderType => {
            [ "spatial", "canvas_item", "particles", "sky", "fog" ]
                .iter()
                .map(|x| CompletionItem {
                    label: x.to_string(),
                    kind: Some(CompletionItemKind::VALUE),
                    ..Default::default()
                })
                .collect()
        },
        CompletionElement::SwitchCase=> {
            [ "case", "default" ]
                .iter()
                .map(|x| CompletionItem {
                    label: x.to_string(),
                    kind: Some(CompletionItemKind::VALUE),
                    ..Default::default()
                })
                .collect()
        },
        CompletionElement::RenderMode => {
            memory.valid_render_modes
                .iter()
                .map(|(x,_)| CompletionItem {
                    label: x.to_string(),
                    kind: Some(CompletionItemKind::VALUE),
                    ..Default::default()
                })
                .collect()
        },
        CompletionElement::Uniform => {
            vec![CompletionItem {
                label: "uniform".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                ..Default::default()
            }]
        },
        CompletionElement::Hint(ty) => {
            memory.get_hints(ty.clone())
        },
        CompletionElement::Interpolation => {
            [ "smooth", "float", "lowp", "mediump", "highp" ]
                .iter().map(|x| CompletionItem {
                    label: x.to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    ..Default::default()
                })
                .chain(memory.get_builtin_types(0))
                .chain(memory.get_structs()).collect()
        },
        CompletionElement::Precision => {
            [ "lowp", "mediump", "highp" ]
                .iter().map(|x| CompletionItem {
                    label: x.to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    ..Default::default()
                })
                .chain(memory.get_builtin_types(0))
                .chain(memory.get_structs())
                .collect()
        },
        CompletionElement::FunctionQualifier => {
            [ "in", "out", "inout" ]
                .iter().map(|x| CompletionItem {
                    label: x.to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    ..Default::default()
                })
                .chain(memory.get_builtin_types(0))
                .chain(memory.get_structs())
                .collect()
        },
        CompletionElement::Statement => {
            [
                "for",
                "while",
                "if",
                "switch",
                "continue",
                "break",
                "return",
                "const",
                "true",
                "false"
            ]
                .iter().map(|x| CompletionItem {
                    label: x.to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    ..Default::default()
                })
                .chain(memory.get_builtin_types(scope))
                .chain(memory.get_structs())
                .chain(
                    memory.get_functions(cursor, false)
                        .into_iter()
                        .filter(|x| !cast_types().contains(&x.label))
                        .collect::<Vec<_>>())
                .chain(memory.get_variables(scope, false))
                .collect()
        }
        CompletionElement::Type => {
            memory.get_builtin_types(scope)
                .into_iter()
                .chain(memory.get_structs())
                .collect()
        },
        CompletionElement::Identifier(is_const) => {
            [
                "true",
                "false"
            ]
                .iter().map(|x| CompletionItem {
                    label: x.to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    ..Default::default()
                })
                .chain(memory.get_functions(cursor, *is_const))
                .chain(memory.get_variables(scope, *is_const))
                .chain(memory.get_structs())
                .collect()
        },
        CompletionElement::Member(member) => {
            memory.scopes.force_scope(scope);
            match evaluate_expression(memory, *member.clone()) {
                Ok(result) => {
                    let type_info = result.type_info;
                    if type_info.size != 0 {
                        return vec![];
                    }
                    match type_info {
                        _ if type_info == TypeInfo::from_str("gvec2_type") => vec!["x","y"],
                        _ if type_info == TypeInfo::from_str("gvec3_type") => vec!["x","y","z"],
                        _ if type_info == TypeInfo::from_str("gvec4_type") => vec!["x","y","z","w"],
                        _ => if let Some(struct_info) = memory.structs.get(&type_info.base) {
                            struct_info.fields.iter().map(|x| x.name.as_str()).collect()
                        } else {
                            vec![]
                        }
                    }.iter().map(|x| CompletionItem {
                        label: x.to_string(),
                        kind: Some(CompletionItemKind::FIELD),
                        ..Default::default()
                    }).collect()
                }
                _ => {
                    vec![]
                } 
            }
        },
        CompletionElement::FunctionName => {
            match memory.shader_type {
                ShaderType::Spatial => ["fragment", "vertex", "light"].iter(),
                ShaderType::CanvasItem => ["fragment", "vertex", "light"].iter(),
                ShaderType::Particles => ["start", "process"].iter(),
                ShaderType::Fog => ["sky"].iter(),
                ShaderType::Sky => ["fog"].iter(),
            }
                .map(|x| CompletionItem {
                    label: x.to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    ..Default::default()
                })
                .collect()
        }
        CompletionElement::None => vec![],
        CompletionElement::Include => {
            vec![CompletionItem {
                label: "#include".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                ..Default::default()
            }]
        },
        CompletionElement::IncludeString => {
            let root = memory.root_dir.clone().map_or("".to_string(), |x| x);
            memory.fetch_gdshaderinc_files(&root)
                .iter()
                .map(|x| CompletionItem {
                    label: format!("\"{}\"",x.to_string()),
                    kind: Some(CompletionItemKind::TEXT),
                    insert_text: Some(x.to_string()),
                    ..Default::default()
                })
                .collect()
        },
    }
}

pub fn get_hover_description(
    memory: &mut Memory,
    cursor: Position,
    text: &String,
) -> Option<HoverContents> {
    let scope = memory.scopes.find_scope_from_position(cursor);
    if let Some(info) = memory.builtin_types.get(text) {
        Some(HoverContents::Markup(MarkupContent {
            kind: MarkupKind::PlainText,
            value: format!("{}\n\n{}", text.clone(), info.description)
        }))
    } else if let Some(variable) = memory.scopes.collect_scopes_from(scope)
        .iter().find_map(|x| x.get(text)) {
        let description = variable.description.clone();
        let description = description.map_or("".to_string(), |x| format!("\n\n{}", x));
        Some(HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: format!(
                "{} {}{}",
                variable.ty.to_string(),
                text,
                description
            )
        }))
    } else if let Some(struct_info) = memory.structs.get(text) {
        let fields = struct_info.fields.iter().map(|x| {
            format!("  {} {};", x.ty.to_string(), x.name)
        }).collect::<Vec<_>>().join("\n");
        Some(HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: format!("struct {} {{\n{}\n}}", text, fields)
        }))
    } else if let Some(function) = memory.functions.get(text) {
        let signatures = function.signatures.iter().map(|x| {
            let params = x.params.iter().map(|y| {
                let qualifier = match y.qualifier {
                    Some(FunctionParamQualifier::In) => "in ",
                    Some(FunctionParamQualifier::Out) => "out ",
                    Some(FunctionParamQualifier::InOut) => "inout ",
                    None => ""
                }.to_string();
                format!("{}{} {}", qualifier, y.ty.to_string(), y.name)
            }).collect::<Vec<_>>().join(", ");
            format!("{} {} ({})", x.return_type.to_string(), text, params)
        }).collect::<Vec<_>>().join("\n");
        let description = function.description.clone().map_or("".to_string(), |x| x);
        Some(HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: format!("{}\n\n{}", signatures, description)
        }))
    } else if let Some(hint) = memory.hints.get(text) {
        let types = hint.type_info
            .iter()
            .map(|x| format!("{}", x.to_string()))
            .collect::<Vec<_>>()
            .join(",");
        let args = if hint.num_arguments.iter().any(|&x| x > 0){ "(...)" }
        else { "" };
        let description = hint.description.clone();
        Some(HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: format!("({}) {}{}\n\n{}", types, text, args, description)
        }))
    } else {
        None
    }
}





