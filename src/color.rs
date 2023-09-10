use crate::{
    ffi,
    math::{Vector3, Vector4},
    texture::{get_pixel_data_size, PixelFormat},
};
use static_assertions::{assert_eq_align, assert_eq_size};

/// Color, 4 components, R8G8B8A8 (32bit)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Color {
    /// Color red value
    pub r: u8,
    /// Color green value
    pub g: u8,
    /// Color blue value
    pub b: u8,
    /// Color alpha value
    pub a: u8,
}

assert_eq_size!(Color, ffi::Color);
assert_eq_align!(Color, ffi::Color);

impl Color {
    pub const LIGHTGRAY: Color = Color {
        r: 200,
        g: 200,
        b: 200,
        a: 255,
    };

    pub const GRAY: Color = Color {
        r: 130,
        g: 130,
        b: 130,
        a: 255,
    };

    pub const DARKGRAY: Color = Color {
        r: 80,
        g: 80,
        b: 80,
        a: 255,
    };

    pub const YELLOW: Color = Color {
        r: 253,
        g: 249,
        b: 0,
        a: 255,
    };

    pub const GOLD: Color = Color {
        r: 255,
        g: 203,
        b: 0,
        a: 255,
    };

    pub const ORANGE: Color = Color {
        r: 255,
        g: 161,
        b: 0,
        a: 255,
    };

    pub const PINK: Color = Color {
        r: 255,
        g: 109,
        b: 194,
        a: 255,
    };

    pub const RED: Color = Color {
        r: 230,
        g: 41,
        b: 55,
        a: 255,
    };

    pub const MAROON: Color = Color {
        r: 190,
        g: 33,
        b: 55,
        a: 255,
    };

    pub const GREEN: Color = Color {
        r: 0,
        g: 228,
        b: 48,
        a: 255,
    };

    pub const LIME: Color = Color {
        r: 0,
        g: 158,
        b: 47,
        a: 255,
    };

    pub const DARKGREEN: Color = Color {
        r: 0,
        g: 117,
        b: 44,
        a: 255,
    };

    pub const SKYBLUE: Color = Color {
        r: 102,
        g: 191,
        b: 255,
        a: 255,
    };

    pub const BLUE: Color = Color {
        r: 0,
        g: 121,
        b: 241,
        a: 255,
    };

    pub const DARKBLUE: Color = Color {
        r: 0,
        g: 82,
        b: 172,
        a: 255,
    };

    pub const PURPLE: Color = Color {
        r: 200,
        g: 122,
        b: 255,
        a: 255,
    };

    pub const VIOLET: Color = Color {
        r: 135,
        g: 60,
        b: 190,
        a: 255,
    };

    pub const DARKPURPLE: Color = Color {
        r: 112,
        g: 31,
        b: 126,
        a: 255,
    };

    pub const BEIGE: Color = Color {
        r: 211,
        g: 176,
        b: 131,
        a: 255,
    };

    pub const BROWN: Color = Color {
        r: 127,
        g: 106,
        b: 79,
        a: 255,
    };

