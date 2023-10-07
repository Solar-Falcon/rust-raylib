use crate::{
    color::Color,
    ffi,
    math::{BoundingBox, Camera, Camera2D, Camera3D, Matrix, Ray, Rectangle, Vector2, Vector3},
    model::{Material, Mesh, Model},
    shader::Shader,
    text::Font,
    texture::{NPatchInfo, RenderTexture2D, Texture, Texture2D},
    vr::VrStereoConfig,
    Raylib,
};

use std::{ffi::CString, ops::{Deref, Range}};

pub use crate::ffi::BlendMode;

/// A struct containing the info for drawing textures.
#[derive(Clone, Debug)]
pub struct DrawTextureParams {
    /// Part of texture to draw. If None - draw the whole texture.
    /// Default: None
    pub source: Option<Rectangle>,
    /// Default: (1.0, 1.0)
    pub scale: Vector2,
    /// Rotate around this point.
    /// Default: (0, 0)
    pub origin: Vector2,
    /// Default: 0.0
    pub rotation: f32,
    /// Default: white.
    pub tint: Color,
}

impl Default for DrawTextureParams {
    #[inline]
    fn default() -> Self {
        Self {
            source: None,
            scale: Vector2 { x: 1., y: 1. },
            origin: Vector2 { x: 0., y: 0. },
            rotation: 0.,
            tint: Color::WHITE,
        }
    }
}

/// A struct containing the info for drawing billboard textures.
#[derive(Clone, Debug)]
pub struct DrawBillboardParams {
    /// Part of texture to draw. If None - draw the whole texture.
    /// Default: None
    pub source: Option<Rectangle>,
    /// Default: (0, 1, 0) - Y axis
    pub up: Vector3,
    /// Rotate the texture around this point.
    /// Default: (0, 0)
    pub origin: Vector2,
    /// Default: 0.0
    pub rotation: f32,
    /// Default: white.
    pub tint: Color,
}

impl Default for DrawBillboardParams {
    #[inline]
    fn default() -> Self {
        Self {
            source: None,
            up: Vector3 { x: 0., y: 1., z: 0. },
            origin: Vector2 { x: 0., y: 0. },
            rotation: 0.,
            tint: Color::WHITE,
        }
    }
}

/// An object that handles drawing
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
        self.0
    }
}

impl<'a> Drop for DrawHandle<'a> {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::EndDrawing() }
    }
}

/// An object that handles drawing with a custom 2D camera
pub struct DrawMode2D<'a, T>(&'a mut T);

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
        self.0
    }
}

impl<'a, T> Drop for DrawMode2D<'a, T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::EndMode2D() }
    }
}

/// An object that handles drawing with a custom 3D camera
pub struct DrawMode3D<'a, T>(&'a mut T);

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
        self.0
    }
}

impl<'a, T> Drop for DrawMode3D<'a, T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::EndMode3D() }
    }
}

/// An object that handles drawing onto a `RenderTexture`
pub struct DrawTextureMode<'a, T>(&'a mut T);

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
        self.0
    }
}

impl<'a, T> Drop for DrawTextureMode<'a, T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::EndTextureMode() }
    }
}

/// An object that handles drawing with a custom shader
pub struct DrawShaderMode<'a, T>(&'a mut T);

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
        self.0
    }
}

impl<'a, T> Drop for DrawShaderMode<'a, T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::EndShaderMode() }
    }
}

/// An object that handles drawing with a custom blend mode
pub struct DrawBlendMode<'a, T>(&'a mut T);

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
        self.0
    }
}

impl<'a, T> Drop for DrawBlendMode<'a, T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::EndBlendMode() }
    }
}

/// An object that handles drawing within a screen area
pub struct DrawScissorMode<'a, T>(&'a mut T);

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
        self.0
    }
}

impl<'a, T> Drop for DrawScissorMode<'a, T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::EndScissorMode() }
    }
}

