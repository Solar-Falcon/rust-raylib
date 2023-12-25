use crate::{
    color::Color,
    core::Raylib,
    ffi,
    math::{Rectangle, Vector2},
    text::Font,
};

use std::ffi::{CStr, CString};

use static_assertions::{assert_eq_align, assert_eq_size};

pub use crate::ffi::{CubemapLayout, NPatchLayout, PixelFormat, TextureFilter, TextureWrap};

/// Get pixel data size in bytes for certain format
#[inline]
pub fn get_pixel_data_size(width: u32, height: u32, format: PixelFormat) -> usize {
    unsafe { ffi::GetPixelDataSize(width as _, height as _, format as _) as usize }
}

/// Image file format
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageFormat {
    /// PNG
    Png,
    /// BMP
    Bmp,
    /// TGA
    Tga,
    /// JPEG
    Jpg,
    /// GIF
    Gif,
    /// PIC
    Pic,
    /// PNM
    Pnm,
    /// PSD
    Psd,
    /// HDR
    Hdr,
    /// QOI
    Qoi,
    /// DDS
    Dds,
    /// PKM
    Pkm,
    /// KTX
    Ktx,
    /// PVR
    Pvr,
    /// ASTC
    Astc,
}

impl ImageFormat {
    fn as_cstr(&self) -> &'static CStr {
        use ImageFormat::*;

        CStr::from_bytes_with_nul(match self {
            Png => b".png\0",
            Bmp => b".bmp\0",
            Tga => b".tga\0",
            Jpg => b".jpg\0",
            Gif => b".gif\0",
            Pic => b".pic\0",
            Pnm => b".ppm\0",
            Psd => b".psd\0",
            Hdr => b".hdr\0",
            Qoi => b".qoi\0",
            Dds => b".dds\0",
            Pkm => b".pkm\0",
            Ktx => b".ktx\0",
            Pvr => b".pvr\0",
            Astc => b".astc\0",
        })
        .unwrap()
    }
}

/// NPatchInfo, n-patch layout info
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct NPatchInfo {
    /// Texture source rectangle
    pub source: Rectangle,
    /// Left border offset
    pub left: i32,
    /// Top border offset
    pub top: i32,
    /// Right border offset
    pub right: i32,
    /// Bottom border offset
    pub bottom: i32,
    /// Layout of the n-patch: 3x3, 1x3 or 3x1
    pub layout: NPatchLayout,
}

assert_eq_size!(NPatchInfo, ffi::NPatchInfo);
assert_eq_align!(NPatchInfo, ffi::NPatchInfo);

impl From<NPatchInfo> for ffi::NPatchInfo {
    #[inline]
    fn from(val: NPatchInfo) -> Self {
        unsafe { std::mem::transmute(val) }
    }
}

impl From<ffi::NPatchInfo> for NPatchInfo {
    #[inline]
    fn from(value: ffi::NPatchInfo) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

/// Image, pixel data stored in CPU memory (RAM)
#[derive(Debug)]
#[repr(transparent)]
pub struct Image {
    pub(crate) raw: ffi::Image,
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
        unsafe { std::mem::transmute(self.raw.format) }
    }

    /// Load image from file into CPU memory (RAM)
    #[inline]
    pub fn from_file(file_name: &str) -> Option<Self> {
        let file_name = CString::new(file_name).unwrap();

        let raw = unsafe { ffi::LoadImage(file_name.as_ptr()) };

        if unsafe { ffi::IsImageReady(raw.clone()) } {
            Some(Self { raw })
        } else {
            None
        }
    }

    /// Load image from RAW file data
    #[inline]
    pub fn from_raw_file(
        file_name: &str,
        width: u32,
        height: u32,
        format: PixelFormat,
        header_size: u32,
    ) -> Option<Self> {
        let file_name = CString::new(file_name).unwrap();

        let raw = unsafe {
            ffi::LoadImageRaw(
                file_name.as_ptr(),
                width as _,
                height as _,
                format as _,
                header_size as _,
            )
        };

        if unsafe { ffi::IsImageReady(raw.clone()) } {
            Some(Self { raw })
        } else {
            None
        }
    }

