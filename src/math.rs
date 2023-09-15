use crate::ffi;
use static_assertions::{assert_eq_align, assert_eq_size};
use std::mem::transmute;

pub use crate::ffi::{CameraMode, CameraProjection};

/// Vector2, 2x f32 components
pub type Vector2 = mint::Vector2<f32>;
assert_eq_size!(Vector2, ffi::Vector2);
assert_eq_align!(Vector2, ffi::Vector2);

impl From<Vector2> for ffi::Vector2 {
    #[inline]
    fn from(val: Vector2) -> Self {
        unsafe { transmute(val) }
    }
}

impl From<ffi::Vector2> for Vector2 {
    #[inline]
    fn from(value: ffi::Vector2) -> Self {
        unsafe { transmute(value) }
    }
}

/// Vector3, 3x f32 components
pub type Vector3 = mint::Vector3<f32>;
assert_eq_size!(Vector3, ffi::Vector3);
assert_eq_align!(Vector3, ffi::Vector3);

impl From<Vector3> for ffi::Vector3 {
    #[inline]
    fn from(val: Vector3) -> Self {
        unsafe { transmute(val) }
    }
}

impl From<ffi::Vector3> for Vector3 {
    #[inline]
    fn from(value: ffi::Vector3) -> Self {
        unsafe { transmute(value) }
    }
}

/// Vector4, 4x f32 components
pub type Vector4 = mint::Vector4<f32>;
assert_eq_size!(Vector4, ffi::Vector4);
assert_eq_align!(Vector4, ffi::Vector4);

impl From<Vector4> for ffi::Vector4 {
    #[inline]
    fn from(val: Vector4) -> Self {
        unsafe { transmute(val) }
    }
}

impl From<ffi::Vector4> for Vector4 {
    #[inline]
    fn from(value: ffi::Vector4) -> Self {
        unsafe { transmute(value) }
    }
}

/// Quaternion, 4x f32 components
pub type Quaternion = mint::Quaternion<f32>;
assert_eq_size!(Quaternion, ffi::Quaternion);
assert_eq_align!(Quaternion, ffi::Quaternion);

impl From<Quaternion> for ffi::Vector4 {
    #[inline]
    fn from(val: Quaternion) -> Self {
        unsafe { transmute(val) }
    }
}

impl From<ffi::Vector4> for Quaternion {
    #[inline]
    fn from(value: ffi::Vector4) -> Self {
        unsafe { transmute(value) }
    }
}

/// Matrix, 4x4 f32 components, column major
pub type Matrix = mint::ColumnMatrix4<f32>;
assert_eq_size!(Matrix, ffi::Matrix);
assert_eq_align!(Matrix, ffi::Matrix);

impl From<Matrix> for ffi::Matrix {
    #[inline]
    fn from(val: Matrix) -> Self {
        unsafe { transmute(val) }
    }
}

impl From<ffi::Matrix> for Matrix {
    #[inline]
    fn from(value: ffi::Matrix) -> Self {
        unsafe { transmute(value) }
    }
}

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
    /// Create new rectangle
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

impl From<Rectangle> for ffi::Rectangle {
    #[inline]
    fn from(val: Rectangle) -> Self {
        unsafe { transmute(val) }
    }
}

impl From<ffi::Rectangle> for Rectangle {
    #[inline]
    fn from(value: ffi::Rectangle) -> Self {
        unsafe { transmute(value) }
    }
}

/// Ray, ray for raycasting
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct Ray {
    /// Ray position (origin)
    pub position: Vector3,
    /// Ray direction
    pub direction: Vector3,
}

assert_eq_size!(Ray, ffi::Ray);
assert_eq_align!(Ray, ffi::Ray);

impl From<Ray> for ffi::Ray {
    #[inline]
    fn from(val: Ray) -> Self {
        unsafe { transmute(val) }
    }
}

impl From<ffi::Ray> for Ray {
    #[inline]
    fn from(value: ffi::Ray) -> Self {
        unsafe { transmute(value) }
    }
}

/// RayCollision, ray hit information
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct RayCollision {
    /// Did the ray hit something?
    pub hit: bool,
    /// Distance to the nearest hit
    pub distance: f32,
    /// Point of the nearest hit
    pub point: Vector3,
    /// Surface normal of hit
    pub normal: Vector3,
}

assert_eq_size!(RayCollision, ffi::RayCollision);
assert_eq_align!(RayCollision, ffi::RayCollision);

impl From<RayCollision> for ffi::RayCollision {
    #[inline]
    fn from(val: RayCollision) -> Self {
        unsafe { transmute(val) }
    }
}

impl From<ffi::RayCollision> for RayCollision {
    #[inline]
    fn from(value: ffi::RayCollision) -> Self {
        unsafe { transmute(value) }
    }
}

/// BoundingBox
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct BoundingBox {
    /// Minimum vertex box-corner
    pub min: Vector3,
    /// Maximum vertex box-corner
    pub max: Vector3,
}

assert_eq_size!(Ray, ffi::BoundingBox);
assert_eq_align!(Ray, ffi::BoundingBox);

impl From<BoundingBox> for ffi::BoundingBox {
    #[inline]
    fn from(val: BoundingBox) -> Self {
        unsafe { transmute(val) }
    }
}

