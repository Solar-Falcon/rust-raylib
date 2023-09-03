/// Raw ffi bindings
pub mod ffi;

pub use ffi::{RAYLIB_VERSION, RAYLIB_VERSION_MAJOR, RAYLIB_VERSION_MINOR, RAYLIB_VERSION_PATCH};

/// Color type and color constants
pub mod color;
/// Drawing traits and functions
pub mod drawing;
/// Math types
pub mod math;
/// Shader type
pub mod shader;
/// Fonts and text related types and functions
pub mod text;
/// Images and textures
pub mod texture;
/// VR related types
pub mod vr;

mod core;
pub use crate::core::*;
