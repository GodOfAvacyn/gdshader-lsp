use std::collections::HashMap;
use lsp_types::Range;
use crate::lexer::{Token, TokenKind};

use super::TypeInfo;


#[derive(Clone, Debug, PartialEq)]
pub enum FunctionParamQualifier {
    In,
    Out,
    InOut,
}
impl FunctionParamQualifier {
    pub fn kind(&self) -> TokenKind {
        use FunctionParamQualifier::*;
        match self {
            In => TokenKind::In, 
            Out => TokenKind::Out, 
            InOut => TokenKind::InOut, 
        }
    }
    pub fn from(token: Token) -> Self {
        use FunctionParamQualifier::*;
        match token.kind {
            TokenKind::In => In, 
            TokenKind::Out => Out, 
            TokenKind::InOut => InOut, 
            _ => unreachable!()
        }
    }
}

#[derive(Clone, Debug)]
pub struct FunctionParam {
    pub name: String,
    pub ty: TypeInfo,
    pub qualifier: Option<FunctionParamQualifier>
}

#[derive(Debug)]
pub struct FunctionSignature {
    pub return_type: TypeInfo,
    pub params: Vec<FunctionParam>,
}

#[derive(Debug)]
pub struct FunctionInfo {
    pub signatures: Vec<FunctionSignature>,
    pub range: Option<Range>,
    pub description: Option<String>,
    pub is_const: bool
}

macro_rules! builtin_function {
    ($name:ident [$( ($( $($arg_id:ident)* ,)*) -> $r_type:ident,)*] $desc:literal) => {
        builtin_function!(@main false $name [$( ($( $($arg_id)*,) *) -> $r_type,)*] $desc)
    };
    ($name:ident [$( ($( $($arg_id:ident)* ,)*) -> $r_type:ident,)*] $desc:literal const) => {
        builtin_function!(@main true $name [$( ($( $($arg_id)*,)*) -> $r_type,)*] $desc)
    };
    (@main $const:literal $name:ident [$( ($( $($arg_id:ident)*, )*) -> $r_type:ident,)*] $desc:literal) => {
        (stringify!($name).to_string(), FunctionInfo {
            signatures: vec![
               $(
                FunctionSignature {
                    return_type: TypeInfo::from_str(stringify!($r_type)),
                    params: vec![ $( builtin_function!(@param $($arg_id)*), )* ]
                },
                )*
            ],
            range: None,
            description: Some($desc.to_string()),
            is_const: $const 
        }) 
    };
    (@param $param_type:ident $param_name:ident) => {
        FunctionParam {
            name: stringify!($param_name).to_string(),
            ty: TypeInfo::from_str(stringify!($param_type)),
            qualifier: None
        }
    };
    (@param $qualifier:ident $param_type:ident $param_name:ident) => {
        FunctionParam {
            name: stringify!($param_name).to_string(),
            ty: TypeInfo::from_str(stringify!($param_type)),
            qualifier: Some(FunctionParamQualifier::$qualifier)
        }
    };
}

