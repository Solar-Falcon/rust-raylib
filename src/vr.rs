use crate::{ffi, math::Matrix};

use static_assertions::{assert_eq_align, assert_eq_size};

/// VrDeviceInfo, Head-Mounted-Display device parameters
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VrDeviceInfo {
    /// Horizontal resolution in pixels
    pub horizontal_resolution: u32,
    /// Vertical resolution in pixels
    pub vertical_resolution: u32,
    /// Horizontal size in meters
    pub horizontal_screen_size: f32,
    /// Vertical size in meters
    pub vertical_screen_size: f32,
    /// Screen center in meters
    pub screen_center_v: f32,
    /// Distance between eye and display in meters
    pub eye_to_screen_distance: f32,
    /// Lens separation distance in meters
    pub lens_separation_distance: f32,
    /// IPD (distance between pupils) in meters
    pub interpupillary_distance: f32,
    /// Lens distortion constant parameters
    pub lens_distortion_values: [f32; 4],
    /// Chromatic aberration correction parameters
    pub chroma_ab_correction: [f32; 4],
}

assert_eq_size!(VrDeviceInfo, ffi::VrDeviceInfo);
assert_eq_align!(VrDeviceInfo, ffi::VrDeviceInfo);

impl Into<ffi::VrDeviceInfo> for VrDeviceInfo {
    #[inline]
    fn into(self) -> ffi::VrDeviceInfo {
        unsafe { std::mem::transmute(self) }
    }
}

impl From<ffi::VrDeviceInfo> for VrDeviceInfo {
    #[inline]
    fn from(value: ffi::VrDeviceInfo) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

/// VrStereoConfig, VR stereo rendering configuration for simulator
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VrStereoConfig {
    /// VR projection matrices (per eye)
    pub projection: [Matrix; 2],
    /// VR view offset matrices (per eye)
    pub view_offset: [Matrix; 2],
    /// VR left lens center
    pub left_lens_center: [f32; 2],
    /// VR right lens center
    pub right_lens_center: [f32; 2],
    /// VR left screen center
    pub left_screen_center: [f32; 2],
    /// VR right screen center
    pub right_screen_center: [f32; 2],
    /// VR distortion scale
    pub scale: [f32; 2],
    /// VR distortion scale in
    pub scale_in: [f32; 2],
}

impl VrStereoConfig {
    /// Load VR stereo config for VR simulator device parameters
    pub fn load(device: VrDeviceInfo) -> Self {
        // raylib 4.5.0 doesn't allocate VrStereoConfig and UnloadVrStereoConfig is an empty funcy
        assert_eq!(crate::RAYLIB_VERSION, "4.5");

        unsafe { ffi::LoadVrStereoConfig(device.into()).into() }
    }
}

assert_eq_size!(VrStereoConfig, ffi::VrStereoConfig);
assert_eq_align!(VrStereoConfig, ffi::VrStereoConfig);

impl Into<ffi::VrStereoConfig> for VrStereoConfig {
    #[inline]
    fn into(self) -> ffi::VrStereoConfig {
        // raylib 4.5.0 doesn't allocate VrStereoConfig and UnloadVrStereoConfig is an empty func
        assert_eq!(crate::RAYLIB_VERSION, "4.5");

        unsafe { std::mem::transmute(self) }
    }
}

impl From<ffi::VrStereoConfig> for VrStereoConfig {
    #[inline]
    fn from(value: ffi::VrStereoConfig) -> Self {
        // raylib 4.5.0 doesn't allocate VrStereoConfig and UnloadVrStereoConfig is an empty funcy
        assert_eq!(crate::RAYLIB_VERSION, "4.5");

        unsafe { std::mem::transmute(value) }
    }
}

impl Drop for VrStereoConfig {
    #[inline]
    fn drop(&mut self) {
        // raylib 4.5.0 doesn't allocate VrStereoConfig and UnloadVrStereoConfig is an empty func
        assert_eq!(crate::RAYLIB_VERSION, "4.5");

        // unsafe { ffi::UnloadVrStereoConfig( ... ) }
    }
}
