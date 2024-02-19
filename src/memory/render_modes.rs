use std::collections::HashMap;

pub struct RenderModeInfo {
    pub description: String
}

macro_rules! render_mode {
    ($name:ident $description:literal) => {
        (stringify!($name).to_string(), $description.to_string()) 
    };
}

pub fn spatial_render_modes() -> HashMap<String, String> {
HashMap::from([
    render_mode!(blend_mix "Mix blend mode (alpha is transparency), default."),
    render_mode!(blend_add "Additive blend mode."),
    render_mode!(blend_sub "Subtractive blend mode."),
    render_mode!(blend_mul "Multiplicative blend mode."),
    render_mode!(depth_draw_opaque "Only draw depth for opaque geometry (not transparent)."),
    render_mode!(depth_draw_always "Always draw depth (opaque and transparent)."),
    render_mode!(depth_draw_never "Never draw depth."),
    render_mode!(depth_prepass_alpha "Do opaque depth pre-pass for transparent geometry."),
    render_mode!(depth_test_disabled "Disable depth testing."),
    render_mode!(sss_mode_skin "Subsurface Scattering mode for skin."),
    render_mode!(cull_back "Cull back-faces (default)."),
    render_mode!(cull_front "Cull front-faces."),
    render_mode!(cull_disabled "Culling disabled (double sided)."),
    render_mode!(unshaded "Result is just albedo. No lighting/shading happens in material."),
    render_mode!(wireframe "Geometry draws using lines."),
    render_mode!(diffuse_burley "Burley (Disney PBS) for diffuse (default)."),
    render_mode!(diffuse_lambert "Lambert shading for diffuse."),
    render_mode!(diffuse_lambert_wrap "Lambert wrapping (roughness dependent) for diffuse."),
    render_mode!(diffuse_toon "Toon shading for diffuse."),
    render_mode!(specular_schlick_ggx "Schlick-GGX for specular (default)."),
    render_mode!(specular_toon "Toon for specular."),
    render_mode!(specular_disabled "Disable specular."),
    render_mode!(skip_vertex_transform
"VERTEX/NORMAL/etc. need to be transformed manually in vertex function."),
    render_mode!(world_vertex_coords
"VERTEX/NORMAL/etc. are modified in world coordinates instead of local."),
    render_mode!(ensure_correct_normals "Use when non-uniform scale is applied to mesh."),
    render_mode!(shadows_disabled "Disable computing shadows in shader."),
    render_mode!(ambient_light_disabled
"Disable contribution from ambient light and radiance map."),
    render_mode!(shadow_to_opacity
"Lighting modifies the alpha so shadowed areas are opaque and non-shadowed
areas are transparent. Useful for overlaying shadows onto a camera feed in AR."),
    render_mode!(vertex_lighting "Use vertex-based lighting."),
    render_mode!(particle_trails "Enables the trails when used on particles geometry."),
    render_mode!(alpha_to_coverage "Alpha antialiasing mode."),
    render_mode!(alpha_to_coverage_and_one "Alpha antialiasing mode."),
    render_mode!(fog_disabled
"Disable receiving depth-based or volumetric fog. Useful for blend_add materials
like particles."),
])}

pub fn canvas_item_render_modes() -> HashMap<String, String> {
    HashMap::from([
        render_mode!(blend_mix "Mix blend mode (alpha is transparency), default."),
        render_mode!(blend_add "Additive blend mode."),
        render_mode!(blend_sub "Subtractive blend mode."),
        render_mode!(blend_mul "Multiplicative blend mode."),
        render_mode!(blend_premul_alpha "Pre-multiplied alpha blend mode."),
        render_mode!(blend_disabled
"Disable blending, values (including alpha) are written as-is."),
        render_mode!(unshaded "Result is just albedo. No lighting/shading happens in material."),
        render_mode!(light_only "Only draw on light pass."),
        render_mode!(skip_vertex_transform
"VERTEX needs to be transformed manually in vertex function."),
        render_mode!(world_vertex_coords
"VERTEX is modified in world coordinates instead of local."),
    ])
}
pub fn particle_render_modes() -> HashMap<String, String> {
    HashMap::from([
        render_mode!(keep_data "Do not clear previous data on restart."),
        render_mode!(disable_force "Disable attractor force."),
        render_mode!(disable_velocity "Ignore VELOCITY value."),
        render_mode!(collision_use_scale "Scale the particle's size for collisions."),
    ])
}

pub fn sky_render_modes() -> HashMap<String, String> {
    HashMap::from([
        render_mode!(use_half_res_pass
"Allows the shader to write to and access the half resolution pass."),
        render_mode!(use_quarter_res_pass
"Allows the shader to write to and access the quarter resolution pass."),
        render_mode!(disable_fog "If used, fog will not affect the sky."),
    ])
}



