use std::{collections::HashSet, mem};

use crate::{memory::*, nodes::*, parser::parse_int};

use super::*;

pub fn evaluate_top_level_node(
    node: TopLevelNode,
    memory: &mut Memory,
) -> Result<(), EvaluateError> {
    match node {
        TopLevelNode::ShaderType(x) => evaluate_shader_type(memory, x),
        TopLevelNode::RenderMode(x) => evaluate_render_mode(memory, x),
        TopLevelNode::GroupUniforms(_) => Ok(()),
        TopLevelNode::Uniform(x) => evaluate_uniform(memory, x),
        TopLevelNode::Const(x) => evaluate_const(memory, x),
        TopLevelNode::Varying(x) => evaluate_varying(memory, x),
        TopLevelNode::Struct(x) => evaluate_struct(memory, x),
        TopLevelNode::Function(x) => evaluate_function(memory, x),
    }
}

pub fn evaluate_shader_type(memory: &mut Memory, node: ShaderTypeNode) -> EvaluateResult {
    let shader_type_slice = memory.get_token_text(node.shader_type);
    match shader_type_slice.as_str() {
        "spatial" => {
            memory.shader_type = ShaderType::Spatial;
            memory.valid_render_modes = spatial_render_modes(); 
        }
        "canvas_item" => {
            memory.shader_type = ShaderType::CanvasItem;
            memory.valid_render_modes = canvas_item_render_modes();
        }
        "particles" => {
            memory.shader_type = ShaderType::Particles;
            memory.valid_render_modes = particle_render_modes();
        }
        "sky" => {
            memory.shader_type = ShaderType::Sky;
            memory.valid_render_modes = sky_render_modes();
            memory.scopes.extend(sky_builtins());
        }
        "fog" => {
            memory.shader_type = ShaderType::Fog;
        }
        _ => return Err(memory.alert_error("Invalid Shader Type.", node.shader_type.range))
    }
    Ok(())
}

pub fn evaluate_render_mode(memory: &mut Memory, node: RenderModeNode) -> EvaluateResult {
    for render_mode in node.render_modes {
        let string = memory.get_token_text(render_mode);
        if memory.valid_render_modes.get(&string).is_none() {
            return Err(memory.alert_error("Invalid Render Mode.", render_mode.range));
        }
    }
    Ok(())
}

pub fn evaluate_const(memory: &mut Memory, node: ConstNode) -> EvaluateResult {
    let value = &node.value;
    let expression = Some(node.expression);
    ensure_valid_value(memory, value, expression.map(|x| *x), true)?;

    let name = memory.get_token_text(value.identifier);
    let ty = value.type_node.info.clone();
    let range = value.range;
    let (is_const, editable) = (true, false);
    memory.scopes.insert( name, ValueInfo {
        ty,
        is_const,
        editable,
        range: Some(range),
        description: None
    });
    Ok(())
}

pub fn evaluate_varying(memory: &mut Memory, node: VaryingNode) -> EvaluateResult {
    let value = &node.value;
    ensure_valid_value(memory, value, None, true)?;

    let name = memory.get_token_text(value.identifier);
    let ty = value.type_node.info.clone();
    let range = value.range;
    let (is_const, editable) = (false, true);
    memory.scopes.insert( name, ValueInfo {
        ty,
        is_const,
        editable,
        range: Some(range),
        description: None
    });
    Ok(())
}

pub fn evaluate_uniform(memory: &mut Memory, node: UniformNode) -> EvaluateResult {
    let value = &node.value;
    ensure_valid_value(memory, value, node.expression.map(|x| *x), true)?;

    let name = memory.get_token_text(value.identifier);
    let ty = value.type_node.info.clone();
    let range = value.range;

    if let Some(hint) = node.hint {
        let name = memory.get_token_text(hint.identifier);
        let maybe_hint_info = memory.hints.get(&name).map(|x| x.clone());
        if let Some(hint_info) = maybe_hint_info {
            if !hint_info.num_arguments.contains(&hint.params.map_or(0, |x| x.len())) {
                memory.alert_error("Invalid number of hint args.", hint.identifier.range);
            }
            if !(hint_info.type_info.contains(&ty)) {
                let valid_hints = hint_info.type_info.iter()
                    .map(|x| format!("'{}'", x.to_string()))
                    .collect::<Vec<_>>()
                    .join(" or ");
                let message = format!("Hint expects type {}.", valid_hints);
                memory.alert_error(&message, hint.identifier.range);
            }
        } else {
            memory.alert_error("Invalid hint.", hint.identifier.range);
        }
    }

    let (is_const, editable) = (false, false);
    memory.scopes.insert( name, ValueInfo {
        ty,
        is_const,
        editable,
        range: Some(range),
        description: None
    });
    Ok(())
}

