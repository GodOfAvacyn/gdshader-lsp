use std::{collections::HashMap, hash::Hash};

use super::{TypeInfo, ValueInfo};

macro_rules! builtin_variable {
    ((const $type:ident $name:ident $description:literal)) => {
        builtin_variable!(@main $type $name true $description) 
    };
    (($type:ident $name:ident $description:literal)) => {
        builtin_variable!(@main $type $name false $description) 
    };
    (@main $type:ident $name:ident $const:literal $description:literal) => {
        (stringify!($name).to_string(), ValueInfo {
            ty: TypeInfo::from_str(stringify!($type)),
            editable: !$const,
            is_const: $const,
            range: None,
            description: Some($description.to_string())
        })
    };
}

macro_rules! var_hashmap {
    ($($var:tt,)*) => {
        Vec::from([
            $(builtin_variable!($var),)*
        ]) 
    };
}

//#[derive(Clone, Debug)]
//pub struct ValueInfo {
//    pub ty: TypeInfo,
//    pub editable: bool,
//    pub is_const: bool,
//    pub range: Range,
//    pub description: Option<String>
//}


pub fn variable_builtins() -> Vec<(String, ValueInfo)> {
    var_hashmap!(
        (const float TIME
"Global time since the engine has started, in seconds (always positive).
It's subject to the rollover setting (which is 3,600 seconds by default).
It's not affected by time_scale or pausing, but you can define a global shader uniform
to add a 'scaled' TIME variable if desired."),
        (const float PI
"A PI constant (3.141592). A ratio of circle's circumference to its diameter and
amount of radians in half turn."),
        (const float TAU
"A TAU constant (6.283185). An equivalent of PI * 2 and amount of radians in full turn."),
        (const float E
"A E constant (2.718281). Euler's number and a base of the natural logarithm."),
    )
}

pub fn spatial_vertex_vars() -> Vec<(String, ValueInfo)> {
    var_hashmap!(
        (const vec2 VIEWPORT_SIZE "Size of viewport (in pixels.)"),
        (const mat4 VIEW_MATRIX "World space to view space transform."),
        (const mat4 INV_VIEW_MATRIX "View space to world space transform."),
        (const mat4 INV_PROJECTION_MATRIX "Clip space to view space transform."),
        (const vec3 NODE_POSITION_WORLD "Node world space position."),
        (const vec3 NODE_POSITION_VIEW "Node view space position."),
        (const vec3 CAMERA_POSITION_WORLD "Camera world space position."),
        (const vec3 CAMERA_DIRECTION_WORLD "Camera world space direction."),
        (const bool OUTPUT_IS_SRGB
"True when output is in sRGB color space (this is true in the Compatibility
renderer, false in Forward+ and Forward Mobile)."),
        (const int INSTANCE_ID "Instance ID for instancing."),
        (const vec4 INSTANCE_CUSTOM "Instance custom data (for particles, mostly)."),
        (const int VIEW_INDEX
"The view that we are rendering. VIEW_MONO_LEFT (0) for Mono (not multiview) or
left eye, VIEW_RIGHT (1) for right eye."),
        (const int VIEW_MONO_LEFT "Constant for Mono or left eye, always 0."),
        (const int VIEW_RIGHT "Constant for right eye, always 1."),
        (const vec3 EYE_OFFSET
"Position offset for the eye being rendered. Only applicable for multiview rendering."),
        (vec3 VERTEX "Vertex in local coordinates."),
        (const int VERTEX_ID "The index of the current vertex in the vertex buffer."),
        (vec3 NORMAL "Normal in local coordinates."),
        (vec3 TANGENT "Tangent in local coordinates."),
        (vec3 BINORMAL "Binormal in local coordinates."),
        (vec4 POSITION "If written to, overrides final vertex position."),
        (vec2 UV "UV main channel."),
        (vec2 UV2 "UV secondary channel."),
        (vec4 COLOR "Color from vertices."),
        (float ROUGHNESS "Roughness for vertex lighting."),
        (float POINT_SIZE "Point size for point rendering."),
        (mat4 MODELVIEW_MATRIX "Model space to view space transform (use if possible)."),
        (mat3 MODELVIEW_NORMAL_MATRIX ""),
        (mat4 MODEL_MATRIX "Model space to world space transform."),
        (mat3 MODEL_NORMAL_MATRIX ""),
        (mat4 PROJECTION_MATRIX "View space to clip space transform."),
    )
}

pub fn spatial_fragment_vars() -> Vec<(String, ValueInfo)> {
    var_hashmap!(
        (const vec2 VIEWPORT_SIZE "Size of viewport (in pixels.)"),
        (const vec4 FRAGCOORD
"Coordinate of pixel center in screen space. xy specifies position in window,
z specifies fragment depth if DEPTH is not used. Origin is lower-left."),
        (const bool FRONT_FACING "true if current face if front face."),
        (const vec3 VIEW
"Normalized vector from fragment position to camera (in view space).
This is the same for both perspective and orthogonal cameras."),
        (const vec2 UV "UV that comes from vertex function."),
        (const vec2 UV2 "UV2 that comes from vertex function."),
        (const vec4 COLOR "COLOR that comes from vertex function."),
        (const vec2 POINT_COORD "Point Coordinate for drawing points with POINT_SIZE."),
        (const bool OUTPUT_IS_SRGB
"true when output is in sRGB color space (this is true in the Compatibility renderer,
false in Forward+ and Forward Mobile)."),
        (const mat4 MODEL_MATRIX "Model space to world space transform."),
        (const mat3 MODEL_NORMAL_MATRIX ""),
        (const mat4 VIEW_MATRIX "World space to view space transform."),
        (const mat4 INV_VIEW_MATRIX "View space to world space transform."),
        (const mat4 PROJECTION_MATRIX "View space to clip space transform."),
        (const mat4 INV_PROJECTION_MATRIX "Clip space to view space transform."),
        (const vec3 NODE_POSITION_WORLD "Node position, in world space."),
        (const vec3 NODE_POSITION_VIEW "Node position, in view space."),
        (const vec3 CAMERA_POSITION_WORLD "Camera position, in world space."),
        (const vec3 CAMERA_DIRECTION_WORLD "Camera direction, in world space."),
        (const vec3 VERTEX "Vertex that comes from vertex function (default, in view space)."),
        (const int VIEW_INDEX
"The view that we are rendering. VIEW_MONO_LEFT (0) for Mono (not multiview) or
left eye, VIEW_RIGHT (1) for right eye."),
        (const int VIEW_MONO_LEFT "Constant for Mono or left eye, always 0."),
        (const int VIEW_RIGHT "Constant for right eye, always 1."),
        (const vec3 EYE_OFFSET
"Position offset for the eye being rendered. Only applicable for multiview rendering."),
        (const vec2 SCREEN_UV "Screen UV coordinate for current pixel."),
        (float DEPTH
"Custom depth value (0..1). If DEPTH is being written to in any shader branch,
then you are responsible for setting the DEPTH for all other branches. Otherwise,
the graphics API will leave them uninitialized."),
        (vec3 NORMAL "Normal that comes from vertex function (default, in view space)."),
        (vec3 TANGENT "Tangent that comes from vertex function."),
        (vec3 BINORMAL "Binormal that comes from vertex function."),
        (vec3 NORMAL_MAP "Set normal here if reading normal from a texture instead of NORMAL."),
        (float NORMAL_MAP_DEPTH "Depth from variable above. Defaults to 1.0."),
        (vec3 ALBEDO "Albedo (default white)."),
        (float ALPHA
"Alpha (0..1); if written to, the material will go to the transparent pipeline."),
        (float ALPHA_SCISSOR_THRESHOLD
"If written to, values below a certain amount of alpha are discarded."),
        (float ALPHA_HASH_SCALE ""),
        (float ALPHA_ANTIALIASING_EDGE ""),
        (vec2 ALPHA_TEXTURE_COORDINATE ""),
        (float METALLIC "Metallic (0..1)."),
        (float SPECULAR
"Specular. Defaults to 0.5, best not to modify unless you want to change IOR."),
        (float ROUGHNESS "Roughness (0..1)."),
        (float RIM "Rim (0..1). If used, Godot calculates rim lighting."),
        (float RIM_TINT
"Rim Tint, goes from 0 (white) to 1 (albedo). If used, Godot calculates rim lighting."),
        (float CLEARCOAT "Small added specular blob. If used, Godot calculates Clearcoat."),
        (float CLEARCOAT_GLOSS "Gloss of Clearcoat. If used, Godot calculates Clearcoat."),
        (float ANISOTROPY "For distorting the specular blob according to tangent space."),
        (vec2 ANISOTROPY_FLOW "Distortion direction, use with flowmaps."),
        (float SSS_STRENGTH
"Strength of Subsurface Scattering. If used, Subsurface Scattering will be applied to object."),
        (vec4 SSS_TRANSMITTANCE_COLOR ""),
        (float SSS_TRANSMITTANCE_DEPTH ""),
        (float SSS_TRANSMITTANCE_BOOST ""),
        (invec3 BACKLIGHT ""),
        (float AO "Strength of Ambient Occlusion. For use with pre-baked AO."),
        (float AO_LIGHT_AFFECT "How much AO affects lights (0..1; default 0)."),
        (vec3 EMISSION "Emission color (can go over 1,1,1 for HDR)."),
        (vec4 FOG "If written to, blends final pixel color with FOG.rgb based on FOG.a."),
        (vec4 RADIANCE
"If written to, blends environment map radiance with RADIANCE.rgb based on RADIANCE.a."),
        (vec4 IRRADIANCE
"If written to, blends environment map IRRADIANCE with IRRADIANCE.rgb based on IRRADIANCE.a."),
    )
}

pub fn spatial_light_vars() -> Vec<(String, ValueInfo)> {
    var_hashmap!(
        (const vec2 VIEWPORT_SIZE "Size of viewport (in pixels)."),
        (const vec4 FRAGCOORD
"Coordinate of pixel center in screen space. xy specifies position in window,
z specifies fragment depth if DEPTH is not used. Origin is lower-left."),
        (const mat4 MODEL_MATRIX "Model space to world space transform."),
        (const mat4 INV_VIEW_MATRIX "View space to world space transform."),
        (const mat4 VIEW_MATRIX "World space to view space transform."),
        (const mat4 PROJECTION_MATRIX "View space to clip space transform."),
        (const mat4 INV_PROJECTION_MATRIX "Clip space to view space transform."),
        (const vec3 NORMAL "Normal vector, in view space."),
        (const vec2 UV "UV that comes from vertex function."),
        (const vec2 UV2 "UV2 that comes from vertex function."),
        (const vec3 VIEW "View vector, in view space."),
        (const vec3 LIGHT "Light Vector, in view space."),
        (const vec3 LIGHT_COLOR
"Color of light multiplied by energy * PI. The PI multiplication is present
because physically-based lighting models include a division by PI."),
        (const float SPECULAR_AMOUNT
"2.0 * light_specular property for OmniLight3D and SpotLight3D. 1.0 for DirectionalLight3D."),
        (const bool LIGHT_IS_DIRECTIONAL "true if this pass is a DirectionalLight3D."),
        (const float ATTENUATION "Attenuation based on distance or shadow."),
        (const vec3 ALBEDO "Base albedo."),
        (const vec3 BACKLIGHT ""),
        (const float METALLIC "Metallic."),
        (const float ROUGHNESS "Roughness."),
        (const bool OUTPUT_IS_SRGB
"true when output is in sRGB color space (this is true in the Compatibility renderer,
false in Forward+ and Forward Mobile)."),
        (vec3 DIFFUSE_LIGHT "Diffuse light result."),
        (vec3 SPECULAR_LIGHT "Specular light result."),
        (float ALPHA
"Alpha (0..1); if written to, the material will go to the transparent pipeline."),

    )
}

pub fn canvas_item_vertex_vars() -> Vec<(String, ValueInfo)> {
    var_hashmap!(
        (const mat4 MODEL_MATRIX
"Local space to world space transform. World space is the coordinates you normally
use in the editor."),
        (const mat4 CANVAS_MATRIX
"World space to canvas space transform. In canvas space the origin is the upper-left
corner of the screen and coordinates ranging from (0, 0) to viewport size."),
        (const mat4 SCREEN_MATRIX
"Canvas space to clip space. In clip space coordinates ranging from (-1, -1) to (1, 1)."),
        (const int INSTANCE_ID "Instance ID for instancing."),
        (const vec4 INSTANCE_CUSTOM "Instance custom data."),
        (const bool AT_LIGHT_PASS "Always false."),
        (const vec2 TEXTURE_PIXEL_SIZE
"Normalized pixel size of default 2D texture. For a Sprite2D with a texture of size
64x32px, TEXTURE_PIXEL_SIZE = vec2(1/64, 1/32)"),
        (vec2 VERTEX "Vertex, in local space."),
        (const int VERTEX_ID "The index of the current vertex in the vertex buffer."),
        (vec2 UV "Normalized texture coordinates. Range from 0 to 1."),
        (vec4 COLOR "Color from vertex primitive."),
        (float POINT_SIZE "Point size for point drawing."),
    )
}

pub fn canvas_item_fragment_vars() -> Vec<(String, ValueInfo)> {
    var_hashmap!(
        (const vec4 FRAGCOORD
"Coordinate of pixel center. In screen space. xy specifies position in window.
Origin is lower-left."),
        (const vec2 SCREEN_PIXEL_SIZE "Size of individual pixels. Equal to inverse of resolution."),
        (const vec2 POINT_COORD "Coordinate for drawing points."),
        (sampler2D TEXTURE "Default 2D texture."),
        (const vec2 TEXTURE_PIXEL_SIZE
"Normalized pixel size of default 2D texture. For a Sprite2D with a texture of size
64x32px, TEXTURE_PIXEL_SIZE = vec2(1/64, 1/32)"),
        (const bool AT_LIGHT_PASS "Always false."),
        (sampler2D SPECULAR_SHININESS_TEXTURE "Specular shininess texture of this object."),
        (const vec4 SPECULAR_SHININESS "Specular shininess color, as sampled from the texture."),
        (const vec2 UV "UV from vertex function."),
        (const vec2 SCREEN_UV "Screen UV coordinate for current pixel."),
        (vec3 NORMAL "Normal read from NORMAL_TEXTURE. Writable."),
        (sampler2D NORMAL_TEXTURE "Default 2D normal texture."),
        (vec3 NORMAL_MAP
"Configures normal maps meant for 3D for use in 2D. If used, overrides NORMAL."),
        (float NORMAL_MAP_DEPTH "Normalmap depth for scaling."),
        (vec2 VERTEX "Pixel position in screen space."),
        (vec2 SHADOW_VERTEX "Same as VERTEX but can be written to alter shadows."),
        (vec3 LIGHT_VERTEX
"Same as VERTEX but can be written to alter lighting. Z component represents height."),
        (vec4 COLOR
"Color from vertex function multiplied by the TEXTURE color. Also output color value."),
    )
}

pub fn canvas_item_light_vars() -> Vec<(String, ValueInfo)> {
    var_hashmap!(
        (const vec4 FRAGCOORD
"Coordinate of pixel center. In screen space. xy specifies position in window.
Origin is lower-left."),
        (const vec3 NORMAL "Input Normal."),
        (const vec4 COLOR "Input Color. This is the output of the fragment function."),
        (const vec2 UV "UV from vertex function, equivalent to the UV in the fragment function."),
        (sampler2D TEXTURE "Current texture in use for CanvasItem."),
        (const vec2 TEXTURE_PIXEL_SIZE
"Normalized pixel size of TEXTURE. For a Sprite2D with a TEXTURE of size
64x32px, TEXTURE_PIXEL_SIZE = vec2(1/64, 1/32)"),
        (const vec2 SCREEN_UV "Screen UV coordinate for current pixel."),
        (const vec2 POINT_COORD "UV for Point Sprite."),
        (const vec4 LIGHT_COLOR "Color of Light multiplied by Light's texture."),
        (const float LIGHT_ENERGY "Energy multiplier of Light."),
        (const vec3 LIGHT_POSITION
"Position of Light in screen space. If using a DirectionalLight2D this is always vec3(0,0,0)."),
        (const vec3 LIGHT_DIRECTION "Direction of Light in screen space."),
        (const bool LIGHT_IS_DIRECTIONAL "true if this pass is a DirectionalLight2D."),
        (const vec3 LIGHT_VERTEX
"Pixel position, in screen space as modified in the fragment function."),
        (vec4 LIGHT "Output color for this Light."),
        (const vec4 SPECULAR_SHININESS "Specular shininess, as set in the object's texture."),
        (vec4 SHADOW_MODULATE "Multiply shadows cast at this point by this color."),
    )
}

pub fn particle_start_process() -> Vec<(String, ValueInfo)> {
    var_hashmap!(
        (const float LIFETIME "Particle lifetime."),
        (const float DELTA "Delta process time."),
        (const uint NUMBER "Unique number since emission start."),
        (const uint INDEX "Particle index (from total particles)."),
        (const mat4 EMISSION_TRANSFORM "Emitter transform (used for non-local systems)."),
        (const uint RANDOM_SEED "Random seed used as base for random."),
        (bool ACTIVE "true when the particle is active, can be set false."),
        (vec4 COLOR "Particle color, can be written to and accessed in mesh's vertex function."),
        (vec3 VELOCITY "Particle velocity, can be modified."),
        (mat4 TRANSFORM "Particle transform."),
        (vec4 CUSTOM "Custom particle data. Accessible from shader of mesh as INSTANCE_CUSTOM."),
        (float MASS "Particle mass, intended to be used with attractors. Equals 1.0 by default."),
        (vec4 USERDATAX
"Vector that enables the integration of supplementary user-defined data into the
particle process shader. USERDATAX are six built-ins identified by number, X can
be numbers between 1 and 6."),
        (uint FLAG_EMIT_POSITION
"A flag for using on the last argument of emit_subparticle function to assign a
position to a new particle's transform."),
        (uint FLAG_EMIT_ROT_SCALE
"A flag for using on the last argument of emit_subparticle function to assign the
rotation and scale to a new particle's transform."),
        (uint FLAG_EMIT_VELOCITY
"A flag for using on the last argument of emit_subparticle function to assign a
velocity to a new particle."),
        (uint FLAG_EMIT_COLOR
"A flag for using on the last argument of emit_subparticle function to assign a
color to a new particle."),
        (uint FLAG_EMIT_CUSTOM
"A flag for using on the last argument of emit_subparticle function to assign a
custom data vector to a new particle."),
        (vec3 EMITTER_VELOCITY "Velocity of the Particles node."),
        (float INTERPOLATE_TO_END "Value of interp_to_end property of Particles node."),
        (uint AMOUNT_RATIO "Value of amount_ratio property of Particles node."),
    )
}

pub fn particle_start() -> Vec<(String, ValueInfo)> {
    var_hashmap!(
        (const bool RESTART_POSITION
"true if particle is restarted, or emitted without a custom position
(i.e. this particle was created by emit_subparticle() without the FLAG_EMIT_POSITION flag)."),
        (const bool RESTART_ROT_SCALE
"true if particle is restarted, or emitted without a custom rotation or scale
(i.e. this particle was created by emit_subparticle() without the FLAG_EMIT_ROT_SCALE flag)."),
        (const bool RESTART_VELOCITY
"true if particle is restarted, or emitted without a custom velocity
(i.e. this particle was created by emit_subparticle() without the FLAG_EMIT_VELOCITY flag)."),
        (const bool RESTART_COLOR
"true if particle is restarted, or emitted without a custom color
(i.e. this particle was created by emit_subparticle() without the FLAG_EMIT_COLOR flag)."),
        (const bool RESTART_CUSTOM
"true if particle is restarted, or emitted without a custom property
(i.e. this particle was created by emit_subparticle() without the FLAG_EMIT_CUSTOM flag)."),
    )
}

pub fn particle_process() -> Vec<(String, ValueInfo)> {
    var_hashmap!(
        (const bool RESTART "true if the current process frame is first for the particle."),
        (const bool COLLIDED "true when the particle has collided with a particle collider."),
        (const vec3 COLLISION_NORMAL
"A normal of the last collision. If there is no collision detected it is equal to vec3(0.0)."),
        (const float COLLISION_DEPTH
"A length of normal of the last collision. If there is no collision detected it is equal to 0.0."),
        (const vec3 ATTRACTOR_FORCE
"A combined force of the attractors at the moment on that particle."),
    )
}

pub fn sky_builtins() -> Vec<(String, ValueInfo)> {
    var_hashmap!(
        (const vec3 POSITION "Camera position in world space."),
        (const samplerCube RADIANCE
"Radiance cubemap. Can only be read from during background pass.
Check !AT_CUBEMAP_PASS before using."),
        (const bool AT_HALF_RES_PASS "Currently rendering to half resolution pass."),
        (const bool AT_QUARTER_RES_PASS "Currently rendering to quarter resolution pass."),
        (const bool AT_CUBEMAP_PASS "Currently rendering to radiance cubemap."),
        (const bool LIGHTX_ENABLED
"LightX is visible and in the scene. If false, other light properties may be garbage."),
        (const float LIGHTX_ENERGY "Energy multiplier for LIGHTX."),
        (const vec3 LIGHTX_DIRECTION "Direction that LIGHTX is facing."),
        (const vec3 LIGHTX_COLOR "Color of LIGHTX."),
        (const float LIGHTX_SIZE
"Angular diameter of LIGHTX in the sky. Expressed in degrees. For reference,
the sun from earth is about 0.5 degrees."),
    )
}

pub fn sky_stuff() -> Vec<(String, ValueInfo)> {
    var_hashmap!(
        (const vec3 EYEDIR
"Normalized direction of current pixel. Use this as your basic direction for procedural effects."),
        (const vec2 SCREEN_UV
"Screen UV coordinate for current pixel. Used to map a texture to the full screen."),
        (const vec2 SKY_COORDS "Sphere UV. Used to map a panorama texture to the sky."),
        (const vec4 HALF_RES_COLOR
"Color value of corresponding pixel from half resolution pass. Uses linear filter."),
        (const vec4 QUARTER_RES_COLOR
"Color value of corresponding pixel from quarter resolution pass. Uses linear filter."),
        (vec3 COLOR "Output color."),
        (float ALPHA "Output alpha value, can only be used in subpasses."),
        (vec4 FOG ""),
    )
}

pub fn fog_stuff() -> Vec<(String, ValueInfo)> {
    var_hashmap!(
        (const vec3 WORLD_POSITION "Position of current froxel cell in world space."),
        (const vec3 OBJECT_POSITION
"Position of the center of the current FogVolume in world space."),
        (const vec3 UVW "3-dimensional uv, used to map a 3D texture to the current FogVolume."),
        (const vec3 SIZE "Size of the current FogVolume when its shape has a size."),
        (const vec3 SDF
"Signed distance field to the surface of the FogVolume. Negative if inside volume,
positive otherwise."),
        (vec3 ALBEDO
"Output base color value, interacts with light to produce final color. Only written
to fog volume if used."),
        (float DENSITY
"Output density value. Can be negative to allow subtracting one volume from another.
Density must be used for fog shader to write anything at all."),
        (vec3 EMISSION
"Output emission color value, added to color during light pass to produce final color.
Only written to fog volume if used."),
    )
}



