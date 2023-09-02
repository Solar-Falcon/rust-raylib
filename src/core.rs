use crate::ffi;

use std::ffi::CString;

use static_assertions::{assert_eq_align, assert_eq_size};

// <temporary>
// remaining enum imports
// TODO: move to respective modules
// pub use ffi::{
//     BlendMode, CubemapLayout, FontType,
//     MaterialMapIndex, NPatchLayout,
//     ShaderAttributeDataType, ShaderLocationIndex, ShaderUniformDataType,
//     TextureFilter, TextureWrap,
// };
// </temporary>

pub use ffi::{
    CameraMode, CameraProjection, ConfigFlags, GamepadAxis, GamepadButton, Gesture, KeyboardKey,
    MouseButton, MouseCursor,
};

pub type Vector2 = mint::Vector2<f32>;
assert_eq_size!(Vector2, ffi::Vector2);
assert_eq_align!(Vector2, ffi::Vector2);

pub type Vector3 = mint::Vector3<f32>;
assert_eq_size!(Vector3, ffi::Vector3);
assert_eq_align!(Vector3, ffi::Vector3);

pub type Vector4 = mint::Vector4<f32>;
assert_eq_size!(Vector4, ffi::Vector4);
assert_eq_align!(Vector4, ffi::Vector4);

pub type Quaternion = mint::Quaternion<f32>;
assert_eq_size!(Quaternion, ffi::Quaternion);
assert_eq_align!(Quaternion, ffi::Quaternion);

pub type Matrix = mint::RowMatrix4<f32>;
assert_eq_size!(Matrix, ffi::Matrix);
assert_eq_align!(Matrix, ffi::Matrix);

/// Rectangle, 4 components
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rectangle {
    /// Rectangle top-left corner position x
    pub x: f32,
    /// Rectangle top-left corner position y
    pub y: f32,
    /// Rectangle width
    pub width: f32,
    /// Rectangle height
    pub height: f32,
}

assert_eq_size!(Rectangle, ffi::Rectangle);
assert_eq_align!(Rectangle, ffi::Rectangle);

impl Rectangle {
    #[inline]
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

// /// Camera type fallback, defaults to Camera3D
// pub type Camera = Camera3D;

#[derive(Debug)]
pub struct Raylib(());

impl Raylib {
    /// Initialize window and OpenGL context
    #[inline]
    pub fn init_window(width: u32, height: u32, title: &str) -> Self {
        let title = CString::new(title).unwrap();

        unsafe {
            ffi::InitWindow(width as i32, height as i32, title.as_ptr());
        }

        Self(())
    }

    /// Check if Escape key or Close icon is pressed
    #[inline]
    pub fn window_should_close(&self) -> bool {
        unsafe { ffi::WindowShouldClose() }
    }

    /// Close window and unload OpenGL context
    #[inline]
    pub fn close_window(self) {
        drop(self)
    }

    /// Check if window has been initialized successfully
    #[inline]
    pub fn is_window_ready(&self) -> bool {
        unsafe { ffi::IsWindowReady() }
    }

    /// Check if window is currently fullscreen
    #[inline]
    pub fn is_window_fullscreen(&self) -> bool {
        unsafe { ffi::IsWindowFullscreen() }
    }

    /// Check if window is currently hidden (only PLATFORM_DESKTOP)
    #[inline]
    pub fn is_window_hidden(&self) -> bool {
        unsafe { ffi::IsWindowHidden() }
    }

    /// Check if window is currently minimized (only PLATFORM_DESKTOP)
    #[inline]
    pub fn is_window_minimized() -> bool {
        unsafe { ffi::IsWindowMinimized() }
    }

    /// Check if window is currently maximized (only PLATFORM_DESKTOP)
    #[inline]
    pub fn is_window_maximized() -> bool {
        unsafe { ffi::IsWindowMaximized() }
    }

    /// Check if window is currently focused (only PLATFORM_DESKTOP)
    #[inline]
    pub fn is_window_focused() -> bool {
        unsafe { ffi::IsWindowFocused() }
    }

    /// Check if window has been resized last frame
    #[inline]
    pub fn is_window_resized() -> bool {
        unsafe { ffi::IsWindowResized() }
    }
}

impl Drop for Raylib {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ffi::CloseWindow();
        }
    }
}
