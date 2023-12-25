use crate::{
    color::Color,
    ffi,
    math::{Rectangle, Vector2},
    texture::Image,
};
use std::ffi::CString;

pub use crate::ffi::FontType;

/// Font, font texture and GlyphInfo array data
#[derive(Debug)]
#[repr(transparent)]
pub struct Font {
    pub(crate) raw: ffi::Font,
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
    pub fn from_file(file_name: &str) -> Option<Self> {
        let file_name = CString::new(file_name).unwrap();

        let raw = unsafe { ffi::LoadFont(file_name.as_ptr()) };

        if unsafe { ffi::IsFontReady(raw.clone()) } {
            Some(Self { raw })
        } else {
            None
        }
    }

    /// Load font from file with extended parameters
    #[inline]
    pub fn from_file_ex(file_name: &str, font_size: u32, chars: &[char]) -> Option<Self> {
        let file_name = CString::new(file_name).unwrap();

        let raw = unsafe {
            ffi::LoadFontEx(
                file_name.as_ptr(),
                font_size as _,
                chars.as_ptr() as *mut _,
                chars.len() as _,
            )
        };

        if unsafe { ffi::IsFontReady(raw.clone()) } {
            Some(Self { raw })
        } else {
            None
        }
    }

    /// Load font from Image (XNA style)
    #[inline]
    pub fn from_image(image: &Image, key_color: Color, first_char: char) -> Option<Self> {
        let raw =
            unsafe { ffi::LoadFontFromImage(image.raw.clone(), key_color.into(), first_char as _) };

        if unsafe { ffi::IsFontReady(raw.clone()) } {
            Some(Self { raw })
        } else {
            None
        }
    }

    /// Load font from memory buffer, fileType refers to extension: i.e. '.ttf'
    #[inline]
    pub fn from_memory(
        file_type: &str,
        file_data: &[u8],
        font_size: u32,
        chars: &[char],
    ) -> Option<Self> {
        let file_type = CString::new(file_type).unwrap();

        let raw = unsafe {
            ffi::LoadFontFromMemory(
                file_type.as_ptr(),
                file_data.as_ptr(),
                file_data.len() as _,
                font_size as _,
                chars.as_ptr() as *mut _,
                chars.len() as _,
            )
        };

        if unsafe { ffi::IsFontReady(raw.clone()) } {
            Some(Self { raw })
        } else {
            None
        }
    }

    /// Export font as code file, returns true on success
    #[inline]
    pub fn export_as_code(&self, file_name: &str) -> bool {
        let file_name = CString::new(file_name).unwrap();

        unsafe { ffi::ExportFontAsCode(self.raw.clone(), file_name.as_ptr()) }
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

        unsafe { ffi::MeasureTextEx(self.raw.clone(), text.as_ptr(), font_size, spacing).into() }
    }

    /// Get glyph index position in font for a codepoint (unicode character), fallback to '?' if not found
    #[inline]
    pub fn get_glyph_index(&self, codepoint: char) -> usize {
        unsafe { ffi::GetGlyphIndex(self.raw.clone(), codepoint as _) as _ }
    }

    /// Get glyph rectangle in font atlas for a codepoint (unicode character), fallback to '?' if not found
    #[inline]
    pub fn get_glyph_atlas_rect(&self, codepoint: char) -> Rectangle {
        unsafe { ffi::GetGlyphAtlasRec(self.raw.clone(), codepoint as _).into() }
    }

    /// Get glyph font info data for a codepoint (unicode character), fallback to '?' if not found
    #[inline]
    pub fn get_glyph_info(&self, codepoint: char) -> GlyphInfo {
        let info = unsafe { ffi::GetGlyphInfo(self.raw.clone(), codepoint as _) };

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

    /// Get the 'raw' ffi type
    /// Take caution when cloning so it doesn't outlive the original
    #[inline]
    pub fn as_raw(&self) -> &ffi::Font {
        &self.raw
    }

    /// Get the 'raw' ffi type
    /// Take caution when cloning so it doesn't outlive the original
    #[inline]
    pub fn as_raw_mut(&mut self) -> &mut ffi::Font {
        &mut self.raw
    }

    /// Convert a 'raw' ffi object to a safe wrapper
    ///
    /// # Safety
    /// * The raw object must be correctly initialized
    /// * The raw object should be unique. Otherwise, make sure its clones don't outlive the newly created object.
    #[inline]
    pub unsafe fn from_raw(raw: ffi::Font) -> Self {
        Self { raw }
    }
}

impl Default for Font {
    /// Get the default Font
    #[inline]
    fn default() -> Self {
        Self {
            raw: unsafe { ffi::GetFontDefault() },
        }
    }
}

impl Drop for Font {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::UnloadFont(self.raw.clone()) }
    }
}

/// Generate image font atlas using chars info
#[inline]
pub fn gen_image_font_atlas(
    chars: Vec<GlyphInfo>,
    font_size: u32,
    padding: i32,
    skyline_pack: bool,
) -> Option<(Image, Vec<Rectangle>)> {
    assert!(!chars.is_empty());

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

    if !unsafe { ffi::IsImageReady(image.clone()) } {
        return None;
    }

    let mut vec = Vec::new();

    for i in 0..chars.len() {
        vec.push(unsafe { recs.add(i).read().into() });
    }

    unsafe {
        ffi::MemFree(recs as *mut _);
    }

    Some((Image { raw: image }, vec))
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
        assert!(!font_chars.is_empty());
        let len = font_chars.len();

        let infos = unsafe {
            ffi::LoadFontData(
                file_data.as_ptr(),
                file_data.len() as _,
                font_size as _,
                font_chars.as_ptr() as *mut _,
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
