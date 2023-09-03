use crate::{
    ffi,
    math::{Camera2D, Camera3D, Vector2, Rectangle},
    shader::Shader,
    texture::{RenderTexture2D, Texture, NPatchInfo},
    vr::VrStereoConfig, Raylib, color::Color,
};

use std::ops::Deref;

pub use crate::ffi::BlendMode;

pub struct DrawHandle<'a>(pub(crate) &'a mut Raylib);

impl<'a> DrawHandle<'a> {
    /// End canvas drawing and swap buffers (double buffering)
    #[inline]
    pub fn end_drawing(self) {
        drop(self)
    }
}

impl<'a> Deref for DrawHandle<'a> {
    type Target = Raylib;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Drop for DrawHandle<'a> {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::EndDrawing() }
    }
}

pub struct DrawMode2D<'a, T>(&'a mut T, Camera2D);

impl<'a, T> DrawMode2D<'a, T> {
    /// Ends 2D mode with custom camera
    #[inline]
    pub fn end_mode_2d(self) {
        drop(self)
    }
}

impl<'a, T> Deref for DrawMode2D<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T> Drop for DrawMode2D<'a, T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::EndMode2D() }
    }
}

pub struct DrawMode3D<'a, T>(&'a mut T, Camera3D);

impl<'a, T> DrawMode3D<'a, T> {
    /// Ends 3D mode and returns to default 2D orthographic mode
    #[inline]
    pub fn end_mode_3d(self) {
        drop(self)
    }
}

impl<'a, T> Deref for DrawMode3D<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T> Drop for DrawMode3D<'a, T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::EndMode3D() }
    }
}

pub struct DrawTextureMode<'a, T>(&'a mut T, RenderTexture2D);

impl<'a, T> DrawTextureMode<'a, T> {
    /// Ends drawing to render texture
    #[inline]
    pub fn end_texture_mode(self) {
        drop(self)
    }
}

impl<'a, T> Deref for DrawTextureMode<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T> Drop for DrawTextureMode<'a, T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::EndTextureMode() }
    }
}

pub struct DrawShaderMode<'a, T>(&'a mut T, Shader);

impl<'a, T> DrawShaderMode<'a, T> {
    /// End custom shader drawing (use default shader)
    #[inline]
    pub fn end_shader_mode(self) {
        drop(self)
    }
}

impl<'a, T> Deref for DrawShaderMode<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T> Drop for DrawShaderMode<'a, T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::EndShaderMode() }
    }
}

pub struct DrawBlendMode<'a, T>(&'a mut T, BlendMode);

impl<'a, T> DrawTextureMode<'a, T> {
    /// End blending mode (reset to default: alpha blending)
    #[inline]
    pub fn end_blend_mode(self) {
        drop(self)
    }
}

impl<'a, T> Deref for DrawBlendMode<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T> Drop for DrawBlendMode<'a, T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::EndBlendMode() }
    }
}

pub struct DrawScissorMode<'a, T>(&'a mut T, [u32; 4]);

impl<'a, T> DrawScissorMode<'a, T> {
    /// End scissor mode
    #[inline]
    pub fn end_scissor_mode(self) {
        drop(self)
    }
}

impl<'a, T> Deref for DrawScissorMode<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T> Drop for DrawScissorMode<'a, T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::EndScissorMode() }
    }
}

pub struct DrawVrStereoMode<'a, T>(&'a mut T, &'a VrStereoConfig);

impl<'a, T> DrawVrStereoMode<'a, T> {
    /// End stereo rendering (requires VR simulator)
    #[inline]
    pub fn end_vr_stereo_mode(self) {
        drop(self)
    }
}

impl<'a, T> Deref for DrawVrStereoMode<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T> Drop for DrawVrStereoMode<'a, T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::EndVrStereoMode() }
    }
}

pub trait Draw where Self: Sized {
    /// Begin 2D mode with custom camera (2D)
    #[inline]
    fn begin_mode_2d(&mut self, camera: Camera2D) -> DrawMode2D<Self> {
        unsafe { ffi::BeginMode2D(camera.clone().into()); }

        DrawMode2D(self, camera)
    }