    /// Load image sequence from file (frames appended to image.data)
    ///
    /// Returns the amount of frames in the image.
    #[inline]
    pub fn from_file_anim(file_name: &str) -> Option<(Self, usize)> {
        let file_name = CString::new(file_name).unwrap();
        let mut frames: i32 = 0;

        let image = unsafe { ffi::LoadImageAnim(file_name.as_ptr(), (&mut frames) as *mut _) };

        if unsafe { ffi::IsImageReady(image.clone()) } {
            Some((Self { raw: image }, frames as _))
        } else {
            None
        }
    }

    /// Load image from memory buffer
    ///
    /// If `format` is None, it will make an educated guess on the ImageFormat (not all formats are supported for guessing).
    #[inline]
    pub fn from_memory(file_data: &[u8], format: Option<ImageFormat>) -> Option<Self> {
        let raw = unsafe {
            let format = if let Some(format) = format {
                format.as_cstr().as_ptr()
            } else {
                CStr::from_bytes_with_nul(b".png\0").unwrap().as_ptr()
            };

            ffi::LoadImageFromMemory(format, file_data.as_ptr(), file_data.len() as _)
        };

        if unsafe { ffi::IsImageReady(raw.clone()) } {
            Some(Self { raw })
        } else {
            None
        }
    }

    /// Load image from GPU texture data
    #[inline]
    pub fn from_texture(texture: &Texture) -> Option<Self> {
        let raw = unsafe { ffi::LoadImageFromTexture(texture.raw.clone()) };

        if unsafe { ffi::IsImageReady(raw.clone()) } {
            Some(Self { raw })
        } else {
            None
        }
    }

    /// Load image from screen buffer and (screenshot)
    #[inline]
    pub fn from_screen(_raylib: &Raylib) -> Option<Self> {
        let raw = unsafe { ffi::LoadImageFromScreen() };

        if unsafe { ffi::IsImageReady(raw.clone()) } {
            Some(Self { raw })
        } else {
            None
        }
    }

    /// Export image data to file, returns true on success
    #[inline]
    pub fn export(&self, file_name: &str) -> bool {
        let file_name = CString::new(file_name).unwrap();

        unsafe { ffi::ExportImage(self.raw.clone(), file_name.as_ptr()) }
    }

    /// Export image as code file defining an array of bytes, returns true on success
    #[inline]
    pub fn export_as_code(&self, file_name: &str) -> bool {
        let file_name = CString::new(file_name).unwrap();

        unsafe { ffi::ExportImageAsCode(self.raw.clone(), file_name.as_ptr()) }
    }

    /// Generate image: plain color
    #[inline]
    pub fn generate_color(width: u32, height: u32, color: Color) -> Self {
        Self {
            raw: unsafe { ffi::GenImageColor(width as _, height as _, color.into()) },
        }
    }

    /// Generate image: vertical gradient
    #[inline]
    pub fn generate_gradient_vertical(width: u32, height: u32, top: Color, bottom: Color) -> Self {
        Self {
            raw: unsafe {
                ffi::GenImageGradientV(width as _, height as _, top.into(), bottom.into())
            },
        }
    }

    /// Generate image: horizontal gradient
    #[inline]
    pub fn generate_gradient_horizontal(
        width: u32,
        height: u32,
        left: Color,
        right: Color,
    ) -> Self {
        Self {
            raw: unsafe {
                ffi::GenImageGradientH(width as _, height as _, left.into(), right.into())
            },
        }
    }

    /// Generate image: radial gradient
    #[inline]
    pub fn generate_gradient_radial(
        width: u32,
        height: u32,
        density: f32,
        inner: Color,
        outer: Color,
    ) -> Self {
        Self {
            raw: unsafe {
                ffi::GenImageGradientRadial(
                    width as _,
                    height as _,
                    density,
                    inner.into(),
                    outer.into(),
                )
            },
        }
    }

    /// Generate image: checked
    #[inline]
    pub fn generate_checked(
        width: u32,
        height: u32,
        checks_x: u32,
        checks_y: u32,
        color1: Color,
        color2: Color,
    ) -> Self {
        Self {
            raw: unsafe {
                ffi::GenImageChecked(
                    width as _,
                    height as _,
                    checks_x as _,
                    checks_y as _,
                    color1.into(),
                    color2.into(),
                )
            },
        }
    }