    pub const DARKBROWN: Color = Color {
        r: 76,
        g: 63,
        b: 47,
        a: 255,
    };

    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };

    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };

    pub const BLANK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };

    pub const MAGENTA: Color = Color {
        r: 255,
        g: 0,
        b: 255,
        a: 255,
    };

    pub const RAYWHITE: Color = Color {
        r: 245,
        g: 245,
        b: 245,
        a: 255,
    };

    #[inline]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Get color with alpha applied, alpha goes from 0.0f to 1.0f
    #[inline]
    pub fn fade(self, alpha: f32) -> Self {
        unsafe { ffi::Fade(self.into(), alpha).into() }
    }

    /// Get hexadecimal value for a Color
    #[inline]
    pub fn to_hex(self) -> u32 {
        // no real need to use ffi here
        ((self.r as u32) << 24) | ((self.g as u32) << 16) | ((self.b as u32) << 8) | (self.a as u32)
    }

    /// Get Color structure from hexadecimal value
    #[inline]
    pub fn from_hex(val: u32) -> Self {
        // no real need to use ffi here
        Self {
            r: (val >> 24 & 0xFF) as u8,
            g: (val >> 16 & 0xFF) as u8,
            b: (val >> 8 & 0xFF) as u8,
            a: (val & 0xFF) as u8,
        }
    }

    /// Get Color normalized as float [0..1]
    #[inline]
    pub fn normalize(self) -> Vector4 {
        // no real need to use ffi here
        Vector4 {
            x: self.r as f32 / 255.,
            y: self.g as f32 / 255.,
            z: self.b as f32 / 255.,
            w: self.a as f32 / 255.,
        }
    }

    /// Get Color from normalized values [0..1]
    #[inline]
    pub fn from_normalized(normalized: Vector4) -> Self {
        // no real need to use ffi here
        Self {
            r: (normalized.x * 255.) as u8,
            g: (normalized.y * 255.) as u8,
            b: (normalized.z * 255.) as u8,
            a: (normalized.w * 255.) as u8,
        }
    }

    /// Get HSV values for a Color, hue [0..360], saturation/value [0..1]
    #[inline]
    pub fn to_hsv(self) -> Vector3 {
        unsafe { ffi::ColorToHSV(self.into()).into() }
    }

    /// Get a Color from HSV values, hue [0..360], saturation/value [0..1]
    #[inline]
    pub fn from_hsv(hue: f32, saturation: f32, value: f32) -> Self {
        unsafe { ffi::ColorFromHSV(hue, saturation, value).into() }
    }

    /// Get color multiplied with another color
    #[inline]
    pub fn tint(self, tint: Self) -> Self {
        unsafe { ffi::ColorTint(self.into(), tint.into()).into() }
    }

    /// Get color with brightness correction, brightness factor goes from -1.0f to 1.0f
    #[inline]
    pub fn brightness(self, factor: f32) -> Self {
        unsafe { ffi::ColorBrightness(self.into(), factor).into() }
    }

    /// Get color with contrast correction, contrast values between -1.0f and 1.0f
    #[inline]
    pub fn contrast(self, contrast: f32) -> Self {
        unsafe { ffi::ColorContrast(self.into(), contrast).into() }
    }

    /// Get color with alpha applied, alpha goes from 0.0f to 1.0f
    #[inline]
    pub fn alpha(self, alpha: f32) -> Self {
        unsafe { ffi::ColorAlpha(self.into(), alpha).into() }
    }

    /// Get src alpha-blended into dst color with tint
    #[inline]
    pub fn alpha_blend(self, src: Self, tint: Self) -> Self {
        unsafe { ffi::ColorAlphaBlend(self.into(), src.into(), tint.into()).into() }
    }

    /// Get Color from a source pixel pointer of certain format (uncompressed formats only)
    ///
    /// Returns `None` if buffer isn't large enough
    #[inline]
    pub fn get_pixel_color(source: &[u8], format: PixelFormat) -> Option<Self> {
        if source.len() >= get_pixel_data_size(1, 1, format) {
            unsafe {
                Some(
                    ffi::GetPixelColor(source.as_ptr() as *mut core::ffi::c_void, format as _)
                        .into(),
                )
            }
        } else {
            None
        }
    }

    /// Set color formatted into destination pixel pointer (uncompressed formats only)
    ///
    /// Returns `true` on success, `false` if buffer isn't large enough
    #[inline]
    pub fn set_pixel_color(self, dest: &mut [u8], format: PixelFormat) -> bool {
        if dest.len() >= get_pixel_data_size(1, 1, format) {
            unsafe {
                ffi::SetPixelColor(
                    dest.as_mut_ptr() as *mut core::ffi::c_void,
                    self.into(),
                    format as _,
                );
            }
            true
        } else {
            false
        }
    }
}

impl From<Color> for ffi::Color {
    #[inline]
    fn from(val: Color) -> Self {
        unsafe { std::mem::transmute(val) }
    }
}

impl From<ffi::Color> for Color {
    #[inline]
    fn from(value: ffi::Color) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
