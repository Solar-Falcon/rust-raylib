use crate::{
    core::{Raylib, Vector3, Vector4},
    ffi,
};

use std::{ffi::CString, mem::transmute, ops::Deref, sync::Arc};

use static_assertions::{assert_eq_align, assert_eq_size};

pub use crate::ffi::PixelFormat;

/// Get pixel data size in bytes for certain format
#[inline]
pub fn get_pixel_data_size(width: u32, height: u32, format: PixelFormat) -> usize {
    unsafe { ffi::GetPixelDataSize(width as i32, height as i32, format as i32) as usize }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

assert_eq_size!(Color, ffi::Color);
assert_eq_align!(Color, ffi::Color);

impl Color {
    #[inline]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Get color with alpha applied, alpha goes from 0.0f to 1.0f
    #[inline]
    pub fn fade(self, alpha: f32) -> Self {
        unsafe { transmute(ffi::Fade(transmute(self), alpha)) }
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
        unsafe { transmute(ffi::ColorToHSV(transmute(self))) }
    }

    /// Get a Color from HSV values, hue [0..360], saturation/value [0..1]
    #[inline]
    pub fn from_hsv(hue: f32, saturation: f32, value: f32) -> Self {
        unsafe { transmute(ffi::ColorFromHSV(hue, saturation, value)) }
    }

    /// Get color multiplied with another color
    #[inline]
    pub fn tint(self, tint: Self) -> Self {
        unsafe { transmute(ffi::ColorTint(transmute(self), transmute(tint))) }
    }

    /// Get color with brightness correction, brightness factor goes from -1.0f to 1.0f
    #[inline]
    pub fn brightness(self, factor: f32) -> Self {
        unsafe { transmute(ffi::ColorBrightness(transmute(self), factor)) }
    }

    /// Get color with contrast correction, contrast values between -1.0f and 1.0f
    #[inline]
    pub fn contrast(self, contrast: f32) -> Self {
        unsafe { transmute(ffi::ColorContrast(transmute(self), contrast)) }
    }

    /// Get color with alpha applied, alpha goes from 0.0f to 1.0f
    #[inline]
    pub fn alpha(self, alpha: f32) -> Self {
        unsafe { transmute(ffi::ColorAlpha(transmute(self), alpha)) }
    }

    /// Get src alpha-blended into dst color with tint
    #[inline]
    pub fn alpha_blend(self, src: Self, tint: Self) -> Self {
        unsafe {
            transmute(ffi::ColorAlphaBlend(
                transmute(self),
                transmute(src),
                transmute(tint),
            ))
        }
    }

    /// Get Color from a source pixel pointer of certain format (uncompressed formats only)
    ///
    /// Returns `None` if buffer isn't large enough
    #[inline]
    pub fn get_pixel_color(source: &[u8], format: PixelFormat) -> Option<Self> {
        if source.len() >= get_pixel_data_size(1, 1, format) {
            unsafe {
                Some(transmute(ffi::GetPixelColor(
                    source.as_ptr() as *mut core::ffi::c_void,
                    format as i32,
                )))
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
                    transmute(self),
                    format as i32,
                );
            }
            true
        } else {
            false
        }
    }
}

/// Image, pixel data stored in CPU memory (RAM)
#[derive(Debug)]
pub struct Image {
    raw: ffi::Image,
}

impl Image {
    /// Image base width
    #[inline]
    pub fn width(&self) -> u32 {
        self.raw.width as u32
    }

    /// Image base height
    #[inline]
    pub fn height(&self) -> u32 {
        self.raw.height as u32
    }

    /// Mipmap levels, 1 by default
    #[inline]
    pub fn mipmaps(&self) -> u32 {
        self.raw.mipmaps as u32
    }

    /// Data format
    #[inline]
    pub fn format(&self) -> PixelFormat {
        unsafe { transmute(self.raw.format) }
    }

    /// Load image from file into CPU memory (RAM)
    #[inline]
    pub fn from_file(filename: &str) -> Self {
        let filename = CString::new(filename).unwrap();

        Self {
            raw: unsafe { ffi::LoadImage(filename.as_ptr()) },
        }
    }

    /// Load image from RAW file data
    #[inline]
    pub fn from_raw_file(
        filename: &str,
        width: u32,
        height: u32,
        format: PixelFormat,
        header_size: u32,
    ) -> Self {
        let filename = CString::new(filename).unwrap();

        Self {
            raw: unsafe {
                ffi::LoadImageRaw(
                    filename.as_ptr(),
                    width as i32,
                    height as i32,
                    format as i32,
                    header_size as i32,
                )
            },
        }
    }