    /// Generate image: white noise
    #[inline]
    pub fn generate_white_noise(width: u32, height: u32, factor: f32) -> Self {
        Self {
            raw: unsafe { ffi::GenImageWhiteNoise(width as _, height as _, factor) },
        }
    }

    /// Generate image: perlin noise
    #[inline]
    pub fn generate_perlin_noise(
        width: u32,
        height: u32,
        offset_x: i32,
        offset_y: i32,
        scale: f32,
    ) -> Self {
        Self {
            raw: unsafe {
                ffi::GenImagePerlinNoise(width as _, height as _, offset_x, offset_y, scale)
            },
        }
    }

    /// Generate image: cellular algorithm, bigger tileSize means bigger cells
    #[inline]
    pub fn generate_cellular(width: u32, height: u32, tile_size: u32) -> Self {
        Self {
            raw: unsafe { ffi::GenImageCellular(width as _, height as _, tile_size as _) },
        }
    }

    /// Generate image: grayscale image from text data
    #[inline]
    pub fn generate_text(width: u32, height: u32, text: &str) -> Self {
        let text = CString::new(text).unwrap();

        Self {
            raw: unsafe { ffi::GenImageText(width as _, height as _, text.as_ptr()) },
        }
    }

    /// Create an image from another image piece
    #[inline]
    pub fn from_other_image(image: Self, rect: Rectangle) -> Self {
        Self {
            raw: unsafe { ffi::ImageFromImage(image.raw.clone(), rect.into()) },
        }
    }

    /// Create an image from text (default font)
    #[inline]
    pub fn text(text: &str, font_size: u32, color: Color) -> Self {
        let text = CString::new(text).unwrap();

        Self {
            raw: unsafe { ffi::ImageText(text.as_ptr(), font_size as _, color.into()) },
        }
    }

    /// Create an image from text (custom sprite font)
    #[inline]
    pub fn text_with_font(
        text: &str,
        font: &Font,
        font_size: f32,
        spacing: f32,
        tint: Color,
    ) -> Self {
        let text = CString::new(text).unwrap();

        Self {
            raw: unsafe {
                ffi::ImageTextEx(
                    font.raw.clone(),
                    text.as_ptr(),
                    font_size,
                    spacing,
                    tint.into(),
                )
            },
        }
    }

    /// Convert image data to desired format
    #[inline]
    pub fn convert_to_format(&mut self, new_format: PixelFormat) {
        unsafe { ffi::ImageFormat(self.as_mut_ptr(), new_format as _) }
    }

    /// Convert image to POT (power-of-two)
    #[inline]
    pub fn convert_to_power_of_two(&mut self, fill: Color) {
        unsafe { ffi::ImageToPOT(self.as_mut_ptr(), fill.into()) }
    }

    /// Crop an image to a defined rectangle
    #[inline]
    pub fn crop(&mut self, rect: Rectangle) {
        unsafe { ffi::ImageCrop(self.as_mut_ptr(), rect.into()) }
    }

    /// Crop image depending on alpha value
    #[inline]
    pub fn alpha_crop(&mut self, threshold: f32) {
        unsafe { ffi::ImageAlphaCrop(self.as_mut_ptr(), threshold) }
    }

    /// Clear alpha channel to desired color
    #[inline]
    pub fn alpha_clear(&mut self, color: Color, threshold: f32) {
        unsafe { ffi::ImageAlphaClear(self.as_mut_ptr(), color.into(), threshold) }
    }

    /// Apply alpha mask to image
    #[inline]
    pub fn alpha_mask(&mut self, alpha_mask: &Image) {
        unsafe { ffi::ImageAlphaMask(self.as_mut_ptr(), alpha_mask.raw.clone()) }
    }

    /// Premultiply alpha channel
    #[inline]
    pub fn alpha_premultiply(&mut self) {
        unsafe { ffi::ImageAlphaPremultiply(self.as_mut_ptr()) }
    }

    /// Apply Gaussian blur using a box blur approximation
    #[inline]
    pub fn blur_gaussian(&mut self, blur_size: u32) {
        unsafe { ffi::ImageBlurGaussian(self.as_mut_ptr(), blur_size as _) }
    }