    /// Begin 3D mode with custom camera (3D)
    #[inline]
    fn begin_mode_3d(&mut self, camera: Camera3D) -> DrawMode3D<Self> {
        unsafe { ffi::BeginMode3D(camera.clone().into()); }

        DrawMode3D(self, camera)
    }

    /// Begin drawing to render texture
    #[inline]
    fn begin_texture_mode(&mut self, target: RenderTexture2D) -> DrawTextureMode<Self> {
        unsafe { ffi::BeginTextureMode(target.raw.deref().clone()); }

        DrawTextureMode(self, target)
    }

    /// Begin custom shader drawing
    #[inline]
    fn begin_shader_mode(&mut self, shader: Shader) -> DrawShaderMode<Self> {
        unsafe { ffi::BeginShaderMode(shader.raw.deref().clone()); }

        DrawShaderMode(self, shader)
    }

    /// Begin blending mode (alpha, additive, multiplied, subtract, custom)
    #[inline]
    fn begin_blend_mode(&mut self, mode: BlendMode) -> DrawBlendMode<Self> {
        unsafe { ffi::BeginBlendMode(mode as _); }

        DrawBlendMode(self, mode)
    }

    /// Begin scissor mode (define screen area for following drawing)
    #[inline]
    fn begin_scissor_mode(&mut self, x: u32, y: u32, width: u32, height: u32) -> DrawScissorMode<Self> {
        unsafe { ffi::BeginScissorMode(x as _, y as _, width as _, height as _); }

        DrawScissorMode(self, [x, y, width, height])
    }

    /// Begin stereo rendering (requires VR simulator)
    #[inline]
    fn begin_vr_stereo_mode<'s, 'v: 's>(&'s mut self, config: &'v VrStereoConfig) -> DrawVrStereoMode<Self> {
        unsafe { ffi::BeginVrStereoMode(config.clone().into()); }

