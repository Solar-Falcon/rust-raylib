use crate::{
    core::{Raylib, Rectangle, Vector2, Vector3, Vector4},
    ffi,
    text::Font,
};

use std::{ffi::CString, mem::transmute, ops::Deref, rc::Rc};

use static_assertions::{assert_eq_align, assert_eq_size};

pub use crate::ffi::{CubemapLayout, NPatchLayout, PixelFormat, TextureFilter, TextureWrap};

/// Get pixel data size in bytes for certain format
#[inline]
pub fn get_pixel_data_size(width: u32, height: u32, format: PixelFormat) -> usize {
    unsafe { ffi::GetPixelDataSize(width as _, height as _, format as _) as usize }
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
                    format as _,
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
                    format as _,
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
                    width as _,
                    height as _,
                    format as _,
                    header_size as _,
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

        let image = unsafe { ffi::LoadImageAnim(filename.as_ptr(), (&mut frames) as *mut _) };

        (Self { raw: image }, frames as usize)
    }

    /// Load image from memory buffer, fileType refers to extension: i.e. '.png'
    #[inline]
    pub fn from_memory(filetype: &str, filedata: &[u8]) -> Self {
        let filetype = CString::new(filetype).unwrap();

        Self {
            raw: unsafe {
                ffi::LoadImageFromMemory(filetype.as_ptr(), filedata.as_ptr(), filedata.len() as _)
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

    /// Generate image: plain color
    #[inline]
    pub fn generate_color(width: u32, height: u32, color: Color) -> Self {
        Self {
            raw: unsafe { ffi::GenImageColor(width as _, height as _, transmute(color)) },
        }
    }

    /// Generate image: vertical gradient
    #[inline]
    pub fn generate_gradient_vertical(width: u32, height: u32, top: Color, bottom: Color) -> Self {
        Self {
            raw: unsafe {
                ffi::GenImageGradientV(width as _, height as _, transmute(top), transmute(bottom))
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
                ffi::GenImageGradientH(width as _, height as _, transmute(left), transmute(right))
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
                    transmute(inner),
                    transmute(outer),
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
                    transmute(color1),
                    transmute(color2),
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
            raw: unsafe { ffi::ImageFromImage(image.raw.clone(), transmute(rect)) },
        }
    }

    /// Create an image from text (default font)
    #[inline]
    pub fn text(text: &str, font_size: u32, color: Color) -> Self {
        let text = CString::new(text).unwrap();

        Self {
            raw: unsafe { ffi::ImageText(text.as_ptr(), font_size as _, transmute(color)) },
        }
    }

    /// Create an image from text (custom sprite font)
    #[inline]
    pub fn text_ex(font: &Font, text: &str, font_size: f32, spacing: f32, tint: Color) -> Self {
        let text = CString::new(text).unwrap();

        Self {
            raw: unsafe {
                ffi::ImageTextEx(
                    font.raw.clone(),
                    text.as_ptr(),
                    font_size,
                    spacing,
                    transmute(tint),
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
        unsafe { ffi::ImageToPOT(self.as_mut_ptr(), transmute(fill)) }
    }

    /// Crop an image to a defined rectangle
    #[inline]
    pub fn crop(&mut self, rect: Rectangle) {
        unsafe { ffi::ImageCrop(self.as_mut_ptr(), transmute(rect)) }
    }

    /// Crop image depending on alpha value
    #[inline]
    pub fn alpha_crop(&mut self, threshold: f32) {
        unsafe { ffi::ImageAlphaCrop(self.as_mut_ptr(), threshold) }
    }

    /// Clear alpha channel to desired color
    #[inline]
    pub fn alpha_clear(&mut self, color: Color, threshold: f32) {
        unsafe { ffi::ImageAlphaClear(self.as_mut_ptr(), transmute(color), threshold) }
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
                transmute(fill),
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
        unsafe { ffi::ImageColorTint(self.as_mut_ptr(), transmute(color)) }
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
        unsafe { ffi::ImageColorReplace(self.as_mut_ptr(), transmute(color), transmute(replace)) }
    }

    /// Load color data from image as a Color array (RGBA - 32bit)
    pub fn load_colors(&self) -> Vec<Color> {
        let colors = unsafe { ffi::LoadImageColors(self.raw.clone()) };
        let len = (self.width() * self.height()) as usize;

        let mut vec = Vec::with_capacity(len);

        for i in 0..len {
            unsafe {
                vec.push(transmute(colors.add(i).read()));
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
                vec.push(transmute(palette.add(i).read()));
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
        unsafe { transmute(ffi::GetImageAlphaBorder(self.raw.clone(), threshold)) }
    }

    /// Get image pixel color at (x, y) position
    #[inline]
    pub fn get_color(&self, x: u32, y: u32) -> Color {
        unsafe { transmute(ffi::GetImageColor(self.raw.clone(), x as _, y as _)) }
    }

    /// Clear image background with given color
    #[inline]
    pub fn clear_background(&mut self, color: Color) {
        unsafe { ffi::ImageClearBackground(self.as_mut_ptr(), transmute(color)) }
    }

    /// Draw pixel within an image
    #[inline]
    pub fn draw_pixel(&mut self, x: u32, y: u32, color: Color) {
        unsafe { ffi::ImageDrawPixel(self.as_mut_ptr(), x as _, y as _, transmute(color)) }
    }

    /// Draw pixel within an image (Vector version)
    #[inline]
    pub fn draw_pixel_v(&mut self, pos: Vector2, color: Color) {
        unsafe { ffi::ImageDrawPixelV(self.as_mut_ptr(), transmute(pos), transmute(color)) }
    }

    /// Draw line within an image
    #[inline]
    pub fn draw_line(&mut self, start_x: u32, start_y: u32, end_x: u32, end_y: u32, color: Color) {
        unsafe {
            ffi::ImageDrawLine(
                self.as_mut_ptr(),
                start_x as _,
                start_y as _,
                end_x as _,
                end_y as _,
                transmute(color),
            )
        }
    }

    /// Draw line within an image (Vector version)
    #[inline]
    pub fn draw_line_v(&mut self, start: Vector2, end: Vector2, color: Color) {
        unsafe {
            ffi::ImageDrawLineV(
                self.as_mut_ptr(),
                transmute(start),
                transmute(end),
                transmute(color),
            )
        }
    }

    /// Draw a filled circle within an image
    #[inline]
    pub fn draw_circle(&mut self, center_x: u32, center_y: u32, radius: u32, color: Color) {
        unsafe {
            ffi::ImageDrawCircle(
                self.as_mut_ptr(),
                center_x as _,
                center_y as _,
                radius as _,
                transmute(color),
            )
        }
    }

    /// Draw a filled circle within an image (Vector version)
    #[inline]
    pub fn draw_circle_v(&mut self, center: Vector2, radius: u32, color: Color) {
        unsafe {
            ffi::ImageDrawCircleV(
                self.as_mut_ptr(),
                transmute(center),
                radius as _,
                transmute(color),
            )
        }
    }

    /// Draw circle outline within an image
    #[inline]
    pub fn draw_circle_lines(&mut self, center_x: u32, center_y: u32, radius: u32, color: Color) {
        unsafe {
            ffi::ImageDrawCircleLines(
                self.as_mut_ptr(),
                center_x as _,
                center_y as _,
                radius as _,
                transmute(color),
            )
        }
    }

    /// Draw circle outline within an image (Vector version)
    #[inline]
    pub fn draw_circle_lines_v(&mut self, center: Vector2, radius: u32, color: Color) {
        unsafe {
            ffi::ImageDrawCircleLinesV(
                self.as_mut_ptr(),
                transmute(center),
                radius as _,
                transmute(color),
            )
        }
    }

    /// Draw rectangle within an image
    #[inline]
    pub fn draw_rectangle(&mut self, x: u32, y: u32, width: u32, height: u32, color: Color) {
        unsafe {
            ffi::ImageDrawRectangle(
                self.as_mut_ptr(),
                x as _,
                y as _,
                width as _,
                height as _,
                transmute(color),
            )
        }
    }

    /// Draw rectangle within an image (Vector version)
    #[inline]
    pub fn draw_rectangle_v(&mut self, pos: Vector2, size: Vector2, color: Color) {
        unsafe {
            ffi::ImageDrawRectangleV(
                self.as_mut_ptr(),
                transmute(pos),
                transmute(size),
                transmute(color),
            )
        }
    }

    /// Draw rectangle within an image
    #[inline]
    pub fn draw_rectangle_rect(&mut self, rect: Rectangle, color: Color) {
        unsafe { ffi::ImageDrawRectangleRec(self.as_mut_ptr(), transmute(rect), transmute(color)) }
    }

    /// Draw rectangle lines within an image
    #[inline]
    pub fn draw_rectangle_lines(&mut self, rect: Rectangle, thickness: u32, color: Color) {
        unsafe {
            ffi::ImageDrawRectangleLines(
                self.as_mut_ptr(),
                transmute(rect),
                thickness as _,
                transmute(color),
            )
        }
    }

    /// Draw a source image within a destination image (tint applied to source)
    #[inline]
    pub fn draw(
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
                transmute(source_rect),
                transmute(dest_rect),
                transmute(tint),
            )
        }
    }

    /// Draw text (using default font) within an image (destination)
    #[inline]
    pub fn draw_text(&mut self, text: &str, x: u32, y: u32, font_size: u32, color: Color) {
        let text = CString::new(text).unwrap();

        unsafe {
            ffi::ImageDrawText(
                self.as_mut_ptr(),
                text.as_ptr(),
                x as _,
                y as _,
                font_size as _,
                transmute(color),
            )
        }
    }

    /// Draw text (custom sprite font) within an image (destination)
    #[inline]
    pub fn draw_text_ex(
        &mut self,
        font: &Font,
        text: &str,
        pos: Vector2,
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
                transmute(pos),
                font_size,
                spacing,
                transmute(tint),
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

#[derive(Clone, Debug)]
pub struct Texture {
    pub(crate) raw: Rc<ffi::Texture>,
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
        unsafe { transmute(self.raw.format) }
    }

    /// Load texture from file into GPU memory (VRAM)
    #[inline]
    pub fn from_file(filename: &str) -> Self {
        let filename = CString::new(filename).unwrap();

        Self {
            raw: Rc::new(unsafe { ffi::LoadTexture(filename.as_ptr()) }),
        }
    }

    /// Load texture from image data
    #[inline]
    pub fn from_image(image: &Image) -> Self {
        Self {
            raw: Rc::new(unsafe { ffi::LoadTextureFromImage(image.raw.clone()) }),
        }
    }

    /// Load cubemap from image, multiple image cubemap layouts supported
    #[inline]
    pub fn from_cubemap(image: &Image, layout: CubemapLayout) -> TextureCubemap {
        Self {
            raw: Rc::new(unsafe { ffi::LoadTextureCubemap(image.raw.clone(), layout as _) }),
        }
    }

    /// Check if a texture is ready
    #[inline]
    pub fn is_ready(&self) -> bool {
        unsafe { ffi::IsTextureReady(self.raw.deref().clone()) }
    }

    /// Update GPU texture with new data
    ///
    /// Returns `true` on success, `false` if `pixels` has wrong size (use [`get_pixel_data_size()`])
    #[inline]
    pub fn update(&mut self, pixels: &[u8]) -> bool {
        if pixels.len() == self.get_pixel_data_size() {
            unsafe {
                ffi::UpdateTexture(self.raw.deref().clone(), pixels.as_ptr() as *const _);
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
                ffi::UpdateTextureRec(
                    self.raw.deref().clone(),
                    transmute(rect),
                    pixels.as_ptr() as *const _,
                );
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
    pub fn generate_mipmaps(&mut self) -> bool {
        if let Some(texture) = Rc::get_mut(&mut self.raw) {
            unsafe {
                ffi::GenTextureMipmaps(texture as *mut _);
            }
            true
        } else {
            false
        }
    }

    /// Set texture scaling filter mode
    #[inline]
    pub fn set_filter(&mut self, filter: TextureFilter) {
        unsafe { ffi::SetTextureFilter(self.raw.deref().clone(), filter as _) }
    }

    /// Set texture wrapping mode
    #[inline]
    pub fn set_wrap(&mut self, wrap: TextureWrap) {
        unsafe { ffi::SetTextureWrap(self.raw.deref().clone(), wrap as _) }
    }

    /// Draw a Texture2D
    #[inline]
    pub fn draw(&self, _raylib: &mut Raylib, x: i32, y: i32, tint: Color) {
        unsafe { ffi::DrawTexture(self.raw.deref().clone(), x, y, transmute(tint)) }
    }

    /// Draw a Texture2D with position defined as Vector2
    #[inline]
    pub fn draw_v(&self, _raylib: &mut Raylib, pos: Vector2, tint: Color) {
        unsafe { ffi::DrawTextureV(self.raw.deref().clone(), transmute(pos), transmute(tint)) }
    }

    /// Draw a Texture2D with extended parameters
    #[inline]
    pub fn draw_ex(
        &self,
        _raylib: &mut Raylib,
        pos: Vector2,
        rotation: f32,
        scale: f32,
        tint: Color,
    ) {
        unsafe {
            ffi::DrawTextureEx(
                self.raw.deref().clone(),
                transmute(pos),
                rotation,
                scale,
                transmute(tint),
            )
        }
    }

    /// Draw a part of a texture defined by a rectangle
    #[inline]
    pub fn draw_rect(&self, _raylib: &mut Raylib, source: Rectangle, pos: Vector2, tint: Color) {
        // rectangle checks?
        unsafe {
            ffi::DrawTextureRec(
                self.raw.deref().clone(),
                transmute(source),
                transmute(pos),
                transmute(tint),
            )
        }
    }

    /// Draw a part of a texture defined by a rectangle with 'pro' parameters
    #[inline]
    pub fn draw_pro(
        &self,
        source: Rectangle,
        dest: Rectangle,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    ) {
        // rectangle checks?
        unsafe {
            ffi::DrawTexturePro(
                self.raw.deref().clone(),
                transmute(source),
                transmute(dest),
                transmute(origin),
                rotation,
                transmute(tint),
            )
        }
    }

    /// Draws a texture (or part of it) that stretches or shrinks nicely
    #[inline]
    pub fn draw_patch(
        &self,
        _raylib: &mut Raylib,
        patch_info: NPatchInfo,
        dest: Rectangle,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    ) {
        unsafe {
            ffi::DrawTextureNPatch(
                self.raw.deref().clone(),
                transmute(patch_info),
                transmute(dest),
                transmute(origin),
                rotation,
                transmute(tint),
            )
        }
    }
}

impl Drop for Texture {
    #[inline]
    fn drop(&mut self) {
        if let Some(raw) = Rc::get_mut(&mut self.raw) {
            unsafe { ffi::UnloadTexture(raw.clone()) }
        }
    }
}

#[derive(Clone, Debug)]
pub struct RenderTexture {
    pub(crate) raw: Rc<ffi::RenderTexture>,
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
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            raw: Rc::new(unsafe { ffi::LoadRenderTexture(width as _, height as _) }),
        }
    }

    /// Check if a render texture is ready
    #[inline]
    pub fn is_ready(&self) -> bool {
        unsafe { ffi::IsRenderTextureReady(self.raw.deref().clone()) }
    }
}

impl Drop for RenderTexture {
    #[inline]
    fn drop(&mut self) {
        if let Some(raw) = Rc::get_mut(&mut self.raw) {
            unsafe { ffi::UnloadRenderTexture(raw.clone()) }
        }
    }
}

/// Texture2D, same as Texture
pub type Texture2D = Texture;

/// TextureCubemap, same as Texture
pub type TextureCubemap = Texture;

/// RenderTexture2D, same as RenderTexture
pub type RenderTexture2D = RenderTexture;