    /// Load image sequence from file (frames appended to image.data)
    ///
    /// Returns the amount of frames in the image.
    #[inline]
    pub fn from_file_anim(filename: &str) -> (Self, usize) {
        let filename = CString::new(filename).unwrap();
        let mut frames: i32 = 0;

        let image = unsafe { ffi::LoadImageAnim(filename.as_ptr(), (&mut frames) as *mut i32) };

        (Self { raw: image }, frames as usize)
    }

    /// Load image from memory buffer, fileType refers to extension: i.e. '.png'
    #[inline]
    pub fn from_memory(filetype: &str, filedata: &[u8]) -> Self {
        let filetype = CString::new(filetype).unwrap();

        Self {
            raw: unsafe {
                ffi::LoadImageFromMemory(
                    filetype.as_ptr(),
                    filedata.as_ptr(),
                    filedata.len() as i32,
                )
            },
        }
    }

    /// Load image from GPU texture data
    #[inline]
    pub fn from_texture(texture: &Texture) -> Self {
        Self {
            raw: unsafe { ffi::LoadImageFromTexture(texture.raw.deref().clone()) },
        }
    }

    /// Load image from screen buffer and (screenshot)
    #[inline]
    pub fn from_screen(_raylib: &Raylib) -> Self {
        Self {
            raw: unsafe { ffi::LoadImageFromScreen() },
        }
    }

    /// Check if an image is ready
    #[inline]
    pub fn is_ready(&self) -> bool {
        unsafe { ffi::IsImageReady(self.raw.clone()) }
    }

    /// Export image data to file, returns true on success
    #[inline]
    pub fn export(&self, filename: &str) -> bool {
        let filename = CString::new(filename).unwrap();

        unsafe { ffi::ExportImage(self.raw.clone(), filename.as_ptr()) }
    }

    /// Export image as code file defining an array of bytes, returns true on success
    #[inline]
    pub fn export_as_code(&self, filename: &str) -> bool {
        let filename = CString::new(filename).unwrap();

        unsafe { ffi::ExportImageAsCode(self.raw.clone(), filename.as_ptr()) }
    }

    /*
    /// Generate image: plain color
    pub fn GenImageColor(width: core::ffi::c_int, height: core::ffi::c_int, color: Color, ) -> Image;
    /// Generate image: vertical gradient
    pub fn GenImageGradientV(width: core::ffi::c_int, height: core::ffi::c_int, top: Color, bottom: Color, ) -> Image;
    /// Generate image: horizontal gradient
    pub fn GenImageGradientH(width: core::ffi::c_int, height: core::ffi::c_int, left: Color, right: Color, ) -> Image;
    /// Generate image: radial gradient
    pub fn GenImageGradientRadial(width: core::ffi::c_int, height: core::ffi::c_int, density: core::ffi::c_float, inner: Color, outer: Color, ) -> Image;
    /// Generate image: checked
    pub fn GenImageChecked(width: core::ffi::c_int, height: core::ffi::c_int, checksX: core::ffi::c_int, checksY: core::ffi::c_int, col1: Color, col2: Color, ) -> Image;
    /// Generate image: white noise
    pub fn GenImageWhiteNoise(width: core::ffi::c_int, height: core::ffi::c_int, factor: core::ffi::c_float, ) -> Image;
    /// Generate image: perlin noise
    pub fn GenImagePerlinNoise(width: core::ffi::c_int, height: core::ffi::c_int, offsetX: core::ffi::c_int, offsetY: core::ffi::c_int, scale: core::ffi::c_float, ) -> Image;
    /// Generate image: cellular algorithm, bigger tileSize means bigger cells
    pub fn GenImageCellular(width: core::ffi::c_int, height: core::ffi::c_int, tileSize: core::ffi::c_int, ) -> Image;
    /// Generate image: grayscale image from text data
    pub fn GenImageText(width: core::ffi::c_int, height: core::ffi::c_int, text: *const core::ffi::c_char, ) -> Image;
    */

    #[inline]
    pub fn get_pixel_data_size(&self) -> usize {
        unsafe { ffi::GetPixelDataSize(self.raw.width, self.raw.height, self.raw.format) as usize }
    }
}

impl Drop for Image {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::UnloadImage(self.raw.clone()) }
    }
}

#[derive(Clone, Debug)]
pub struct Texture {
    raw: Arc<ffi::Texture>,
}

/// Texture2D, same as Texture
pub type Texture2D = Texture;

/// TextureCubemap, same as Texture
pub type TextureCubemap = Texture;

// /// RenderTexture2D, same as RenderTexture
// pub type RenderTexture2D = RenderTexture;