        DrawVrStereoMode(self, config)
    }

    /// Draw a Texture2D
    #[inline]
    fn draw_texture(&mut self, tex: &Texture, x: i32, y: i32, tint: Color) {
        unsafe { ffi::DrawTexture(tex.raw.deref().clone(), x, y, tint.into()) }
    }

    /// Draw a Texture2D with position defined as Vector2
    #[inline]
    fn draw_texture_v(&mut self, tex: &Texture, pos: Vector2, tint: Color) {
        unsafe { ffi::DrawTextureV(tex.raw.deref().clone(), pos.into(), tint.into()) }
    }

    /// Draw a Texture2D with extended parameters
    #[inline]
    fn draw_texture_ex(
        &mut self,
        tex: &Texture,
        pos: Vector2,
        rotation: f32,
        scale: f32,
        tint: Color,
    ) {
        unsafe {
            ffi::DrawTextureEx(
                tex.raw.deref().clone(),
                pos.into(),
                rotation,
                scale,
                tint.into(),
            )
        }
    }

    /// Draw a part of a texture defined by a rectangle
    #[inline]
    fn draw_texture_rect(&self, tex: &Texture, source: Rectangle, pos: Vector2, tint: Color) {
        // rectangle checks?
        unsafe {
            ffi::DrawTextureRec(
                tex.raw.deref().clone(),
                source.into(),
                pos.into(),
                tint.into(),
            )
        }
    }

    /// Draw a part of a texture defined by source and destination rectangles
    #[inline]
    fn draw_texture_pro(
        &mut self,
        tex: &Texture,
        source: Rectangle,
        dest: Rectangle,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    ) {
        // rectangle checks?
        unsafe {
            ffi::DrawTexturePro(
                tex.raw.deref().clone(),
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
    fn draw_texture_patch(
        &self,
        tex: &Texture,
        patch_info: NPatchInfo,
        dest: Rectangle,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    ) {
        unsafe {
            ffi::DrawTextureNPatch(
                tex.raw.deref().clone(),
                patch_info.into(),
                dest.into(),
                origin.into(),
                rotation,
                tint.into(),
            )
        }
    }
/*
    /// Set texture and rectangle to be used on shapes drawing
	fn SetShapesTexture(texture: Texture2D, source: Rectangle);

	/// Draw a pixel
	fn DrawPixel(posX: core::ffi::c_int, posY: core::ffi::c_int, color: Color);

	/// Draw a pixel (Vector version)
	fn DrawPixelV(position: Vector2, color: Color);

	/// Draw a line
	fn DrawLine(startPosX: core::ffi::c_int, startPosY: core::ffi::c_int, endPosX: core::ffi::c_int, endPosY: core::ffi::c_int, color: Color);
	
    /// Draw a line (Vector version)
	fn DrawLineV(startPos: Vector2, endPos: Vector2, color: Color);
	
    /// Draw a line defining thickness
	fn DrawLineEx(startPos: Vector2, endPos: Vector2, thick: core::ffi::c_float, color: Color);
	
    /// Draw a line using cubic-bezier curves in-out
	fn DrawLineBezier(startPos: Vector2, endPos: Vector2, thick: core::ffi::c_float, color: Color);
	
    /// Draw line using quadratic bezier curves with a control point
	fn DrawLineBezierQuad(startPos: Vector2, endPos: Vector2, controlPos: Vector2, thick: core::ffi::c_float, color: Color);
	
    /// Draw line using cubic bezier curves with 2 control points
	fn DrawLineBezierCubic(startPos: Vector2, endPos: Vector2, startControlPos: Vector2, endControlPos: Vector2, thick: core::ffi::c_float, color: Color);
	
    /// Draw lines sequence
	fn DrawLineStrip(points: *mut Vector2, pointCount: core::ffi::c_int, color: Color);
	
    /// Draw a color-filled circle
	fn DrawCircle(centerX: core::ffi::c_int, centerY: core::ffi::c_int, radius: core::ffi::c_float, color: Color);
	
    /// Draw a piece of a circle
	fn DrawCircleSector(center: Vector2, radius: core::ffi::c_float, startAngle: core::ffi::c_float, endAngle: core::ffi::c_float, segments: core::ffi::c_int, color: Color);
	
    /// Draw circle sector outline
	fn DrawCircleSectorLines(center: Vector2, radius: core::ffi::c_float, startAngle: core::ffi::c_float, endAngle: core::ffi::c_float, segments: core::ffi::c_int, color: Color);
	
    /// Draw a gradient-filled circle
	fn DrawCircleGradient(centerX: core::ffi::c_int, centerY: core::ffi::c_int, radius: core::ffi::c_float, color1: Color, color2: Color);
	
    /// Draw a color-filled circle (Vector version)
	fn DrawCircleV(center: Vector2, radius: core::ffi::c_float, color: Color);
	
    /// Draw circle outline
	fn DrawCircleLines(centerX: core::ffi::c_int, centerY: core::ffi::c_int, radius: core::ffi::c_float, color: Color);
	
    /// Draw ellipse
	fn DrawEllipse(centerX: core::ffi::c_int, centerY: core::ffi::c_int, radiusH: core::ffi::c_float, radiusV: core::ffi::c_float, color: Color);
	
    /// Draw ellipse outline
	fn DrawEllipseLines(centerX: core::ffi::c_int, centerY: core::ffi::c_int, radiusH: core::ffi::c_float, radiusV: core::ffi::c_float, color: Color);
	
    /// Draw ring
	fn DrawRing(center: Vector2, innerRadius: core::ffi::c_float, outerRadius: core::ffi::c_float, startAngle: core::ffi::c_float, endAngle: core::ffi::c_float, segments: core::ffi::c_int, color: Color);
	
    /// Draw ring outline
	fn DrawRingLines(center: Vector2, innerRadius: core::ffi::c_float, outerRadius: core::ffi::c_float, startAngle: core::ffi::c_float, endAngle: core::ffi::c_float, segments: core::ffi::c_int, color: Color);
	
    /// Draw a color-filled rectangle
	fn DrawRectangle(posX: core::ffi::c_int, posY: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int, color: Color);
	
    /// Draw a color-filled rectangle (Vector version)
	fn DrawRectangleV(position: Vector2, size: Vector2, color: Color);
	
    /// Draw a color-filled rectangle
	fn DrawRectangleRec(rec: Rectangle, color: Color);
	
    /// Draw a color-filled rectangle with pro parameters
	fn DrawRectanglePro(rec: Rectangle, origin: Vector2, rotation: core::ffi::c_float, color: Color);
	
    /// Draw a vertical-gradient-filled rectangle
	fn DrawRectangleGradientV(posX: core::ffi::c_int, posY: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int, color1: Color, color2: Color);
	
    /// Draw a horizontal-gradient-filled rectangle
	fn DrawRectangleGradientH(posX: core::ffi::c_int, posY: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int, color1: Color, color2: Color);
	
    /// Draw a gradient-filled rectangle with custom vertex colors
	fn DrawRectangleGradientEx(rec: Rectangle, col1: Color, col2: Color, col3: Color, col4: Color);
	
    /// Draw rectangle outline
	fn DrawRectangleLines(posX: core::ffi::c_int, posY: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int, color: Color);
	
    /// Draw rectangle outline with extended parameters
	fn DrawRectangleLinesEx(rec: Rectangle, lineThick: core::ffi::c_float, color: Color);
	
    /// Draw rectangle with rounded edges
	fn DrawRectangleRounded(rec: Rectangle, roundness: core::ffi::c_float, segments: core::ffi::c_int, color: Color);
	
    /// Draw rectangle with rounded edges outline
	fn DrawRectangleRoundedLines(rec: Rectangle, roundness: core::ffi::c_float, segments: core::ffi::c_int, lineThick: core::ffi::c_float, color: Color);
	
    /// Draw a color-filled triangle (vertex in counter-clockwise order!)
	fn DrawTriangle(v1: Vector2, v2: Vector2, v3: Vector2, color: Color);
	
    /// Draw triangle outline (vertex in counter-clockwise order!)
	fn DrawTriangleLines(v1: Vector2, v2: Vector2, v3: Vector2, color: Color);
	
    /// Draw a triangle fan defined by points (first vertex is the center)
	fn DrawTriangleFan(points: *mut Vector2, pointCount: core::ffi::c_int, color: Color);
	
    /// Draw a triangle strip defined by points
	fn DrawTriangleStrip(points: *mut Vector2, pointCount: core::ffi::c_int, color: Color);
	
    /// Draw a regular polygon (Vector version)
	fn DrawPoly(center: Vector2, sides: core::ffi::c_int, radius: core::ffi::c_float, rotation: core::ffi::c_float, color: Color);
	
    /// Draw a polygon outline of n sides
	fn DrawPolyLines(center: Vector2, sides: core::ffi::c_int, radius: core::ffi::c_float, rotation: core::ffi::c_float, color: Color);
	
    /// Draw a polygon outline of n sides with extended parameters
	fn DrawPolyLinesEx(center: Vector2, sides: core::ffi::c_int, radius: core::ffi::c_float, rotation: core::ffi::c_float, lineThick: core::ffi::c_float, color: Color);
*/
}

impl<'a> Draw for DrawHandle<'a> {}
impl<'a, T> Draw for DrawBlendMode<'a, T> {}
impl<'a, T> Draw for DrawMode2D<'a, T> {}
impl<'a, T> Draw for DrawMode3D<'a, T> {}
impl<'a, T> Draw for DrawScissorMode<'a, T> {}
impl<'a, T> Draw for DrawShaderMode<'a, T> {}
impl<'a, T> Draw for DrawTextureMode<'a, T> {}
impl<'a, T> Draw for DrawVrStereoMode<'a, T> {}
