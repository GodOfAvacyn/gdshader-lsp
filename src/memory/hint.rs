use std::{collections::HashMap};

use super::TypeInfo;

#[derive(Clone, Debug)]
pub struct HintInfo {
    pub type_info: Vec<TypeInfo>,
    pub num_arguments: Vec<usize>,
    pub description: String,
}

macro_rules! build_hint {
    (($($base:ident)*) $hint:ident $description:literal ) => {
        build_hint!(($($base)*) $hint (0) $description)
    };
    (($($base:ident)*) $hint:ident ($($number:literal)*) $description:literal ) => {
        (stringify!($hint).to_string(), HintInfo {
            type_info: vec![$(TypeInfo::from_str(stringify!($base)),)*],
            num_arguments: vec![$($number,)*],
            description: $description.to_string()
        })
    };
}

pub fn make_builtin_hints() -> HashMap<String, HintInfo> {
    HashMap::from([
        build_hint!(
            (vec3 vec4 sampler2D) source_color
            "Used as color."
        ),
        build_hint!(
            (float int) hint_range (2 3)
            "Restricted to values in a range (with min/max/step)."
        ),
        build_hint!(
            (sampler2D) hint_normal
            "Used as normalmap."
        ),
        build_hint!(
            (sampler2D) hint_default_white
            "As value or albedo color, default to opaque white."
        ),
        build_hint!(
            (sampler2D) hint_default_black
            "As value or albedo color, default to opaque black."
        ),
        build_hint!(
            (sampler2D) hint_default_transparent
            "As value or albedo color, default to transparent black."
        ),
        build_hint!(
            (sampler2D) hint_anisotropy
            "As flowmap, default to right."
        ),
        build_hint!(
            (sampler2D) hint_roughness_r
"Used for roughness limiter on import (attempts reducing specular aliasing).
_normal is a normal map that guides the roughness limiter, with roughness
increasing in areas that have high-frequency detail."
        ),
        build_hint!(
            (sampler2D) hint_roughness_g
"Used for roughness limiter on import (attempts reducing specular aliasing).
_normal is a normal map that guides the roughness limiter, with roughness
increasing in areas that have high-frequency detail."
        ),
        build_hint!(
            (sampler2D) hint_roughness_b
"Used for roughness limiter on import (attempts reducing specular aliasing).
_normal is a normal map that guides the roughness limiter, with roughness
increasing in areas that have high-frequency detail."
        ),
        build_hint!(
            (sampler2D) hint_roughness_a
"Used for roughness limiter on import (attempts reducing specular aliasing).
_normal is a normal map that guides the roughness limiter, with roughness
increasing in areas that have high-frequency detail."
        ),
        build_hint!(
            (sampler2D) hint_roughness_normal
"Used for roughness limiter on import (attempts reducing specular aliasing).
_normal is a normal map that guides the roughness limiter, with roughness
increasing in areas that have high-frequency detail."
        ),
        build_hint!(
            (sampler2D) hint_roughness_gray
"Used for roughness limiter on import (attempts reducing specular aliasing).
_normal is a normal map that guides the roughness limiter, with roughness
increasing in areas that have high-frequency detail."
        ),
        build_hint!(
            (sampler2D) filter_nearest
            "Enabled specified texture filtering."
        ),
        build_hint!(
            (sampler2D) filter_linear
            "Enabled specified texture filtering."
        ),
        build_hint!(
            (sampler2D) filter_nearest_mipmap
            "Enabled specified texture filtering."
        ),
        build_hint!(
            (sampler2D) filter_linear_mipmap
            "Enabled specified texture filtering."
        ),
        build_hint!(
            (sampler2D) filter_nearest_mipmap_anisotropic
            "Enabled specified texture filtering."
        ),
        build_hint!(
            (sampler2D) filter_linear_mipmap_anisotropic
            "Enabled specified texture filtering."
        ),
        build_hint!(
            (sampler2D) repeat_enable
            "Enabled texture repeating."
        ),
        build_hint!(
            (sampler2D) repeat_disable
            "Enabled texture repeating."
        ),
        build_hint!(
            (sampler2D) hint_screen_texture
            "Texture is the screen texture."
        ),
        build_hint!(
            (sampler2D) hint_depth_texture
            "Texture is the depth texture."
        ),
        build_hint!(
            (sampler2D) hint_normal_roughness_texture
            "Texture is the normal roughness texture (only supported in Forward+)"
        )
    ])
}