/// An object that handles stereo drawing (VR)
pub struct DrawVrStereoMode<'a, T>(&'a mut T);

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
        self.0
    }
}

impl<'a, T> Drop for DrawVrStereoMode<'a, T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::EndVrStereoMode() }
    }
}

/// A trait that contains all the drawing functions 
pub trait Draw
where
    Self: Sized,
{
    /// Set background color (framebuffer clear color)
    #[inline]
    fn clear_background(&mut self, color: Color) {
        unsafe { ffi::ClearBackground(color.into()) }
    }

    /// Begin 2D mode with custom camera (2D)
    #[inline]
    fn begin_mode_2d(&mut self, camera: Camera2D) -> DrawMode2D<Self> {
        unsafe {
            ffi::BeginMode2D(camera.into());
        }

        DrawMode2D(self)
    }

    /// Begin 3D mode with custom camera (3D)
    #[inline]
    fn begin_mode_3d(&mut self, camera: Camera3D) -> DrawMode3D<Self> {
        unsafe {
            ffi::BeginMode3D(camera.into());
        }

        DrawMode3D(self)
    }

    /// Begin drawing to render texture
    #[inline]
    fn begin_texture_mode(&mut self, target: &RenderTexture2D) -> DrawTextureMode<Self> {
        unsafe {
            ffi::BeginTextureMode(target.raw.clone());
        }

        DrawTextureMode(self)
    }

    /// Begin custom shader drawing
    #[inline]
    fn begin_shader_mode(&mut self, shader: &Shader) -> DrawShaderMode<Self> {
        unsafe {
            ffi::BeginShaderMode(shader.raw.clone());
        }

        DrawShaderMode(self)
    }

    /// Begin blending mode (alpha, additive, multiplied, subtract, custom)
    #[inline]
    fn begin_blend_mode(&mut self, mode: BlendMode) -> DrawBlendMode<Self> {
        unsafe {
            ffi::BeginBlendMode(mode as _);
        }

        DrawBlendMode(self)
    }

    /// Begin scissor mode (define screen area for following drawing)
    #[inline]
    fn begin_scissor_mode(
        &mut self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> DrawScissorMode<Self> {
        unsafe {
            ffi::BeginScissorMode(x as _, y as _, width as _, height as _);
        }

        DrawScissorMode(self)
    }

    /// Begin stereo rendering (requires VR simulator)
    #[inline]
    fn begin_vr_stereo_mode(
        &mut self,
        config: VrStereoConfig,
    ) -> DrawVrStereoMode<Self> {
        unsafe {
            ffi::BeginVrStereoMode(config.into());
        }

        DrawVrStereoMode(self)
    }

    /// Draw a part of a texture defined by source and destination rectangles
    #[inline]
    fn draw_texture(
        &mut self,
        tex: &Texture,
        position: Vector2,
        params: DrawTextureParams,
    ) {
        // rectangle checks?
        unsafe {
            ffi::DrawTexturePro(
                tex.raw.clone(),
                params.source.unwrap_or(Rectangle::new(0., 0., tex.width() as _, tex.height() as _)).into(),
                Rectangle::new(
                    position.x,
                    position.y,
                    params.scale.x * tex.width() as f32,
                    params.scale.y * tex.height() as f32,
                ).into(),
                params.origin.into(),
                params.rotation,
                params.tint.into(),
            )
        }
    }

    /// Draws a texture (or part of it) that stretches or shrinks nicely
    #[inline]
    fn draw_texture_patch(
        &mut self,
        tex: &Texture,
        position: Vector2,
        params: DrawTextureParams,
        patch_info: NPatchInfo,
    ) {
        unsafe {
            ffi::DrawTextureNPatch(
                tex.raw.clone(),
                patch_info.into(),
                Rectangle::new(position.x, position.y, params.scale.x * tex.width() as f32, params.scale.y * tex.height() as f32).into(),
                params.origin.into(),
                params.rotation,
                params.tint.into(),
            )
        }
    }

    /// Set texture and rectangle to be used on shapes drawing
    #[inline]
    fn set_shapes_texture(&mut self, texture: &Texture, source: Rectangle) {
        unsafe { ffi::SetShapesTexture(texture.raw.clone(), source.into()) }
    }

    /// Draw a pixel
    #[inline]
    fn draw_pixel(&mut self, position: Vector2, color: Color) {
        unsafe { ffi::DrawPixelV(position.into(), color.into()) }
    }

    /// Draw a line
    #[inline]
    fn draw_line(&mut self, start: Vector2, end: Vector2, color: Color) {
        unsafe { ffi::DrawLineV(start.into(), end.into(), color.into()) }
    }

    /// Draw a line defining thickness
    #[inline]
    fn draw_line_thick(&mut self, start: Vector2, end: Vector2, thickness: f32, color: Color) {
        unsafe { ffi::DrawLineEx(start.into(), end.into(), thickness, color.into()) }
    }

    /// Draw a line using cubic-bezier curves in-out
    #[inline]
    fn draw_line_bezier(&mut self, start: Vector2, end: Vector2, thickness: f32, color: Color) {
        unsafe { ffi::DrawLineBezier(start.into(), end.into(), thickness, color.into()) }
    }

    /// Draw line using quadratic bezier curves with a control point
    #[inline]
    fn draw_line_bezier_quad(
        &mut self,
        start: Vector2,
        end: Vector2,
        control_pos: Vector2,
        thickness: f32,
        color: Color,
    ) {
        unsafe {
            ffi::DrawLineBezierQuad(
                start.into(),
                end.into(),
                control_pos.into(),
                thickness,
                color.into(),
            )
        }
    }

    /// Draw line using cubic bezier curves with 2 control points
    #[inline]
    fn draw_line_bezier_cubic(
        &mut self,
        start: Vector2,
        end: Vector2,
        start_control_pos: Vector2,
        end_control_pos: Vector2,
        thickness: f32,
        color: Color,
    ) {
        unsafe {
            ffi::DrawLineBezierCubic(
                start.into(),
                end.into(),
                start_control_pos.into(),
                end_control_pos.into(),
                thickness,
                color.into(),
            )
        }
    }

    /// Draw lines sequence
    #[inline]
    fn draw_line_strip(&mut self, points: &[Vector2], color: Color) {
        unsafe { ffi::DrawLineStrip(points.as_ptr() as *mut _, points.len() as _, color.into()) }
    }

    /// Draw a color-filled circle
    #[inline]
    fn draw_circle(&mut self, center: Vector2, radius: f32, color: Color) {
        unsafe { ffi::DrawCircleV(center.into(), radius, color.into()) }
    }

    /// Draw circle outline
    #[inline]
    fn draw_circle_lines(&mut self, center_x: i32, center_y: i32, radius: f32, color: Color) {
        unsafe { ffi::DrawCircleLines(center_x, center_y, radius, color.into()) }
    }

    /// Draw ellipse
    #[inline]
    fn draw_ellipse(
        &mut self,
        center: Vector2,
        radius: Vector2,
        color: Color,
    ) {
        unsafe { ffi::DrawEllipse(center.x as _, center.y as _, radius.x as _, radius.y as _, color.into()) }
    }

    /// Draw ellipse outline
    #[inline]
    fn draw_ellipse_lines(
        &mut self,
        center: Vector2,
        radius: Vector2,
        color: Color,
    ) {
        unsafe { ffi::DrawEllipseLines(center.x as _, center.y as _, radius.x as _, radius.y as _, color.into()) }
    }

    /// Draw a piece of a circle
    #[inline]
    fn draw_circle_sector(
        &mut self,
        center: Vector2,
        radius: f32,
        angle: Range<f32>,
        segments: u32,
        color: Color,
    ) {
        unsafe {
            ffi::DrawCircleSector(
                center.into(),
                radius,
                angle.start,
                angle.end,
                segments as _,
                color.into(),
            )
        }
    }

    /// Draw circle sector outline
    #[inline]
    fn draw_circle_sector_lines(
        &mut self,
        center: Vector2,
        radius: f32,
        angle: Range<f32>,
        segments: u32,
        color: Color,
    ) {
        unsafe {
            ffi::DrawCircleSectorLines(
                center.into(),
                radius,
                angle.start,
                angle.end,
                segments as _,
                color.into(),
            )
        }
    }

    /// Draw a gradient-filled circle
    #[inline]
    fn draw_circle_gradient(
        &mut self,
        center: Vector2,
        radius: f32,
        color1: Color,
        color2: Color,
    ) {
        unsafe { ffi::DrawCircleGradient(center.x as _, center.y as _, radius, color1.into(), color2.into()) }
    }

    /// Draw ring
    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn draw_ring(
        &mut self,
        center: Vector2,
        inner_radius: f32,
        outer_radius: f32,
        start_angle: f32,
        end_angle: f32,
        segments: u32,
        color: Color,
    ) {
        unsafe {
            ffi::DrawRing(
                center.into(),
                inner_radius,
                outer_radius,
                start_angle,
                end_angle,
                segments as _,
                color.into(),
            )
        }
    }

    /// Draw ring outline
    #[inline]
    fn draw_ring_lines(
        &mut self,
        center: Vector2,
        radius: Range<f32>,
        angle: Range<f32>,
        segments: u32,
        color: Color,
    ) {
        unsafe {
            ffi::DrawRingLines(
                center.into(),
                radius.start,
                radius.end,
                angle.start,
                angle.end,
                segments as _,
                color.into(),
            )
        }
    }

    /// Draw a color-filled rectangle
    #[inline]
    fn draw_rectangle(&mut self, rect: Rectangle, color: Color) {
        unsafe { ffi::DrawRectangleRec(rect.into(), color.into()) }
    }

    /// Draw rectangle outline
    #[inline]
    fn draw_rectangle_lines(&mut self, rect: Rectangle, color: Color) {
        unsafe { ffi::DrawRectangleLines(rect.x as _, rect.y as _, rect.width as _, rect.height as _, color.into()) }
    }

    /// Draw rectangle outline with thickness
    #[inline]
    fn draw_rectangle_lines_thick(&mut self, rect: Rectangle, line_thickness: f32, color: Color) {
        unsafe { ffi::DrawRectangleLinesEx(rect.into(), line_thickness, color.into()) }
    }

    /// Draw a color-filled rotated
    #[inline]
    fn draw_rectangle_rotated(
        &mut self,
        rect: Rectangle,
        origin: Vector2,
        rotation: f32,
        color: Color,
    ) {
        unsafe { ffi::DrawRectanglePro(rect.into(), origin.into(), rotation, color.into()) }
    }

    /// Draw a vertical-gradient-filled rectangle
    #[inline]
    fn draw_rectangle_gradient_vertical(
        &mut self,
        rect: Rectangle,
        color1: Color,
        color2: Color,
    ) {
        unsafe {
            ffi::DrawRectangleGradientV(rect.x as _, rect.y as _, rect.width as _, rect.height as _, color1.into(), color2.into())
        }
    }

    /// Draw a horizontal-gradient-filled rectangle
    #[inline]
    fn draw_rectangle_gradient_horizontal(
        &mut self,
        rect: Rectangle,
        color1: Color,
        color2: Color,
    ) {
        unsafe {
            ffi::DrawRectangleGradientH(rect.x as _, rect.y as _, rect.width as _, rect.height as _, color1.into(), color2.into())
        }
    }

    /// Draw a gradient-filled rectangle with custom vertex colors
    #[inline]
    fn draw_rectangle_gradient(
        &mut self,
        rect: Rectangle,
        col1: Color,
        col2: Color,
        col3: Color,
        col4: Color,
    ) {
        unsafe {
            ffi::DrawRectangleGradientEx(
                rect.into(),
                col1.into(),
                col2.into(),
                col3.into(),
                col4.into(),
            )
        }
    }

    /// Draw rectangle with rounded edges
    #[inline]
    fn draw_rectangle_rounded(
        &mut self,
        rect: Rectangle,
        roundness: f32,
        segments: u32,
        color: Color,
    ) {
        unsafe { ffi::DrawRectangleRounded(rect.into(), roundness, segments as _, color.into()) }
    }

    /// Draw rectangle with rounded edges outline
    #[inline]
    fn draw_rectangle_rounded_lines(
        &mut self,
        rect: Rectangle,
        roundness: f32,
        segments: u32,
        line_thickness: f32,
        color: Color,
    ) {
        unsafe {
            ffi::DrawRectangleRoundedLines(
                rect.into(),
                roundness,
                segments as _,
                line_thickness,
                color.into(),
            )
        }
    }

    /// Draw a color-filled triangle (vertex in counter-clockwise order!)
    #[inline]
    fn draw_triangle(&mut self, v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
        unsafe { ffi::DrawTriangle(v1.into(), v2.into(), v3.into(), color.into()) }
    }

    /// Draw triangle outline (vertex in counter-clockwise order!)
    #[inline]
    fn draw_triangle_lines(&mut self, v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
        unsafe { ffi::DrawTriangleLines(v1.into(), v2.into(), v3.into(), color.into()) }
    }

    /// Draw a triangle fan defined by points (first vertex is the center)
    #[inline]
    fn draw_triangle_fan(&mut self, points: &[Vector2], color: Color) {
        unsafe { ffi::DrawTriangleFan(points.as_ptr() as *mut _, points.len() as _, color.into()) }
    }

    /// Draw a triangle strip defined by points
    #[inline]
    fn draw_triangle_strip(&mut self, points: &[Vector2], color: Color) {
        unsafe {
            ffi::DrawTriangleStrip(points.as_ptr() as *mut _, points.len() as _, color.into())
        }
    }

    /// Draw a regular polygon (Vector version)
    #[inline]
    fn draw_polygon(
        &mut self,
        center: Vector2,
        sides: u32,
        radius: f32,
        rotation: f32,
        color: Color,
    ) {
        unsafe { ffi::DrawPoly(center.into(), sides as _, radius, rotation, color.into()) }
    }

    /// Draw a polygon outline of n sides
    #[inline]
    fn draw_polygon_lines(
        &mut self,
        center: Vector2,
        sides: u32,
        radius: f32,
        rotation: f32,
        color: Color,
    ) {
        unsafe { ffi::DrawPolyLines(center.into(), sides as _, radius, rotation, color.into()) }
    }

    /// Draw a polygon outline of n sides with thickness
    #[inline]
    fn draw_polygon_lines_thick(
        &mut self,
        center: Vector2,
        sides: u32,
        radius: f32,
        rotation: f32,
        line_thickness: f32,
        color: Color,
    ) {
        unsafe {
            ffi::DrawPolyLinesEx(
                center.into(),
                sides as _,
                radius,
                rotation,
                line_thickness,
                color.into(),
            )
        }
    }

    /// Draw current FPS
    #[inline]
    fn draw_fps(&mut self, position: Vector2) {
        unsafe { ffi::DrawFPS(position.x as _, position.y as _) }
    }

    /// Draw text (using default font)
    #[inline]
    fn draw_text(&mut self, text: &str, position: Vector2, font_size: u32, color: Color) {
        let text = CString::new(text).unwrap();

        unsafe { ffi::DrawText(text.as_ptr(), position.x as _, position.y as _, font_size as _, color.into()) }
    }

    /// Draw text using font and additional parameters
    #[inline]
    fn draw_text_with_font(
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
            ffi::DrawTextEx(
                font.raw.clone(),
                text.as_ptr(),
                pos.into(),
                font_size,
                spacing,
                tint.into(),
            )
        }
    }

    /// Draw text using Font and pro parameters (rotation)
    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn draw_text_with_font_and_rotation(
        &mut self,
        text: &str,
        pos: Vector2,
        origin: Vector2,
        rotation: f32,
        font: &Font,
        font_size: f32,
        spacing: f32,
        tint: Color,
    ) {
        let text = CString::new(text).unwrap();

        unsafe {
            ffi::DrawTextPro(
                font.raw.clone(),
                text.as_ptr(),
                pos.into(),
                origin.into(),
                rotation,
                font_size,
                spacing,
                tint.into(),
            )
        }
    }

    /// Draw one character
    #[inline]
    fn draw_char(&mut self, ch: char, pos: Vector2, font: &Font, font_size: f32, tint: Color) {
        unsafe {
            ffi::DrawTextCodepoint(
                font.raw.clone(),
                ch as _,
                pos.into(),
                font_size,
                tint.into(),
            )
        }
    }

    /// Draw multiple characters
    #[inline]
    fn draw_chars(
        &mut self,
        chars: &[char],
        pos: Vector2,
        font: &Font,
        font_size: f32,
        spacing: f32,
        tint: Color,
    ) {
        unsafe {
            ffi::DrawTextCodepoints(
                font.raw.clone(),
                chars.as_ptr() as *const _,
                chars.len() as _,
                pos.into(),
                font_size,
                spacing,
                tint.into(),
            )
        }
    }

    /// Draw a line in 3D world space
    #[inline]
    fn draw_line_3d(&mut self, start_pos: Vector3, end_pos: Vector3, color: Color) {
        unsafe { ffi::DrawLine3D(start_pos.into(), end_pos.into(), color.into()) }
    }

    /// Draw a point in 3D space, actually a small line
    #[inline]
    fn draw_point_3d(&mut self, position: Vector3, color: Color) {
        unsafe { ffi::DrawPoint3D(position.into(), color.into()) }
    }

    /// Draw a circle in 3D world space
    #[inline]
    fn draw_circle_3d(
        &mut self,
        center: Vector3,
        radius: f32,
        rotation_axis: Vector3,
        rotation_angle: f32,
        color: Color,
    ) {
        unsafe {
            ffi::DrawCircle3D(
                center.into(),
                radius,
                rotation_axis.into(),
                rotation_angle,
                color.into(),
            )
        }
    }

    /// Draw a color-filled triangle (vertex in counter-clockwise order!)
    #[inline]
    fn draw_triangle_3d(&mut self, v1: Vector3, v2: Vector3, v3: Vector3, color: Color) {
        unsafe { ffi::DrawTriangle3D(v1.into(), v2.into(), v3.into(), color.into()) }
    }

    /// Draw a triangle strip defined by points
    #[inline]
    fn draw_triangle_strip_3d(&mut self, points: &[Vector3], color: Color) {
        unsafe {
            ffi::DrawTriangleStrip3D(points.as_ptr() as *mut _, points.len() as _, color.into())
        }
    }

    /// Draw cube
    #[inline]
    fn draw_cube(&mut self, position: Vector3, size: Vector3, color: Color) {
        unsafe { ffi::DrawCubeV(position.into(), size.into(), color.into()) }
    }

    /// Draw cube wires (Vector version)
    #[inline]
    fn draw_cube_wires(&mut self, position: Vector3, size: Vector3, color: Color) {
        unsafe { ffi::DrawCubeWiresV(position.into(), size.into(), color.into()) }
    }

    /// Draw sphere
    #[inline]
    fn draw_sphere(&mut self, center_pos: Vector3, radius: f32, color: Color) {
        unsafe { ffi::DrawSphere(center_pos.into(), radius, color.into()) }
    }

    /// Draw sphere with extended parameters
    #[inline]
    fn draw_sphere_ex(
        &mut self,
        center_pos: Vector3,
        radius: f32,
        rings: u32,
        slices: u32,
        color: Color,
    ) {
        unsafe {
            ffi::DrawSphereEx(
                center_pos.into(),
                radius,
                rings as _,
                slices as _,
                color.into(),
            )
        }
    }

    /// Draw sphere wires
    #[inline]
    fn draw_sphere_wires(
        &mut self,
        center_pos: Vector3,
        radius: f32,
        rings: u32,
        slices: u32,
        color: Color,
    ) {
        unsafe {
            ffi::DrawSphereWires(
                center_pos.into(),
                radius,
                rings as _,
                slices as _,
                color.into(),
            )
        }
    }

    /// Draw a cylinder/cone
    #[inline]
    fn draw_cylinder(
        &mut self,
        position: Vector3,
        radius_top: f32,
        radius_bottom: f32,
        height: f32,
        slices: u32,
        color: Color,
    ) {
        unsafe {
            ffi::DrawCylinder(
                position.into(),
                radius_top,
                radius_bottom,
                height,
                slices as _,
                color.into(),
            )
        }
    }

    /// Draw a cylinder with extended parameters
    #[inline]
    fn draw_cylinder_ex(
        &mut self,
        pos_top: Vector3,
        pos_bottom: Vector3,
        radius_top: f32,
        radius_bottom: f32,
        sides: u32,
        color: Color,
    ) {
        unsafe {
            ffi::DrawCylinderEx(
                pos_bottom.into(),
                pos_top.into(),
                radius_bottom,
                radius_top,
                sides as _,
                color.into(),
            )
        }
    }

    /// Draw a cylinder/cone wires
    #[inline]
    fn draw_cylinder_wires(
        &mut self,
        position: Vector3,
        radius_top: f32,
        radius_bottom: f32,
        height: f32,
        slices: u32,
        color: Color,
    ) {
        unsafe {
            ffi::DrawCylinderWires(
                position.into(),
                radius_top,
                radius_bottom,
                height,
                slices as _,
                color.into(),
            )
        }
    }

    /// Draw a cylinder wires with extended parameters
    #[inline]
    fn draw_cylinder_wires_ex(
        &mut self,
        pos_top: Vector3,
        pos_bottom: Vector3,
        radius_top: f32,
        radius_bottom: f32,
        sides: u32,
        color: Color,
    ) {
        unsafe {
            ffi::DrawCylinderWiresEx(
                pos_bottom.into(),
                pos_top.into(),
                radius_bottom,
                radius_top,
                sides as _,
                color.into(),
            )
        }
    }

    /// Draw a capsule with the center of its sphere caps at start_pos and end_pos
    #[inline]
    fn draw_capsule(
        &mut self,
        start_pos: Vector3,
        end_pos: Vector3,
        radius: f32,
        slices: u32,
        rings: u32,
        color: Color,
    ) {
        unsafe {
            ffi::DrawCapsule(
                start_pos.into(),
                end_pos.into(),
                radius,
                slices as _,
                rings as _,
                color.into(),
            )
        }
    }

    /// Draw capsule wireframe with the center of its sphere caps at start_pos and end_pos
    #[inline]
    fn draw_capsule_wires(
        &mut self,
        start_pos: Vector3,
        end_pos: Vector3,
        radius: f32,
        slices: u32,
        rings: u32,
        color: Color,
    ) {
        unsafe {
            ffi::DrawCapsuleWires(
                start_pos.into(),
                end_pos.into(),
                radius,
                slices as _,
                rings as _,
                color.into(),
            )
        }
    }

    /// Draw a plane XZ
    #[inline]
    fn draw_plane(&mut self, center_pos: Vector3, size: Vector2, color: Color) {
        unsafe { ffi::DrawPlane(center_pos.into(), size.into(), color.into()) }
    }

    /// Draw a ray line
    #[inline]
    fn draw_ray(&mut self, ray: Ray, color: Color) {
        unsafe { ffi::DrawRay(ray.into(), color.into()) }
    }

    /// Draw a grid (centered at (0, 0, 0))
    #[inline]
    fn draw_grid(&mut self, slices: u32, spacing: f32) {
        unsafe { ffi::DrawGrid(slices as _, spacing) }
    }

    /// Draw a model (with texture if set)
    #[inline]
    fn draw_model(&mut self, model: &Model, position: Vector3, scale: f32, tint: Color) {
        unsafe { ffi::DrawModel(model.raw.clone(), position.into(), scale, tint.into()) }
    }

    /// Draw a model with extended parameters
    #[inline]
    fn draw_model_ex(
        &mut self,
        model: &Model,
        position: Vector3,
        rotation_axis: Vector3,
        rotation_angle: f32,
        scale: Vector3,
        tint: Color,
    ) {
        unsafe {
            ffi::DrawModelEx(
                model.raw.clone(),
                position.into(),
                rotation_axis.into(),
                rotation_angle,
                scale.into(),
                tint.into(),
            )
        }
    }

    /// Draw a model wires (with texture if set)
    #[inline]
    fn draw_model_wires(&mut self, model: &Model, position: Vector3, scale: f32, tint: Color) {
        unsafe { ffi::DrawModelWires(model.raw.clone(), position.into(), scale, tint.into()) }
    }

    /// Draw a model wires (with texture if set) with extended parameters
    #[inline]
    fn draw_model_wires_ex(
        &mut self,
        model: &Model,
        position: Vector3,
        rotation_axis: Vector3,
        rotation_angle: f32,
        scale: Vector3,
        tint: Color,
    ) {
        unsafe {
            ffi::DrawModelWiresEx(
                model.raw.clone(),
                position.into(),
                rotation_axis.into(),
                rotation_angle,
                scale.into(),
                tint.into(),
            )
        }
    }

    /// Draw bounding box (wires)
    #[inline]
    fn draw_bounding_box(&mut self, bbox: BoundingBox, color: Color) {
        unsafe { ffi::DrawBoundingBox(bbox.into(), color.into()) }
    }

    /// Draw a billboard texture
    #[inline]
    fn draw_billboard(
        &mut self,
        camera: Camera,
        texture: &Texture2D,
        position: Vector3,
        size: Vector2,
        params: DrawBillboardParams,
    ) {
        unsafe {
            ffi::DrawBillboardPro(
                camera.into(),
                texture.raw.clone(),
                params.source.unwrap_or(Rectangle::new(0., 0., texture.width() as _, texture.height() as _)).into(),
                position.into(),
                params.up.into(),
                size.into(),
                params.origin.into(),
                params.rotation,
                params.tint.into(),
            )
        }
    }

    /// Draw a 3d mesh with material and transform
    #[inline]
    fn draw_mesh(&mut self, mesh: &Mesh, material: &Material, transform: Matrix) {
        unsafe { ffi::DrawMesh(mesh.raw.clone(), material.raw.clone(), transform.into()) }
    }

    /// Draw multiple mesh instances with material and different transforms
    #[inline]
    fn draw_mesh_instanced(&mut self, mesh: &Mesh, material: &Material, transforms: &[Matrix]) {
        unsafe {
            ffi::DrawMeshInstanced(
                mesh.raw.clone(),
                material.raw.clone(),
                transforms.as_ptr() as *const _,
                transforms.len() as _,
            )
        }
    }
}

impl<'a> Draw for DrawHandle<'a> {}
impl<'a, T> Draw for DrawBlendMode<'a, T> {}
impl<'a, T> Draw for DrawMode2D<'a, T> {}
impl<'a, T> Draw for DrawMode3D<'a, T> {}
impl<'a, T> Draw for DrawScissorMode<'a, T> {}
impl<'a, T> Draw for DrawShaderMode<'a, T> {}
impl<'a, T> Draw for DrawTextureMode<'a, T> {}
impl<'a, T> Draw for DrawVrStereoMode<'a, T> {}
