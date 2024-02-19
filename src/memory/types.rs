use std::collections::HashMap;

use crate::lexer::{Token, TokenKind};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Primitive {
    Float,
    Int,
    Uint,
    Bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GenericSize {
    Number,
    GVec2Type,
    GVec3Type,
    GVec4Type,
    GSampler2D,
    GSampler2DArray,
    GSampler3D,
}
impl GenericSize {
    pub fn as_size(&self) -> Option<u32> {
        match self {
            GenericSize::Number => Some(1),
            GenericSize::GVec2Type => Some(2),
            GenericSize::GVec3Type => Some(3),
            GenericSize::GVec4Type => Some(4),
            GenericSize::GSampler2D => None,
            GenericSize::GSampler2DArray => None,
            GenericSize::GSampler3D => None,
        } 
    }
}

#[derive(Clone, Debug)]
pub struct TypeInfo {
    pub base: String,
    /// A size of '0' means that this is not an array.
    pub size: u32,
}
impl TypeInfo {
    pub fn void() -> Self {
        Self { base: "void".to_string(), size: 0 }
    }
    pub fn from_str(base: &str) -> Self {
        Self { base: base.to_string(), size: 0 }
    }
    pub fn from_primitive(token: Token) -> Self {
        match token.kind {
            TokenKind::IntConstant => Self { base: "int".to_string(), size: 0},
            TokenKind::UintConstant => Self { base: "uint".to_string(), size: 0},
            TokenKind::FloatConstant => Self { base: "float".to_string(), size: 0},
            TokenKind::BoolConstant => Self { base: "bool".to_string(), size: 0},
            _ => unreachable!()
        } 
    }
    pub fn to_string(&self) -> String {
        if self.size != 0 {
            format!("{}[{}]", self.base, self.size.to_string())
        } else {
            self.base.clone()
        }
    }
    pub fn from_pieces(primitive_type: Primitive, generic_size: GenericSize) -> Self {
        use Primitive::*;
        use GenericSize::*;
        match (primitive_type, generic_size) {
            (Float, Number) => Self { base: "float".to_string(), size: 0 },
            (Float, GVec2Type) => Self { base: "vec2".to_string(), size: 0 },
            (Float, GVec3Type) => Self { base: "vec3".to_string(), size: 0 },
            (Float, GVec4Type) => Self { base: "vec4".to_string(), size: 0 },
            (Int, Number) => Self { base: "int".to_string(), size: 0 },
            (Int, GVec2Type) => Self { base: "ivec2".to_string(), size: 0 },
            (Int, GVec3Type) => Self { base: "ivec3".to_string(), size: 0 },
            (Int, GVec4Type) => Self { base: "ivec4".to_string(), size: 0 },
            (Uint, Number) => Self { base: "uint".to_string(), size: 0 },
            (Uint, GVec2Type) => Self { base: "uvec2".to_string(), size: 0 },
            (Uint, GVec3Type) => Self { base: "uvec3".to_string(), size: 0 },
            (Uint, GVec4Type) => Self { base: "uvec4".to_string(), size: 0 },
            (Bool, Number) => Self { base: "bint".to_string(), size: 0 },
            (Bool, GVec2Type) => Self { base: "bvec2".to_string(), size: 0 },
            (Bool, GVec3Type) => Self { base: "bvec3".to_string(), size: 0 },
            (Bool, GVec4Type) => Self { base: "bvec4".to_string(), size: 0 },
            (Float, GSampler2D) => Self { base: "sampler2D".to_string(), size: 0 },
            (Float, GSampler2DArray) => Self { base: "sampler2DArray".to_string(), size: 0 },
            (Float, GSampler3D) => Self { base: "sampler3D".to_string(), size: 0 },
            (Int, GSampler2D) => Self { base: "isampler2D".to_string(), size: 0 },
            (Int, GSampler2DArray) => Self { base: "isampler2DArray".to_string(), size: 0 },
            (Int, GSampler3D) => Self { base: "isampler3D".to_string(), size: 0 },
            (Uint, GSampler2D) => Self { base: "usampler2D".to_string(), size: 0 },
            (Uint, GSampler2DArray) => Self { base: "usampler2DArray".to_string(), size: 0 },
            (Uint, GSampler3D) => Self { base: "usampler3D".to_string(), size: 0 },
            _ => panic!()
        }
    }
    pub fn get_generic_type(&self) -> Option<Primitive> {
        match self.base.as_str() {
            "float" => Some(Primitive::Float),
            "vec2" => Some(Primitive::Float),
            "vec3" => Some(Primitive::Float),
            "vec4" => Some(Primitive::Float),
            "vec_type" => Some(Primitive::Float),
            "int" => Some(Primitive::Int),
            "ivec2" => Some(Primitive::Int),
            "ivec3" => Some(Primitive::Int),
            "ivec4" => Some(Primitive::Int),
            "ivec_type" => Some(Primitive::Int),
            "uint" => Some(Primitive::Uint),
            "uvec2" => Some(Primitive::Uint),
            "uvec3" => Some(Primitive::Uint),
            "uvec4" => Some(Primitive::Uint),
            "uvec_type" => Some(Primitive::Uint),
            "bool" => Some(Primitive::Bool),
            "bvec2" => Some(Primitive::Bool),
            "bvec3" => Some(Primitive::Bool),
            "bvec4" => Some(Primitive::Bool),
            "bvec_type" => Some(Primitive::Float),
            "sampler2D" => Some(Primitive::Float),
            "sampler2DArray" => Some(Primitive::Float),
            "sampler3D" => Some(Primitive::Float),
            "samplerCube" => Some(Primitive::Float),
            "samplerCubeArray" => Some(Primitive::Float),
            "isampler2D" => Some(Primitive::Int),
            "isampler2DArray" => Some(Primitive::Int),
            "isampler3D" => Some(Primitive::Int),
            "usampler2D" => Some(Primitive::Uint),
            "usampler2DArray" => Some(Primitive::Uint),
            "usampler3D" => Some(Primitive::Uint),
            _ => None
        }
    }
    pub fn get_generic_size(&self) -> Option<GenericSize> {
        match self.base.as_str() {
            "float" => Some(GenericSize::Number),
            "int" => Some(GenericSize::Number),
            "uint" => Some(GenericSize::Number),
            "number" => Some(GenericSize::Number),
            "vec2" => Some(GenericSize::GVec2Type),
            "ivec2" => Some(GenericSize::GVec2Type),
            "uvec2" => Some(GenericSize::GVec2Type),
            "gvec2_type" => Some(GenericSize::GVec2Type),
            "vec3" => Some(GenericSize::GVec3Type),
            "ivec3" => Some(GenericSize::GVec3Type),
            "uvec3" => Some(GenericSize::GVec3Type),
            "gvec3_type" => Some(GenericSize::GVec3Type),
            "vec4" => Some(GenericSize::GVec4Type),
            "ivec4" => Some(GenericSize::GVec4Type),
            "uvec4" => Some(GenericSize::GVec4Type),
            "gvec4_type" => Some(GenericSize::GVec3Type),
            "sampler2D" => Some(GenericSize::GSampler2D),
            "isampler2D" => Some(GenericSize::GSampler2D),
            "usampler2D" => Some(GenericSize::GSampler2D),
            "gsampler2D" => Some(GenericSize::GSampler2D),
            "sampler2DArray" => Some(GenericSize::GSampler2DArray),
            "isampler2DArray" => Some(GenericSize::GSampler2DArray),
            "usampler2DArray" => Some(GenericSize::GSampler2DArray),
            "gsampler2DArray" => Some(GenericSize::GSampler2DArray),
            "sampler3D" => Some(GenericSize::GSampler3D),
            "isampler3D" => Some(GenericSize::GSampler3D),
            "usampler3D" => Some(GenericSize::GSampler3D),
            "gsampler3D" => Some(GenericSize::GSampler3D),
            _ => None 
        }
    }
    /// Checks if this type is a generically typed type, and returns its size.
    pub fn is_generically_typed(&self) -> Option<GenericSize> {
        match self.base.as_str() {
            "number" => Some(GenericSize::Number),
            "gvec2_type" => Some(GenericSize::GVec2Type),
            "gvec3_type" => Some(GenericSize::GVec3Type),
            "gvec4_type" => Some(GenericSize::GVec4Type),
            "gsampler2D" => Some(GenericSize::GSampler2D),
            "gsampler2DArray" => Some(GenericSize::GSampler2DArray),
            "gsampler3D" => Some(GenericSize::GSampler3D),
            _ => None,
        }
    }
    /// Checks if this type is a generically sized type, and returns its primitive type.
    pub fn is_generically_sized(&self) -> Option<Primitive> {
        match self.base.as_str() {
            "vec_type" => Some(Primitive::Float),
            "ivec_type" => Some(Primitive::Int),
            "uvec_type" => Some(Primitive::Uint),
            "bvec_type" => Some(Primitive::Bool),
            "mat_type" => Some(Primitive::Float),
            _ => None,
        }
    }
}
impl PartialEq for TypeInfo {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }
        if self.base == other.base {
            return true;
        }
        let base = self.base.as_str();
        let other_base = other.base.as_str();
        match base {
            "vec_type" => if ["float", "vec2", "vec3", "vec4"].contains(&other_base) {
                return true;
            }
            "ivec_type" => if ["int", "ivec2", "ivec3", "ivec4"].contains(&other_base) {
                return true;
            }
            "uvec_type" => if ["uint", "uvec2", "uvec3", "uvec4"].contains(&other_base) {
                return true;
            }
            "bvec_type" => if ["bool", "bvec2", "bvec3", "bvec4"].contains(&other_base) {
                return true;
            }
            "mat_type" => if ["mat2", "mat3", "mat4"].contains(&other_base) {
                return true;
            }
            "number" => if ["float", "int", "uint"].contains(&other_base) {
                return true;
            }
            "gvec2_type" => if ["vec2", "ivec2", "uvec2"].contains(&other_base) {
                return true;
            }
            "gvec3_type" => if ["vec3", "ivec3", "uvec3"].contains(&other_base) {
                return true;
            }
            "gvec4_type" => if ["vec4", "ivec4", "uvec4"].contains(&other_base) {
                return true;
            }
            "gsampler2D" => if ["sampler2D", "isampler2D", "usampler2D"].contains(&other_base) {
                return true;
            }
            "gsampler3D" => if ["sampler3D", "isampler3D", "usampler3D"].contains(&other_base) {
                return true;
            }
            "gsampler2DArray" => if ["sampler2DArray", "isampler2DArray", "usampler2DArray"].contains(&other_base) {
                return true;
            }
            _ => {}
        }
        match other_base {
            "vec_type" => if ["float", "vec2", "vec3", "vec4"].contains(&base) {
                return true;
            }
            "ivec_type" => if ["int", "ivec2", "ivec3", "ivec4"].contains(&base) {
                return true;
            }
            "uvec_type" => if ["uint", "uvec2", "uvec3", "uvec4"].contains(&base) {
                return true;
            }
            "bvec_type" => if ["bool", "bvec2", "bvec3", "bvec4"].contains(&base) {
                return true;
            }
            "mat_type" => if ["mat2", "mat3", "mat4"].contains(&base) {
                return true;
            }
            "number" => if ["float", "int", "uint"].contains(&base) {
                return true;
            }
            "gvec2_type" => if ["vec2", "ivec2", "uvec2"].contains(&base) {
                return true;
            }
            "gvec3_type" => if ["vec2", "ivec2", "uvec2"].contains(&base) {
                return true;
            }
            "gvec4_type" => if ["vec2", "ivec2", "uvec2"].contains(&base) {
                return true;
            }
            "gsampler2D" => if ["sampler2D", "isampler2D", "usampler2D"].contains(&base) {
                return true;
            }
            "gsampler3D" => if ["sampler3D", "isampler3D", "usampler3D"].contains(&base) {
                return true;
            }
            "gsampler2DArray" => if ["sampler2DArray", "isampler2DArray", "usampler2DArray"].contains(&base) {
                return true;
            }
            _ => {}
        }
        false
    }
}
impl Eq for TypeInfo{}