impl From<ffi::BoundingBox> for BoundingBox {
    #[inline]
    fn from(value: ffi::BoundingBox) -> Self {
        unsafe { transmute(value) }
    }
}

/// Transform, vertex transformation data
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct Transform {
	/// Translation
	pub translation: Vector3,
	/// Rotation
	pub rotation: Quaternion,
	/// Scale
	pub scale: Vector3,
}

assert_eq_size!(Transform, ffi::Transform);
assert_eq_align!(Transform, ffi::Transform);

impl From<Transform> for ffi::Transform {
    #[inline]
    fn from(val: Transform) -> Self {
        unsafe { transmute(val) }
    }
}

impl From<ffi::Transform> for Transform {
    #[inline]
    fn from(value: ffi::Transform) -> Self {
        unsafe { transmute(value) }
    }
}

/// Camera2D, defines position/orientation in 2d space
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Camera2D {
    /// Camera offset (displacement from target)
    pub offset: Vector2,
    /// Camera target (rotation and zoom origin)
    pub target: Vector2,
    /// Camera rotation in degrees
    pub rotation: f32,
    /// Camera zoom (scaling), should be 1.0f by default
    pub zoom: f32,
}

assert_eq_size!(Camera2D, ffi::Camera2D);
assert_eq_align!(Camera2D, ffi::Camera2D);

impl Camera2D {
    /// Get camera 2d transform matrix
    #[inline]
    pub fn get_matrix(&self) -> Matrix {
        unsafe { ffi::GetCameraMatrix2D(self.clone().into()).into() }
    }

    /// Get the world space position for a 2d camera screen space position
    #[inline]
    pub fn screen_to_world(&self, position: Vector2) -> Vector2 {
        unsafe { ffi::GetScreenToWorld2D(position.into(), self.clone().into()).into() }
    }

    /// Get the screen space position for a 2d camera world space position
    #[inline]
    pub fn world_to_screen(&self, position: Vector2) -> Vector2 {
        unsafe { ffi::GetWorldToScreen2D(position.into(), self.clone().into()).into() }
    }
}

impl From<Camera2D> for ffi::Camera2D {
    #[inline]
    fn from(val: Camera2D) -> Self {
        unsafe { transmute(val) }
    }
}

impl From<ffi::Camera2D> for Camera2D {
    #[inline]
    fn from(value: ffi::Camera2D) -> Self {
        unsafe { transmute(value) }
    }
}

/// Camera, defines position/orientation in 3d space
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Camera3D {
    /// Camera position
    pub position: Vector3,
    /// Camera target it looks-at
    pub target: Vector3,
    /// Camera up vector (rotation over its axis)
    pub up: Vector3,
    /// Camera field-of-view aperture in Y (degrees) in perspective, used as near plane width in orthographic
    pub fovy: f32,
    /// Camera projection: CAMERA_PERSPECTIVE or CAMERA_ORTHOGRAPHIC
    pub projection: CameraProjection,
}

assert_eq_size!(Camera3D, ffi::Camera3D);
assert_eq_align!(Camera3D, ffi::Camera3D);

impl Camera3D {
    /// Update camera position for selected mode
    #[inline]
    pub fn update(&mut self, mode: CameraMode) {
        unsafe { ffi::UpdateCamera(self as *mut Camera3D as *mut ffi::Camera3D, mode as _) }
    }

    /// Update camera movement/rotation
    #[inline]
    pub fn update_pro(&mut self, movement: Vector3, rotation: Vector3, zoom: f32) {
        unsafe {
            ffi::UpdateCameraPro(
                self as *mut Camera3D as *mut ffi::Camera3D,
                movement.into(),
                rotation.into(),
                zoom,
            )
        }
    }

    /// Get a ray trace from mouse position
    #[inline]
    pub fn get_mouse_ray(&self, mouse_position: Vector2) -> Ray {
        unsafe { ffi::GetMouseRay(mouse_position.into(), self.clone().into()).into() }
    }

    /// Get camera transform matrix (view matrix)
    #[inline]
    pub fn get_matrix(&self) -> Matrix {
        unsafe { ffi::GetCameraMatrix(self.clone().into()).into() }
    }

    /// Get the screen space position for a 3d world space position
    #[inline]
    pub fn world_to_screen(&self, position: Vector3) -> Vector2 {
        unsafe { ffi::GetWorldToScreen(position.into(), self.clone().into()).into() }
    }

    /// Get size position for a 3d world space position
    #[inline]
    pub fn world_to_screen_ex(&self, position: Vector3, width: u32, height: u32) -> Vector2 {
        unsafe {
            ffi::GetWorldToScreenEx(
                position.into(),
                self.clone().into(),
                width as _,
                height as _,
            )
            .into()
        }
    }
}

impl From<Camera3D> for ffi::Camera3D {
    #[inline]
    fn from(val: Camera3D) -> Self {
        unsafe { transmute(val) }
    }
}

impl From<ffi::Camera3D> for Camera3D {
    #[inline]
    fn from(value: ffi::Camera3D) -> Self {
        if value.projection != CameraProjection::Orthographic as i32
            && value.projection != CameraProjection::Perspective as i32
        {
            panic!(
                "Attempted to convert a ffi::Camera3D with an invalid projection value: {}",
                value.projection
            );
        }

        unsafe { transmute(value) }
    }
}

/// Camera type fallback, defaults to Camera3D
pub type Camera = Camera3D;