    /// Resize image (Bicubic scaling algorithm)
    #[inline]
    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        unsafe { ffi::ImageResize(self.as_mut_ptr(), new_width as _, new_height as _) }
    }

    /// Resize image (Nearest-Neighbor scaling algorithm)
    #[inline]
    pub fn resize_nn(&mut self, new_width: u32, new_height: u32) {
        unsafe { ffi::ImageResizeNN(self.as_mut_ptr(), new_width as _, new_height as _) }
    }

    /// Resize canvas and fill with color
    #[inline]
    pub fn resize_canvas(
        &mut self,
        new_width: u32,
        new_height: u32,
        offset_x: i32,
        offset_y: i32,
        fill: Color,
    ) {
        unsafe {
            ffi::ImageResizeCanvas(
                self.as_mut_ptr(),
                new_width as _,
                new_height as _,
                offset_x,
                offset_y,
                fill.into(),
            )
        }
    }

    /// Compute all mipmap levels for a provided image
    #[inline]
    pub fn compute_mipmaps(&mut self) {
        unsafe { ffi::ImageMipmaps(self.as_mut_ptr()) }
    }

    /// Dither image data to 16bpp or lower (Floyd-Steinberg dithering)
    #[inline]
    pub fn dither(&mut self, r_bpp: u32, g_bpp: u32, b_bpp: u32, a_bpp: u32) {
        unsafe {
            ffi::ImageDither(
                self.as_mut_ptr(),
                r_bpp as _,
                g_bpp as _,
                b_bpp as _,
                a_bpp as _,
            )
        }
    }

    /// Flip image vertically
    #[inline]
    pub fn flip_vertical(&mut self) {
        unsafe { ffi::ImageFlipVertical(self.as_mut_ptr()) }
    }

    /// Flip image horizontally
    #[inline]
    pub fn flip_horizontal(&mut self) {
        unsafe { ffi::ImageFlipHorizontal(self.as_mut_ptr()) }
    }

    /// Rotate image clockwise 90deg
    #[inline]
    pub fn rotate_clockwise(&mut self) {
        unsafe { ffi::ImageRotateCW(self.as_mut_ptr()) }
    }

    /// Rotate image counter-clockwise 90deg
    #[inline]
    pub fn rotate_counter_clockwise(&mut self) {
        unsafe { ffi::ImageRotateCCW(self.as_mut_ptr()) }
    }

    /// Modify image color: tint
    #[inline]
    pub fn color_tint(&mut self, color: Color) {
        unsafe { ffi::ImageColorTint(self.as_mut_ptr(), color.into()) }
    }

    /// Modify image color: invert
    #[inline]
    pub fn color_invert(&mut self) {
        unsafe { ffi::ImageColorInvert(self.as_mut_ptr()) }
    }

    /// Modify image color: grayscale
    #[inline]
    pub fn color_grayscale(&mut self) {
        unsafe { ffi::ImageColorGrayscale(self.as_mut_ptr()) }
    }

    /// Modify image color: contrast (-100 to 100)
    #[inline]
    pub fn color_contrast(&mut self, contrast: f32) {
        unsafe { ffi::ImageColorContrast(self.as_mut_ptr(), contrast) }
    }

    /// Modify image color: brightness (-255 to 255)
    #[inline]
    pub fn color_brightness(&mut self, brightness: i32) {
        unsafe { ffi::ImageColorBrightness(self.as_mut_ptr(), brightness) }
    }

    /// Modify image color: replace color
    #[inline]
    pub fn color_replace(&mut self, color: Color, replace: Color) {
        unsafe { ffi::ImageColorReplace(self.as_mut_ptr(), color.into(), replace.into()) }
    }

    /// Load color data from image as a Color array (RGBA - 32bit)
    pub fn load_colors(&self) -> Vec<Color> {
        let colors = unsafe { ffi::LoadImageColors(self.raw.clone()) };
        let len = (self.width() * self.height()) as usize;

        let mut vec = Vec::with_capacity(len);

        for i in 0..len {
            unsafe {
                vec.push(colors.add(i).read().into());
            }
        }

        unsafe {
            ffi::UnloadImageColors(colors);
        }

        vec
    }

    /// Load colors palette from image as a Color array (RGBA - 32bit)
    pub fn load_palette(&self, max_size: usize) -> Vec<Color> {
        let mut count: i32 = 0;
        let palette = unsafe {
            ffi::LoadImagePalette(self.raw.clone(), max_size as _, (&mut count) as *mut _)
        };

        let mut vec = Vec::with_capacity(count as usize);

        for i in 0..(count as usize) {
            unsafe {
                vec.push(palette.add(i).read().into());
            }
        }

        unsafe {
            ffi::UnloadImagePalette(palette);
        }

        vec
    }

    /// Get image alpha border rectangle
    #[inline]
    pub fn get_alpha_border(&self, threshold: f32) -> Rectangle {
        unsafe { ffi::GetImageAlphaBorder(self.raw.clone(), threshold).into() }
    }

    /// Get image pixel color at (x, y) position
    #[inline]
    pub fn get_color(&self, x: u32, y: u32) -> Color {
        unsafe { ffi::GetImageColor(self.raw.clone(), x as _, y as _).into() }
    }

    /// Clear image background with given color
    #[inline]
    pub fn clear_background(&mut self, color: Color) {
        unsafe { ffi::ImageClearBackground(self.as_mut_ptr(), color.into()) }
    }

    /// Draw pixel within an image
    #[inline]
    pub fn draw_pixel(&mut self, pos: Vector2, color: Color) {
        unsafe { ffi::ImageDrawPixelV(self.as_mut_ptr(), pos.into(), color.into()) }
    }

    /// Draw line within an image
    #[inline]
    pub fn draw_line(&mut self, start: Vector2, end: Vector2, color: Color) {
        unsafe { ffi::ImageDrawLineV(self.as_mut_ptr(), start.into(), end.into(), color.into()) }
    }

    /// Draw a filled circle within an image
    #[inline]
    pub fn draw_circle(&mut self, center: Vector2, radius: u32, color: Color) {
        unsafe {
            ffi::ImageDrawCircleV(self.as_mut_ptr(), center.into(), radius as _, color.into())
        }
    }

    /// Draw circle outline within an image
    #[inline]
    pub fn draw_circle_lines_v(&mut self, center: Vector2, radius: u32, color: Color) {
        unsafe {
            ffi::ImageDrawCircleLinesV(self.as_mut_ptr(), center.into(), radius as _, color.into())
        }
    }

    /// Draw rectangle within an image
    #[inline]
    pub fn draw_rectangle(&mut self, rect: Rectangle, color: Color) {
        unsafe { ffi::ImageDrawRectangleRec(self.as_mut_ptr(), rect.into(), color.into()) }
    }

    /// Draw rectangle lines within an image
    #[inline]
    pub fn draw_rectangle_lines(&mut self, rect: Rectangle, thickness: u32, color: Color) {
        unsafe {
            ffi::ImageDrawRectangleLines(
                self.as_mut_ptr(),
                rect.into(),
                thickness as _,
                color.into(),
            )
        }
    }

    /// Draw a source image within a destination image (tint applied to source)
    #[inline]
    pub fn draw_image(
        &mut self,
        source: &Image,
        source_rect: Rectangle,
        dest_rect: Rectangle,
        tint: Color,
    ) {
        unsafe {
            ffi::ImageDraw(
                self.as_mut_ptr(),
                source.raw.clone(),
                source_rect.into(),
                dest_rect.into(),
                tint.into(),
            )
        }
    }

    /// Draw text (using default font) within an image (destination)
    #[inline]
    pub fn draw_text(&mut self, text: &str, position: Vector2, font_size: u32, color: Color) {
        let text = CString::new(text).unwrap();

        unsafe {
            ffi::ImageDrawText(
                self.as_mut_ptr(),
                text.as_ptr(),
                position.x as _,
                position.y as _,
                font_size as _,
                color.into(),
            )
        }
    }

    /// Draw text (custom sprite font) within an image (destination)
    #[inline]
    pub fn draw_text_with_font(
        &mut self,
        text: &str,
        pos: Vector2,
        font: &Font,
        font_size: f32,
        spacing: f32,
        tint: Color,
    ) {
        let text = CString::new(text).unwrap();

        unsafe {
            ffi::ImageDrawTextEx(
                self.as_mut_ptr(),
                font.raw.clone(),
                text.as_ptr(),
                pos.into(),
                font_size,
                spacing,
                tint.into(),
            )
        }
    }

    /// Get pixel data size in bytes for this image
    #[inline]
    pub fn get_pixel_data_size(&self) -> usize {
        unsafe { ffi::GetPixelDataSize(self.raw.width, self.raw.height, self.raw.format) as usize }
    }

    /// Returns a rectangle with x = 0, y = 0; width and height correspond to image's dimensions
    #[inline]
    pub fn rectangle(&self) -> Rectangle {
        Rectangle::new(0., 0., self.raw.width as f32, self.raw.height as f32)
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut ffi::Image {
        (&mut self.raw) as *mut ffi::Image
    }

    /// Get the 'raw' ffi type
    /// Take caution when cloning so it doesn't outlive the original
    #[inline]
    pub fn as_raw(&self) -> &ffi::Image {
        &self.raw
    }

    /// Get the 'raw' ffi type
    /// Take caution when cloning so it doesn't outlive the original
    #[inline]
    pub fn as_raw_mut(&mut self) -> &mut ffi::Image {
        &mut self.raw
    }

    /// Convert a 'raw' ffi object to a safe wrapper
    ///
    /// # Safety
    /// * The raw object must be correctly initialized
    /// * The raw object should be unique. Otherwise, make sure its clones don't outlive the newly created object.
    #[inline]
    pub unsafe fn from_raw(raw: ffi::Image) -> Self {
        Self { raw }
    }
}

