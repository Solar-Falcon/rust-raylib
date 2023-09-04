use crate::{
    color::Color,
    ffi,
    math::{Rectangle, Vector2},
    texture::Image,
};
use std::{ffi::CString, ops::Deref, rc::Rc};

pub use crate::ffi::FontType;

#[derive(Clone, Debug)]
pub struct Font {
    pub(crate) raw: Rc<ffi::Font>,
}

impl Font {
    /// Base size (default chars height)
    #[inline]
    pub fn base_size(&self) -> u32 {
        self.raw.baseSize as _
    }

    /// Number of glyph characters
    #[inline]
    pub fn glyph_count(&self) -> usize {
        self.raw.glyphCount as _
    }

    /// Padding around the glyph characters
    #[inline]
    pub fn glyph_padding(&self) -> u32 {
        self.raw.glyphPadding as _
    }

    /// Load font from file into GPU memory (VRAM)
    #[inline]
    pub fn from_file(filename: &str) -> Self {
        let filename = CString::new(filename).unwrap();

        Self {
            raw: Rc::new(unsafe { ffi::LoadFont(filename.as_ptr()) }),
        }
    }

    /// Load font from file with extended parameters
    #[inline]
    pub fn from_file_ex(filename: &str, font_size: u32, chars: &[char]) -> Self {
        let filename = CString::new(filename).unwrap();

        Self {
            raw: Rc::new(unsafe {
                ffi::LoadFontEx(
                    filename.as_ptr(),
                    font_size as _,
                    chars.as_ptr() as *mut _,
                    chars.len() as _,
                )
            }),
        }
    }

    /// Load font from Image (XNA style)
    #[inline]
    pub fn from_image(image: &Image, key_color: Color, first_char: char) -> Self {
        Self {
            raw: Rc::new(unsafe {
                ffi::LoadFontFromImage(image.raw.clone(), key_color.into(), first_char as _)
            }),
        }
    }

    /// Load font from memory buffer, fileType refers to extension: i.e. '.ttf'
    #[inline]
    pub fn from_memory(file_type: &str, file_data: &[u8], font_size: u32, chars: &[char]) -> Self {
        let file_type = CString::new(file_type).unwrap();

        Self {
            raw: Rc::new(unsafe {
                ffi::LoadFontFromMemory(
                    file_type.as_ptr(),
                    file_data.as_ptr(),
                    file_data.len() as _,
                    font_size as _,
                    chars.as_ptr() as *mut _,
                    chars.len() as _,
                )
            }),
        }
    }

    /// Check if a font is ready
    #[inline]
    pub fn is_ready(&self) -> bool {
        unsafe { ffi::IsFontReady(self.raw.deref().clone()) }
    }

    /// Export font as code file, returns true on success
    #[inline]
    pub fn export_as_code(&self, filename: &str) -> bool {
        let filename = CString::new(filename).unwrap();

        unsafe { ffi::ExportFontAsCode(self.raw.deref().clone(), filename.as_ptr()) }
    }

    /// Measure string width for default font
    #[inline]
    pub fn measure_text(text: &str, font_size: u32) -> u32 {
        let text = CString::new(text).unwrap();

        unsafe { ffi::MeasureText(text.as_ptr(), font_size as _) as _ }
    }

    /// Measure string size for Font
    #[inline]
    pub fn measure_text_ex(&self, text: &str, font_size: f32, spacing: f32) -> Vector2 {
        let text = CString::new(text).unwrap();

        unsafe {
            ffi::MeasureTextEx(self.raw.deref().clone(), text.as_ptr(), font_size, spacing).into()
        }
    }

    /// Get glyph index position in font for a codepoint (unicode character), fallback to '?' if not found
    #[inline]
    pub fn get_glyph_index(&self, codepoint: char) -> usize {
        unsafe { ffi::GetGlyphIndex(self.raw.deref().clone(), codepoint as _) as _ }
    }

    /// Get glyph rectangle in font atlas for a codepoint (unicode character), fallback to '?' if not found
    #[inline]
    pub fn get_glyph_atlas_rect(&self, codepoint: char) -> Rectangle {
        unsafe { ffi::GetGlyphAtlasRec(self.raw.deref().clone(), codepoint as _).into() }
    }
}

impl Default for Font {
    /// Get the default Font
    #[inline]
    fn default() -> Self {
        Self {
            raw: Rc::new(unsafe { ffi::GetFontDefault() }),
        }
    }
}

impl Drop for Font {
    #[inline]
    fn drop(&mut self) {
        if Rc::strong_count(&self.raw) == 1 {
            unsafe { ffi::UnloadFont(self.raw.deref().clone()) }
        }
    }
}

// /// Load font data for further use
// #[inline]
// pub fn LoadFontData(fileData: *const core::ffi::c_uchar, dataSize: core::ffi::c_int, fontSize: core::ffi::c_int, fontChars: *mut core::ffi::c_int, glyphCount: core::ffi::c_int, r#type: core::ffi::c_int, ) -> *mut GlyphInfo;

// /// Generate image font atlas using chars info
// #[inline]
// pub fn GenImageFontAtlas(chars: *const GlyphInfo, recs: *mut *mut Rectangle, glyphCount: core::ffi::c_int, fontSize: core::ffi::c_int, padding: core::ffi::c_int, packMethod: core::ffi::c_int, ) -> Image;

// /// Unload font chars info data (RAM)
// #[inline]
// pub fn UnloadFontData(chars: *mut GlyphInfo, glyphCount: core::ffi::c_int, );

// /// Get glyph font info data for a codepoint (unicode character), fallback to '?' if not found
// #[inline]
// pub fn get_glyph_info(&self, codepoint: char) -> GlyphInfo {

// }