pub fn make_builtin_functions() -> HashMap<String, FunctionInfo> {
    HashMap::from([
        builtin_function!(
            float [(number x,) -> float, (boolean x,) -> float,]
            "Cast to a float." const
        ),
        builtin_function!(
            int [(number x,) -> int, (boolean x,) -> int,]
            "Cast to a signed integer." const
        ),
        builtin_function!(
            uint [(number x,) -> uint, (boolean x,) -> uint,]
            "Cast to an unsigned integer." const
        ),
        builtin_function!(
            bool [(number x,) -> bool, (boolean x,) -> uint,]
            "Cast to an boolean." const
        ),
        builtin_function!(
            vec2 [(number x,) -> vec2, (number x, number y,) -> vec2,]
            "Cast to a vec2." const
        ),
        builtin_function!(
            vec3 [
                (number x,) -> vec3,
                (number x, number y, number z,) -> vec3,
                (vec2 x, number y,) -> vec3,
            ]
            "Cast to a vec3." const
        ),
        builtin_function!(
            vec4 [
                (number x,) -> vec4,
                (number x, number y, number z, number w,) -> vec4,
                (vec3 x, number y,) -> vec4,
            ]
            "Cast to a vec4." const
        ),
        builtin_function!(
            ivec2 [(int x,) -> ivec2, (int x, int y,) -> ivec2,]
            "Cast to an ivec2." const
        ),
        builtin_function!(
            ivec3 [
                (int x,) -> ivec3,
                (int x, int y, int z,) -> ivec3,
                (ivec2 x, int y,) -> ivec3,
            ]
            "Cast to a vec3." const
        ),
        builtin_function!(
            ivec4 [
                (int x,) -> ivec4,
                (int x, int y, int z, int w,) -> ivec4,
                (ivec3 x, int y,) -> ivec4,
            ]
            "Cast to a vec4." const
        ),
        builtin_function!(
            uvec2 [(uint x,) -> vec2, (uint x, uint y,) -> uvec2,]
            "Cast to a uvec2." const
        ),
        builtin_function!(
            uvec3 [
                (uint x,) -> uvec3,
                (uint x, uint y, uint z,) -> uvec3,
                (uvec2 x, uint y,) -> uvec3,
            ]
            "Cast to a vec3." const
        ),
        builtin_function!(
            uvec4 [
                (uint x,) -> uvec4,
                (uint x, uint y, uint z, uint w,) -> uvec4,
                (uvec3 x, uint y,) -> uvec4,
            ]
            "Cast to a vec4." const
        ),
        builtin_function!(
            bvec2 [(bool x,) -> bvec2, (bool x, bool y,) -> bvec2,]
            "Cast to a bvec2." const
        ),
        builtin_function!(
            bvec3 [
                (bool x,) -> bvec3,
                (bool x, bool y, bool z,) -> bvec3,
                (bvec2 x, bool y,) -> bvec3,
            ]
            "Cast to a vec3." const
        ),
        builtin_function!(
            bvec4 [
                (bool x,) -> bvec4,
                (bool x, bool y, bool z, bool w,) -> bvec4,
                (bvec3 x, bool y,) -> bvec4,
            ]
            "Cast to a vec4." const
        ),
        builtin_function!(
            mat2 [(vec2 x, vec2 y,) -> mat2,]
            "Cast to a mat2." const
        ),
        builtin_function!(
            mat3 [(vec3 x, vec3 y, vec3 z,) -> mat3,]
            "Cast to a mat3." const
        ),
        builtin_function!(
            mat4 [(vec4 x, vec4 y, vec4 z, vec4 w,) -> mat4,]
            "Cast to a mat4." const
        ),
        builtin_function!(
            radians [(vec_type degrees,) -> vec_type,]
            "Convert degrees to radians."
        ),
        builtin_function!(
            degrees [(vec_type radians,) -> vec_type,]
            "Convert radians to degrees."
        ),
        builtin_function!(
            sin [(vec_type x,) -> vec_type,]
            "Sine."
        ),
        builtin_function!(
            cos [(vec_type x,) -> vec_type,]
            "Cosine."
        ),
        builtin_function!(
            tan [(vec_type x,) -> vec_type,]
            "Tangent."
        ),
        builtin_function!(
            asin [(vec_type x,) -> vec_type,]
            "Arcsine."
        ),
        builtin_function!(
            acos [(vec_type x,) -> vec_type,]
            "Arccosine."
        ),
        builtin_function!(
            atan [(vec_type y_over_x,) -> vec_type, (vec_type y, vec_type x,) -> vec_type,]
            "Arctangent."
        ),
        builtin_function!(
            sinh [(vec_type x,) -> vec_type,]
            "Hyperbolic sine."
        ),
        builtin_function!(
            cosh [(vec_type x,) -> vec_type,]
            "Hyperbolic cosine."
        ),
        builtin_function!(
            tanh [(vec_type x,) -> vec_type,]
            "Hyperbolic tangent."
        ),
        builtin_function!(
            asinh [(vec_type x,) -> vec_type,]
            "Inverse hyperbolic sine."
        ),
        builtin_function!(
            acosh [(vec_type x,) -> vec_type,]
            "Inverse hyperbolic cosine."
        ),
        builtin_function!(
            atanh [(vec_type x,) -> vec_type,]
            "Inverse hyperbolic tangent."
        ),
        builtin_function!(
            pow [(vec_type x, vec_type y,) -> vec_type,]
            "Power (undefined if x < 0 or if x == 0 and y <= 0)."
        ),
        builtin_function!(
            exp [(vec_type x,) -> vec_type,]
            "Base-e exponential."
        ),
        builtin_function!(
            exp2 [(vec_type x,) -> vec_type,]
            "Base-2 exponential."
        ),
        builtin_function!(
            log [(vec_type x,) -> vec_type,]
            "Natural logarithm."
        ),
        builtin_function!(
            log2 [(vec_type x,) -> vec_type,]
            "Base-2 logarithm."
        ),
        builtin_function!(
            sqrt [(vec_type x,) -> vec_type,]
            "Square root."
        ),
        builtin_function!(
            inversesqrt [(vec_type x,) -> vec_type,]
            "Inverse square root."
        ),
        builtin_function!(
            abs [(vec_type x,) -> vec_type, (ivec_type x,) -> ivec_type,]
            "Absolute value (returns positive value if negative)."
        ),
        builtin_function!(
            sign [(vec_type x,) -> vec_type, (ivec_type x,) -> ivec_type,]
            "Sign (returns 1.0 if positive, -1.0 if negative, 0.0 if zero)."
        ),
        builtin_function!(
            floor [(vec_type x,) -> vec_type,]
            "Round to the integer below."
        ),
        builtin_function!(
            round [(vec_type x,) -> vec_type,]
            "Round to the nearest integer."
        ),
        builtin_function!(
            roundEven [(vec_type x,) -> vec_type,]
            "Round to the nearest even integer."
        ),
        builtin_function!(
            trunc [(vec_type x,) -> vec_type,]
            "Truncation."
        ),
        builtin_function!(
            ceil [(vec_type x,) -> vec_type,]
            "Round to the integer above."
        ),
        builtin_function!(
            fract [(vec_type x,) -> vec_type,]
            "Fractional (returns x - floor(x))."
        ),
        builtin_function!(
            mod [(vec_type x, vec_type y,) -> vec_type, (vec_type x, float y,) -> vec_type,]
            "Modulo (division remainder)"
        ),
        builtin_function!(
            modf [(vec_type x, Out float y,) -> vec_type,]
            "Fractional of x, with i as integer part."
        ),
        builtin_function!(
            min [(vec_type a, vec_type b,) -> vec_type,]
            "Lowest value between a and b."
        ),
        builtin_function!(
            max [(vec_type a, vec_type b,) -> vec_type,]
            "Highest value between a and b."
        ),
        builtin_function!(
            clamp [(vec_type x, vec_type min, vec_type max,) -> vec_type,]
            "Clamp x between min and max (inclusive)."
        ),
        builtin_function!(
            mix [
                (float a, float b, float c,) -> float,
                (vec_type a, vec_type b, float c,) -> vec_type,
                (vec_type a, vec_type b, bvec_type c,) -> vec_type,
            ]
            "Linear interpolate between a and b by c."
        ),
        builtin_function!(
            fma [(vec_type a, vec_type b, vec_type c,) -> vec_type,]
            "Performs a fused multiply-add operation: (a * b + c) (faster than doing
            it manually)."
        ),
        // STEP
        // SMOOTH_STEP
        builtin_function!(
            isnan [(vec_type x,) -> bvec_type,]
            "Returns true if scalar or vector component is NaN."
        ),
        builtin_function!(
            isinf [(vec_type x,) -> bvec_type,]
            "Returns true if scalar or vector component is INF."
        ),
        builtin_function!(
            floatBitsToInt [(vec_type x,) -> ivec_type,]
            "Float->Int bit copying, no conversion."
        ),
        builtin_function!(
            floatBitsToUint [(vec_type x,) -> uvec_type,]
            "Float->UInt bit copying, no conversion."
        ),
        builtin_function!(
            intBitsToFloat [(ivec_type x,) -> vec_type,]
            "Int->Float bit copying, no conversion."
        ),
        builtin_function!(
            uintBitsToFloat [(uvec_type x,) -> vec_type,]
            "UInt->Float bit copying, no conversion."
        ),
        builtin_function!(
            length [(vec_type x,) -> float,]
            "Vector length."
        ),
        builtin_function!(
            distance [(vec_type a, vec_type b,) -> float,]
            "Distance between vectors i.e length(a - b)."
        ),
        builtin_function!(
            dot [(vec_type a, vec_type b,) -> float,]
            "Dot product."
        ),
        builtin_function!(
            cross [(vec3 a, vec3 b,) -> vec3,]
            "Cross product."
        ),
        builtin_function!(
            normalize [(vec_type x,) -> vec_type,]
            "Normalize to unit length."
        ),
        builtin_function!(
            reflect [(vec3 I, vec3 N,) -> vec3,]
            "Reflect."
        ),
        builtin_function!(
            refract [(vec3 I, vec3 N, float eta,) -> vec3,]
            "Refract."
        ),
        builtin_function!(
            faceforward [(vec_type N, vec_type I, vec_type Nref,) -> vec_type,]
            "If dot(Nref, I) < 0, return N, otherwise -N."
        ),
        builtin_function!(
            matrixCompMult [(mat_type x, mat_type y,) -> mat_type,]
            "Matrix component multiplication."
        ),
        builtin_function!(
            outerProduct [(vec_type column, vec_type row,) -> mat_type,]
            "Matrix outer product."
        ),
        builtin_function!(
            transpose [(mat_type m,) -> mat_type,]
            "Transpose matrix."
        ),
        builtin_function!(
            determinant [(mat_type m,) -> float,]
            "Matrix determinant."
        ),
        builtin_function!(
            inverse [(mat_type m,) -> mat_type,]
            "Inverse matrix."
        ),
        builtin_function!(
            lessThan [(vec_type x, vec_type y,) -> bvec_type,]
            "Bool vector comparison on < int/uint/float vectors."
        ),
        builtin_function!(
            greaterThan [(vec_type x, vec_type y,) -> bvec_type,]
            "Bool vector comparison on > int/uint/float vectors."
        ),
        builtin_function!(
            lessThanEqual [(vec_type x, vec_type y,) -> bvec_type,]
            "Bool vector comparison on <= int/uint/float vectors."
        ),
        builtin_function!(
            greaterThanEqual [(vec_type x, vec_type y,) -> bvec_type,]
            "Bool vector comparison on >= int/uint/float vectors."
        ),
        builtin_function!(
            equal [(vec_type x, vec_type y,) -> bvec_type,]
            "Bool vector comparison on == int/uint/float vectors."
        ),
        builtin_function!(
            notEqual [(vec_type x, vec_type y,) -> bvec_type,]
            "Bool vector comparison on != int/uint/float vectors."
        ),
        builtin_function!(
            any [(bvec_type x,) -> bool,]
            "true if any component is true, false otherwise."
        ),
        builtin_function!(
            all [(bvec_type x,) -> bool,]
            "true if all components are true, false otherwise."
        ),
        builtin_function!(
            not [(bvec_type x,) -> bvec_type,]
            "Invert boolean vector."
        ),
        builtin_function!(
            textureSize [
                (gsampler2D s, int lod,) -> ivec2,
                (gsampler2DArray s, int lod,) -> ivec2,
                (gsampler3D s, int lod,) -> ivec3,
                (samplerCube s, int lod,) -> ivec2,
                (samplerCubeArray s, int lod,) -> ivec2,
            ]
"Get the size of a texture. The LOD defines which mipmap level is used.
An LOD value of 0 will use the full resolution texture."
        ),
        builtin_function!(
            textureQueryLod [
                (gsampler2D s, vec2 p,) -> vec2,
                (gsampler2DArray s, vec2 p,) -> vec3,
                (gsampler3D s, vec3 p,) -> vec2,
                (samplerCube s, vec3 p,) -> vec2,
            ]
"Compute the level-of-detail that would be used to sample from a texture.
The x component of the resulted value is the mipmap array that would be accessed.
The y component is computed level-of-detail relative to the base level
(regardless of the mipmap levels of the texture)."
        ),
        builtin_function!(
            textureQueryLevels [
                (gsampler2D s,) -> int,
                (gsampler2DArray s,) -> int,
                (gsampler3D s,) -> int,
                (samplerCube s,) -> int,
            ]
"Get the number of accessible mipmap levels of a texture.
If the texture is unassigned to a sampler, 1 is returned
(Godot always internally assigns a texture even to an empty sampler)."
        ),
        builtin_function!(
            texture [
                (gsampler2D s, vec2 p,) -> gvec4_type,
                (gsampler2DArray s, vec3 p,) -> gvec4_type,
                (gsampler3D s, vec3 p,) -> gvec4_type,
                (samplerCube s, vec3 p,) -> vec4_type,
                (samplerCubeArray s, vec4 p,) -> vec4_type,
            ]
            "Perform a texture read."
        ),
        builtin_function!(
            textureProj [
                (gsampler2D s, vec3 p,) -> gvec4_type,
                (gsampler2D s, vec4 p,) -> gvec4_type,
                (gsampler3D s, vec4 p,) -> gvec4_type,
            ]
            "Perform a texture read with projection."
        ),
        builtin_function!(
            textureLod [
                (gsampler2D s, vec2 p, float lod,) -> gvec4_type,
                (gsampler2DArray s, vec3 p, float lod,) -> gvec4_type,
                (gsampler3D s, vec3 p, float lod,) -> gvec4_type,
                (samplerCube s, vec3 p, float lod,) -> vec4_type,
                (samplerCubeArray s, vec4 p, float lod,) -> vec4_type,
            ]
            "Perform a texture read at custom mipmap."
        ),
        builtin_function!(
            textureProjLod [
                (gsampler2D s, vec3 p, float lod,) -> gvec4_type,
                (gsampler2D s, vec4 p, float lod,) -> gvec4_type,
                (gsampler3D s, vec4 p, float lod,) -> gvec4_type,
            ]
            "Performs a texture read with projection/LOD."
        ),
        builtin_function!(
            textureGrad [
                (gsampler2D s, vec2 p, vec2 dPdx, vec2 dPdy,) -> gvec4_type,
                (gsampler2DArray s, vec3 p, vec2 dPdx, vec2 dPdy,) -> gvec4_type,
                (gsampler3D s, vec3 p, vec2 dPdx, vec2 dPdy,) -> gvec4_type,
                (samplerCube s, vec3 p, vec3 dPdx, vec3 dPdy,) -> vec4_type,
                (samplerCubeArray s, vec3 p, vec3 dPdx, vec3 dPdy,) -> vec4_type,
            ]
            "Performs a texture read with explicit gradients."
        ),
        builtin_function!(
            textureProjGrad [
                (gsampler2D s, vec3 p, vec2 dPdx, vec2 dPdy,) -> gvec4_type,
                (gsampler2D s, vec4 p, vec2 dPdx, vec2 dPdy,) -> gvec4_type,
                (gsampler3D s, vec4 p, vec3 dPdx, vec3 dPdy,) -> gvec4_type,
            ]
            "Performs a texture read with projection/LOD and with explicit gradients."
        ),
        builtin_function!(
            texelFetch [
                (gsampler2D s, ivec2 p, int lod,) -> gvec4_type,
                (gsampler2DArray s, ivec3 p, int lod,) -> gvec4_type,
                (gsampler3D s, ivec3 p, int lod,) -> gvec4_type,
            ]
            "Fetches a single texel using integer coordinates."
        ),
        builtin_function!(
            textureGather [
                (gsampler2D s, vec2 p,) -> gvec4_type,
                (gsampler2DArray s, vec3 p,) -> gvec4_type,
                (samplerCube s, vec3 p,) -> vec4_type,
            ]
            "Gathers four texels from a texture."
        ),
        builtin_function!(
            dFdx [(vec_type p,) -> vec_type,]
"Derivative in x using local differencing. Internally, can use either
dFdxCoarse or dFdxFine, but the decision for which to use is made by
the GPU driver."
        ),
        builtin_function!(
            dFdxCoarse [(vec_type p,) -> vec_type,]
"Calculates derivative with respect to x window coordinate using local
differencing based on the value of p for the current fragment neighbour(s),
and will possibly, but not necessarily, include the value for the current
fragment. This function is not available on gl_compatibility profile."
        ),
        builtin_function!(
            dFdxFine [(vec_type p,) -> vec_type,]
"Calculates derivative with respect to x window coordinate using local
differencing based on the value of p for the current fragment and its
immediate neighbour(s). This function is not available on gl_compatibility
profile."
        ),
        builtin_function!(
            dFdy [(vec_type p,) -> vec_type,]
"Derivative in y using local differencing. Internally, can use either
dFdyCoarse or dFdyFine, but the decision for which to use is made by
the GPU driver."
        ),
        builtin_function!(
            dFdyCoarse [(vec_type p,) -> vec_type,]
"Calculates derivative with respect to y window coordinate using local
differencing based on the value of p for the current fragment neighbour(s),
and will possibly, but not necessarily, include the value for the current
fragment. This function is not available on gl_compatibility profile."
        ),
        builtin_function!(
            dFdyFine [(vec_type p,) -> vec_type,]
"Calculates derivative with respect to y window coordinate using local
differencing based on the value of p for the current fragment and its
immediate neighbour(s). This function is not available on gl_compatibility
profile."
        ),
        builtin_function!(
            fwidth [(vec_type p,) -> vec_type,]
"Sum of absolute derivative in x and y. This is the equivalent of using
abs(dFdx(p)) + abs(dFdy(p))."
        ),
        builtin_function!(
            fwidthCoarse [(vec_type p,) -> vec_type,]
"Sum of absolute derivative in x and y. This is the equivalent of using
abs(dFdxCoarse(p)) + abs(dFdyCoarse(p)). This function is not available
on gl_compatibility profile."
        ),
        builtin_function!(
            fwidthFine [(vec_type p,) -> vec_type,]
"Sum of absolute derivative in x and y. This is the equivalent of using
abs(dFdxFine(p)) + abs(dFdyFine(p)). This function is not available on
gl_compatibility profile."
        ),
        builtin_function!(
            packHalf2x16 [(vec2 v,) -> uint,]
"Convert two 32-bit floating-point numbers into 16-bit and pack them
into a 32-bit unsigned integer and vice-versa."
        ),
        builtin_function!(
            unpackHalf2x16 [(uint v,) -> vec2,]
"Convert two 32-bit floating-point numbers into 16-bit and pack them
into a 32-bit unsigned integer and vice-versa."
        ),
        builtin_function!(
            packUnorm2x16 [(vec2 v,) -> uint,]
"Convert two 32-bit floating-point numbers (clamped within 0..1 range)
into 16-bit and pack them into a 32-bit unsigned integer and vice-versa."
        ),
        builtin_function!(
            unpackUnorm2x16 [(uint v,) -> vec2,]
"Convert two 32-bit floating-point numbers (clamped within 0..1 range)
into 16-bit and pack them into a 32-bit unsigned integer and vice-versa."
        ),
        builtin_function!(
            packSnorm2x16 [(vec2 v,) -> uint,]
"Convert two 32-bit floating-point numbers (clamped within -1..1 range) 
into 16-bit and pack them into a 32-bit unsigned integer and vice-versa."
        ),
        builtin_function!(
            unpackSnorm2x16 [(uint v,) -> vec2,]
"Convert two 32-bit floating-point numbers (clamped within -1..1 range) 
into 16-bit and pack them into a 32-bit unsigned integer and vice-versa."
        ),
        builtin_function!(
            packUnorm4x8 [(vec4 v,) -> uint,]
"Convert four 32-bit floating-point numbers (clamped within 0..1 range) 
into 8-bit and pack them into a 32-bit unsigned integer and vice-versa."
        ),
        builtin_function!(
            unpackUnorm4x8 [(uint v,) -> vec4,]
"Convert four 32-bit floating-point numbers (clamped within 0..1 range) 
into 8-bit and pack them into a 32-bit unsigned integer and vice-versa."
        ),
        builtin_function!(
            packSnorm4x8 [(vec4 v,) -> uint,]
"Convert four 32-bit floating-point numbers (clamped within -1..1 range) 
into 8-bit and pack them into a 32-bit unsigned integer and vice-versa."
        ),
        builtin_function!(
            unpackSnorm4x8 [(uint v,) -> vec4,]
"Convert four 32-bit floating-point numbers (clamped within -1..1 range) 
into 8-bit and pack them into a 32-bit unsigned integer and vice-versa."
        ),
        builtin_function!(
            bitfieldExtract [
                (uvec_type value, int offset, int bits,) -> uvec_type,
                (ivec_type value, int offset, int bits,) -> ivec_type,
            ]
            "Extracts a range of bits from an integer."
        ),
        builtin_function!(
            bitfieldInsert [
                (ivec_type base, ivec_type insert, int offset, int bits,) -> ivec_type,
                (uvec_type base, uvec_type insert, int offset, int bits,) -> uvec_type,
            ]
            "Insert a range of bits into an integer."
        ),
        builtin_function!(
            bitfieldReverse [(ivec_type value,) -> ivec_type, (uvec_type value,) -> uvec_type,]
            "Reverse the order of bits in an integer."
        ),
        builtin_function!(
            bitCount [(ivec_type value,) -> ivec_type, (uvec_type value,) -> uvec_type,]
            "Counts the number of 1 bits in an integer."
        ),
        builtin_function!(
            findLSB [(ivec_type value,) -> ivec_type, (uvec_type value,) -> uvec_type,]
            "Find the index of the least significant bit set to 1 in an integer."
        ),
        builtin_function!(
            findMSB [(ivec_type value,) -> ivec_type, (uvec_type value,) -> uvec_type,]
            "Find the index of the most significant bit set to 1 in an integer."
        ),
        builtin_function!(
            imulExtended [
                (ivec_type x, ivec_type y, Out ivec_type msb, Out ivec_type lsb,) -> void,
            ]
"Multiplies two 32-bit numbers and produce a 64-bit result.
x - the first number. y - the second number.
msb - will contain the most significant bits.
lsb - will contain the least significant bits."
        ),
        builtin_function!(
            umulExtended [
                (uvec_type x, uvec_type y, Out uvec_type msb, Out uvec_type lsb,) -> void,
            ]
"Multiplies two 32-bit numbers and produce a 64-bit result.
x - the first number. y - the second number.
msb - will contain the most significant bits.
lsb - will contain the least significant bits."
        ),
        builtin_function!(
            uadCarry [(uvec_type x, uvec_type y, Out uvec_type carry,) -> uvec_type,]
            "Adds two unsigned integers and generates carry."
        ),
        builtin_function!(
            usubBorrow [(uvec_type x, uvec_type y, Out uvec_type borrow,) -> uvec_type,]
            "Subtracts two unsigned integers and generates borrow."
        ),
        builtin_function!(
            ldexp [(vec_type x, Out ivec_type exp,) -> vec_type,]
"Assemble a floating-point number from a value and exponent. If this
product is too large to be represented in the floating-point type the
result is undefined."
        ),
        builtin_function!(
            frexp [(vec_type x, Out ivec_type exp,) -> vec_type,]
"Splits a floating-point number(x) into significand (in the range of [0.5, 1.0])
and an integral exponent. For x equals zero the significand and exponent are
both zero. For x of infinity or NaN, the results are undefined."
        )
    ])
}