impl Clone for Image {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            raw: unsafe { ffi::ImageCopy(self.raw.clone()) },
        }
    }
}

impl Drop for Image {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::UnloadImage(self.raw.clone()) }
    }
}

/// Texture, tex data stored in GPU memory (VRAM)
#[derive(Debug)]
#[repr(transparent)]
pub struct Texture {
    pub(crate) raw: ffi::Texture,
}

impl Texture {
    /// Texture base width
    #[inline]
    pub fn width(&self) -> u32 {
        self.raw.width as u32
    }

    /// Texture base height
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
        unsafe { std::mem::transmute(self.raw.format) }
    }

    /// Load texture from file into GPU memory (VRAM)
    #[inline]
    pub fn from_file(file_name: &str) -> Option<Self> {
        let file_name = CString::new(file_name).unwrap();

        let raw = unsafe { ffi::LoadTexture(file_name.as_ptr()) };

        if unsafe { ffi::IsTextureReady(raw.clone()) } {
            Some(Self { raw })
        } else {
            None
        }
    }

    /// Load texture from image data
    #[inline]
    pub fn from_image(image: &Image) -> Option<Self> {
        let raw = unsafe { ffi::LoadTextureFromImage(image.raw.clone()) };

        if unsafe { ffi::IsTextureReady(raw.clone()) } {
            Some(Self { raw })
        } else {
            None
        }
    }

    /// Load cubemap from image, multiple image cubemap layouts supported
    #[inline]
    pub fn from_cubemap(image: &Image, layout: CubemapLayout) -> Option<TextureCubemap> {
        let raw = unsafe { ffi::LoadTextureCubemap(image.raw.clone(), layout as _) };

        if unsafe { ffi::IsTextureReady(raw.clone()) } {
            Some(Self { raw })
        } else {
            None
        }
    }

    /// Update GPU texture with new data
    ///
    /// Returns `true` on success, `false` if `pixels` has wrong size (use [`get_pixel_data_size()`])
    #[inline]
    pub fn update(&mut self, pixels: &[u8]) -> bool {
        if pixels.len() == self.get_pixel_data_size() {
            unsafe {
                ffi::UpdateTexture(self.raw.clone(), pixels.as_ptr() as *const _);
            }
            true
        } else {
            false
        }
    }

    /// Update GPU texture rectangle with new data
    ///
    /// Returns `true` on success, `false` if `pixels` has wrong size or `rect` goes out of bounds
    #[inline]
    pub fn update_rect(&mut self, rect: Rectangle, pixels: &[u8]) -> bool {
        if pixels.len() == get_pixel_data_size(rect.width as u32, rect.height as u32, self.format())
            && rect.x >= 0.
            && rect.y >= 0.
            && ((rect.x + rect.width) as u32) < self.width()
            && ((rect.y + rect.height) as u32) < self.height()
        {
            unsafe {
                ffi::UpdateTextureRec(self.raw.clone(), rect.into(), pixels.as_ptr() as *const _);
            }
            true
        } else {
            false
        }
    }

    /// Get pixel data size in bytes for this texture
    #[inline]
    pub fn get_pixel_data_size(&self) -> usize {
        get_pixel_data_size(self.width(), self.height(), self.format())
    }

    /// Generate GPU mipmaps for a texture
    #[inline]
    pub fn generate_mipmaps(&mut self) {
        unsafe {
            ffi::GenTextureMipmaps(&mut self.raw as *mut _);
        }
    }

    /// Set texture scaling filter mode
    #[inline]
    pub fn set_filter(&mut self, filter: TextureFilter) {
        unsafe { ffi::SetTextureFilter(self.raw.clone(), filter as _) }
    }

    /// Set texture wrapping mode
    #[inline]
    pub fn set_wrap(&mut self, wrap: TextureWrap) {
        unsafe { ffi::SetTextureWrap(self.raw.clone(), wrap as _) }
    }

    /// Get the 'raw' ffi type
    /// Take caution when cloning so it doesn't outlive the original
    #[inline]
    pub fn as_raw(&self) -> &ffi::Texture {
        &self.raw
    }

    /// Get the 'raw' ffi type
    /// Take caution when cloning so it doesn't outlive the original
    #[inline]
    pub fn as_raw_mut(&mut self) -> &mut ffi::Texture {
        &mut self.raw
    }

    /// Convert a 'raw' ffi object to a safe wrapper
    ///
    /// # Safety
    /// * The raw object must be correctly initialized
    /// * The raw object should be unique. Otherwise, make sure its clones don't outlive the newly created object.
    #[inline]
    pub unsafe fn from_raw(raw: ffi::Texture) -> Self {
        Self { raw }
    }
}