pub fn evaluate_struct(memory: &mut Memory, node: StructNode) -> EvaluateResult {
    ensure_valid_id(memory, node.identifier)?;

    let mut fields: Vec<StructField> = vec![];
    for field in &node.fields {
        let field_name = memory.get_token_text(field.identifier);
        if fields.iter().any(|x| x.name == field_name) {
            let message = format!("Duplicate field: '{}'", field_name);
            return Err(memory.alert_error(&message, field.range)); 
        }
        ensure_valid_type(memory, &field.type_node)?;
        fields.push(StructField{
            name: field_name,
            ty: field.type_node.info.clone(),
            range: field.range
        })
    }
    memory.structs.insert(
        memory.get_token_text(node.identifier), StructInfo {
            fields,
            range: node.identifier.range
        }
    );
    Ok(())
}
   
pub fn evaluate_function(
    memory: &mut Memory,
    node: FunctionNode
) -> Result<(), EvaluateError> {
    let function_name = memory.get_token_text(node.identifier);
    ensure_valid_id(memory, node.identifier)?;
    let mut params: Vec<(FunctionParam, Range)> = vec![];
    for param in &node.params {
        let param_name = memory.get_token_text(param.value_node.identifier);
        if params.iter().any(|x| x.0.name == param_name) {
            let message = format!("Duplicate param: '{}'", param_name);
            return Err(memory.alert_error(&message, param.value_node.range)); 
        }
        ensure_valid_type(memory, &param.value_node.type_node)?;
        params.push((FunctionParam {
            name: param_name,
            ty: param.value_node.type_node.info.clone(),
            qualifier: param.qualifier.clone().map(|x| FunctionParamQualifier::from(x))
        }, param.value_node.range))
    }

    let expected = match node.type_node.info.clone() {
        ty if ty.base == "void" => None, 
        ty => Some(ty)
    };
    let function_return = FunctionReturn {
        expected,
        returned: None
    };
    let scope_type = ScopeType::Function(Box::new(function_return));

    memory.scopes.enter_scope(scope_type, node.block.range);
    for param in &params {
        memory.scopes.insert(
            param.0.name.clone(),
            ValueInfo {
                ty: param.0.ty.clone(),
                is_const: false,
                editable: true,
                range: Some(param.1),
                description: None
            }
        );
    }
    match memory.shader_type {
        ShaderType::Spatial => match function_name.as_str() {
            "vertex" => memory.scopes.extend(spatial_vertex_vars()),
            "fragment" => memory.scopes.extend(spatial_fragment_vars()),
            "light" => memory.scopes.extend(spatial_light_vars()),
            _ => {}
        }
        ShaderType::CanvasItem => match function_name.as_str() {
            "vertex" => memory.scopes.extend(canvas_item_vertex_vars()),
            "fragment" => memory.scopes.extend(canvas_item_fragment_vars()),
            "light" => memory.scopes.extend(canvas_item_light_vars()),
            _ => {}
        }
        ShaderType::Particles => match function_name.as_str() {
            "start" => {
                memory.scopes.extend(particle_start_process());
                memory.scopes.extend(particle_start());
            }
            "process" => {
                memory.scopes.extend(particle_start_process());
                memory.scopes.extend(particle_process());
            }
            _ => {}
        }
        ShaderType::Sky => if function_name == "fog" {
            memory.scopes.extend(sky_stuff())
        }
        ShaderType::Fog => if function_name == "sky" {
            memory.scopes.extend(fog_stuff())
        },
    }
    eval_block(memory, node.block);
    if !memory.scopes.assert_returned() { 
        let message = format!("Expected return type '{}'", node.type_node.info.to_string());
        _ = memory.alert_error(&message, node.identifier.range);
    }
    memory.scopes.leave_scope();

    memory.functions.insert(
        function_name, FunctionInfo {
            signatures: vec![
                FunctionSignature {
                    return_type: node.type_node.info,
                    params: params.into_iter().map(|(x, _)| x).collect()
                }
            ],
            range: Some(node.identifier.range),
            description: None,
            is_const: false
        }
    );
    Ok(())
}
