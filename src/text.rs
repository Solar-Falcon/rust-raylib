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

    /// Get glyph font info data for a codepoint (unicode character), fallback to '?' if not found
    #[inline]
    pub fn get_glyph_info(&self, codepoint: char) -> GlyphInfo {
        let info = unsafe { ffi::GetGlyphInfo(self.raw.deref().clone(), codepoint as _) };

        GlyphInfo {
            value: char::from_u32(info.value as _).unwrap(),
            offset_x: info.offsetX,
            offset_y: info.offsetY,
            advance_x: info.advanceX,
            image: Image {
                raw: unsafe { ffi::ImageCopy(info.image) },
            },
        }
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

/// Generate image font atlas using chars info
#[inline]
pub fn gen_image_font_atlas(
    chars: Vec<GlyphInfo>,
    font_size: u32,
    padding: i32,
    skyline_pack: bool,
) -> (Image, Vec<Rectangle>) {
    let mut recs: *mut ffi::Rectangle = std::ptr::null_mut();
    let chars_ffi: Vec<_> = chars
        .iter()
        .map(|gi| ffi::GlyphInfo {
            value: gi.value as _,
            offsetX: gi.offset_x as _,
            offsetY: gi.offset_y as _,
            advanceX: gi.advance_x as _,
            image: gi.image.raw.clone(),
        })
        .collect();

    let image = unsafe {
        ffi::GenImageFontAtlas(
            chars_ffi.as_ptr(),
            (&mut recs) as *mut _,
            chars.len() as _,
            font_size as _,
            padding,
            if skyline_pack { 1 } else { 0 },
        )
    };

    let mut vec = Vec::new();

    for i in 0..chars.len() {
        vec.push(unsafe { recs.add(i).read().into() });
    }

    unsafe { ffi::MemFree(recs as *mut _); }

    (Image { raw: image }, vec)
}

/// GlyphInfo, font characters glyphs info
#[repr(C)]
#[derive(Clone, Debug)]
pub struct GlyphInfo {
    /// Character value (Unicode)
    pub value: char,
    /// Character offset X when drawing
    pub offset_x: i32,
    /// Character offset Y when drawing
    pub offset_y: i32,
    /// Character advance position X
    pub advance_x: i32,
    /// Character image data
    pub image: Image,
}

impl GlyphInfo {
    /// Load font data for further use
    #[inline]
    pub fn from_file_data(
        file_data: &[u8],
        font_size: u32,
        font_chars: &[char],
        font_type: FontType,
    ) -> Vec<GlyphInfo> {
        let len = font_chars.len();

        let infos = unsafe {
            ffi::LoadFontData(
                file_data.as_ptr(),
                file_data.len() as _,
                font_size as _,
                if len != 0 {
                    font_chars.as_ptr()
                } else {
                    std::ptr::null()
                } as *mut _,
                len as _,
                font_type as _,
            )
        };

        let mut vec = Vec::new();

        for i in 0..len {
            let gi = unsafe { infos.add(i).read() };

            vec.push(GlyphInfo {
                value: char::from_u32(gi.value as _).unwrap(),
                offset_x: gi.offsetX,
                offset_y: gi.offsetY,
                advance_x: gi.advanceX,
                image: Image {
                    raw: unsafe { ffi::ImageCopy(gi.image) },
                },
            });
        }

        unsafe {
            ffi::UnloadFontData(infos, len as _);
        }

        vec
    }
}