impl Drop for Texture {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::UnloadTexture(self.raw.clone()) }
    }
}

/// RenderTexture, fbo for texture rendering
#[derive(Debug)]
#[repr(transparent)]
pub struct RenderTexture {
    pub(crate) raw: ffi::RenderTexture,
}

impl RenderTexture {
    /// Texture base width
    #[inline]
    pub fn width(&self) -> u32 {
        self.raw.texture.width as u32
    }

    /// Texture base height
    #[inline]
    pub fn height(&self) -> u32 {
        self.raw.texture.height as u32
    }

    /// Load texture for rendering (framebuffer)
    #[inline]
    pub fn new(width: u32, height: u32) -> Option<Self> {
        let raw = unsafe { ffi::LoadRenderTexture(width as _, height as _) };

        if unsafe { ffi::IsRenderTextureReady(raw.clone()) } {
            Some(Self { raw })
        } else {
            None
        }
    }

    /// Get the 'raw' ffi type
    /// Take caution when cloning so it doesn't outlive the original
    #[inline]
    pub fn as_raw(&self) -> &ffi::RenderTexture {
        &self.raw
    }

    /// Get the 'raw' ffi type
    /// Take caution when cloning so it doesn't outlive the original
    #[inline]
    pub fn as_raw_mut(&mut self) -> &mut ffi::RenderTexture {
        &mut self.raw
    }

    /// Convert a 'raw' ffi object to a safe wrapper
    ///
    /// # Safety
    /// * The raw object must be correctly initialized
    /// * The raw object should be unique. Otherwise, make sure its clones don't outlive the newly created object.
    #[inline]
    pub unsafe fn from_raw(raw: ffi::RenderTexture) -> Self {
        Self { raw }
    }
}

impl Drop for RenderTexture {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::UnloadRenderTexture(self.raw.clone()) }
    }
}

/// Texture2D, same as Texture
pub type Texture2D = Texture;

/// TextureCubemap, same as Texture
pub type TextureCubemap = Texture;

/// RenderTexture2D, same as RenderTexture
pub type RenderTexture2D = RenderTexture;
