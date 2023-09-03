use crate::{
    ffi,
    math::{Camera2D, Camera3D},
    shader::Shader,
    texture::RenderTexture2D,
    vr::VrStereoConfig,
};

/*
/// Draw a Texture2D
#[inline]
pub fn draw(&self, _raylib: &mut Raylib, x: i32, y: i32, tint: Color) {
    unsafe { ffi::DrawTexture(self.raw.deref().clone(), x, y, tint.into()) }
}

/// Draw a Texture2D with position defined as Vector2
#[inline]
pub fn draw_v(&self, _raylib: &mut Raylib, pos: Vector2, tint: Color) {
    unsafe { ffi::DrawTextureV(self.raw.deref().clone(), pos.into(), tint.into()) }
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
            pos.into(),
            rotation,
            scale,
            tint.into(),
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
            source.into(),
            pos.into(),
            tint.into(),
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
            source.into(),
            dest.into(),
            origin.into(),
            rotation,
            tint.into(),
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
            patch_info.into(),
            dest.into(),
            origin.into(),
            rotation,
            tint.into(),
        )
    }
}
*/

/// Setup canvas (framebuffer) to start drawing
#[inline]
pub fn BeginDrawing() {}

/// End canvas drawing and swap buffers (double buffering)
#[inline]
pub fn EndDrawing() {}

/// Begin 2D mode with custom camera (2D)
#[inline]
pub fn BeginMode2D(camera: Camera2D) {}

/// Ends 2D mode with custom camera
#[inline]
pub fn EndMode2D() {}

/// Begin 3D mode with custom camera (3D)
#[inline]
pub fn BeginMode3D(camera: Camera3D) {}

/// Ends 3D mode and returns to default 2D orthographic mode
#[inline]
pub fn EndMode3D() {}

/// Begin drawing to render texture
#[inline]
pub fn BeginTextureMode(target: RenderTexture2D) {}

/// Ends drawing to render texture
#[inline]
pub fn EndTextureMode() {}

/// Begin custom shader drawing
#[inline]
pub fn BeginShaderMode(shader: Shader) {}

/// End custom shader drawing (use default shader)
#[inline]
pub fn EndShaderMode() {}

/// Begin blending mode (alpha, additive, multiplied, subtract, custom)
#[inline]
pub fn BeginBlendMode(mode: u32) {}

/// End blending mode (reset to default: alpha blending)
#[inline]
pub fn EndBlendMode() {}

/// Begin scissor mode (define screen area for following drawing)
#[inline]
pub fn BeginScissorMode(x: u32, y: u32, width: u32, height: u32) {}

/// End scissor mode
#[inline]
pub fn EndScissorMode() {}

/// Begin stereo rendering (requires VR simulator)
#[inline]
pub fn BeginVrStereoMode(config: VrStereoConfig) {}

/// End stereo rendering (requires VR simulator)
#[inline]
pub fn EndVrStereoMode() {}