pub struct BuiltinTypeInfo {
    pub description: String,
    pub used_anywhere: bool
}

macro_rules! builtin_type {
    ($name:ident $anywhere:literal $desc:literal) => {
        (stringify!($name).to_string(), BuiltinTypeInfo {
            description: $desc.to_string(),
            used_anywhere: $anywhere
        })
    };
}

pub fn make_builtin_types() -> HashMap<String, BuiltinTypeInfo> {
    HashMap::from([
        builtin_type!(void false "Void datatype, useful only for functions that return nothing."),
        builtin_type!(bool true "Boolean datatype, can only contain true or false."),
        builtin_type!(bvec2 true "Two-component vector of booleans."),
        builtin_type!(bvec3 true "Three-component vector of booleans."),
        builtin_type!(bvec4 true "Four-component vector of booleans."),
        builtin_type!(int true "Signed scalar integer."),
        builtin_type!(ivec2 true "Two-component vector of signed integers."),
        builtin_type!(ivec3 true "Three-component vector of signed integers."),
        builtin_type!(ivec4 true "Four-component vector of signed integers."),
        builtin_type!(uint true "Unsigned scalar integer; can't contain negative numbers."),
        builtin_type!(uvec2 true "Two-component vector of unsigned integers."),
        builtin_type!(uvec3 true "Three-component vector of unsigned integers."),
        builtin_type!(uvec4 true "Four-component vector of unsigned integers."),
        builtin_type!(float true "Floating-point scalar."),
        builtin_type!(vec2 true "Two-component vector of floating-point values."),
        builtin_type!(vec3 true "Three-component vector of floating-point values."),
        builtin_type!(vec4 true "Four-component vector of floating-point values."),
        builtin_type!(mat2 true "2x2 matrix, in column major order."),
        builtin_type!(mat3 true "3x3 matrix, in column major order."),
        builtin_type!(mat4 true "4x4 matrix, in column major order."),
        builtin_type!(sampler2D  false
"Sampler type for binding 2D textures, which are read as float."),
        builtin_type!(isampler2D false
"Sampler type for binding 2D textures, which are read as signed integer."),
        builtin_type!(usampler2D false
"Sampler type for binding 2D textures, which are read as unsigned integer."),
        builtin_type!(sampler2DArray false
"Sampler type for binding 2D texture arrays, which are read as float."),
        builtin_type!(isampler2DArray false
"Sampler type for binding 2D texture arrays, which are read as signed integer."),
        builtin_type!(usampler2DArray false
"Sampler type for binding 2D texture arrays, which are read as unsigned integer."),
        builtin_type!(sampler3D false
"Sampler type for binding 3D textures, which are read as float."),
        builtin_type!(isampler3D false
"Sampler type for binding 3D textures, which are read as signed integer."),
        builtin_type!(usampler3D false
"Sampler type for binding 3D textures, which are read as unsigned integer."),
        builtin_type!(samplerCube false
"Sampler type for binding Cubemaps, which are read as float."),
        builtin_type!(samplerCubeArray false
"Sampler type for binding Cubemap arrays, which are read as float."),
    ])
}


