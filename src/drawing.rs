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

use std::{ffi::CString, ops::Deref};

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
        self.0
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
        self.0
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
        self.0
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
        self.0
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
        self.0
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
        self.0
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
        self.0
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
        self.0
    }
}

impl<'a, T> Drop for DrawVrStereoMode<'a, T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::EndVrStereoMode() }
    }
}

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
            ffi::BeginMode2D(camera.clone().into());
        }

        DrawMode2D(self, camera)
    }

    /// Begin 3D mode with custom camera (3D)
    #[inline]
    fn begin_mode_3d(&mut self, camera: Camera3D) -> DrawMode3D<Self> {
        unsafe {
            ffi::BeginMode3D(camera.clone().into());
        }

        DrawMode3D(self, camera)
    }

    /// Begin drawing to render texture
    #[inline]
    fn begin_texture_mode(&mut self, target: RenderTexture2D) -> DrawTextureMode<Self> {
        unsafe {
            ffi::BeginTextureMode(target.raw.deref().clone());
        }

        DrawTextureMode(self, target)
    }

    /// Begin custom shader drawing
    #[inline]
    fn begin_shader_mode(&mut self, shader: Shader) -> DrawShaderMode<Self> {
        unsafe {
            ffi::BeginShaderMode(shader.raw.deref().clone());
        }

        DrawShaderMode(self, shader)
    }

    /// Begin blending mode (alpha, additive, multiplied, subtract, custom)
    #[inline]
    fn begin_blend_mode(&mut self, mode: BlendMode) -> DrawBlendMode<Self> {
        unsafe {
            ffi::BeginBlendMode(mode as _);
        }

        DrawBlendMode(self, mode)
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

        DrawScissorMode(self, [x, y, width, height])
    }

    /// Begin stereo rendering (requires VR simulator)
    #[inline]
    fn begin_vr_stereo_mode<'s, 'v: 's>(
        &'s mut self,
        config: &'v VrStereoConfig,
    ) -> DrawVrStereoMode<Self> {
        unsafe {
            ffi::BeginVrStereoMode(config.clone().into());
        }

        DrawVrStereoMode(self, config)
    }

    /// Draw a Texture2D
    #[inline]
    fn draw_texture(&mut self, tex: Texture, x: i32, y: i32, tint: Color) {
        unsafe { ffi::DrawTexture(tex.raw.deref().clone(), x, y, tint.into()) }
    }

    /// Draw a Texture2D with position defined as Vector2
    #[inline]
    fn draw_texture_v(&mut self, tex: Texture, pos: Vector2, tint: Color) {
        unsafe { ffi::DrawTextureV(tex.raw.deref().clone(), pos.into(), tint.into()) }
    }

    /// Draw a Texture2D with extended parameters
    #[inline]
    fn draw_texture_ex(
        &mut self,
        tex: Texture,
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
    fn draw_texture_rect(&mut self, tex: Texture, source: Rectangle, pos: Vector2, tint: Color) {
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
        tex: Texture,
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
        &mut self,
        tex: Texture,
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

    /// Set texture and rectangle to be used on shapes drawing
    #[inline]
    fn set_shapes_texture(&mut self, texture: Texture, source: Rectangle) {
        unsafe { ffi::SetShapesTexture(texture.raw.deref().clone(), source.into()) }
    }

    /// Draw a pixel
    #[inline]
    fn draw_pixel(&mut self, x: i32, y: i32, color: Color) {
        unsafe { ffi::DrawPixel(x, y, color.into()) }
    }

    /// Draw a pixel (Vector version)
    #[inline]
    fn draw_pixel_v(&mut self, position: Vector2, color: Color) {
        unsafe { ffi::DrawPixelV(position.into(), color.into()) }
    }

    /// Draw a line
    #[inline]
    fn draw_line(&mut self, start_x: i32, start_y: i32, end_x: i32, end_y: i32, color: Color) {
        unsafe { ffi::DrawLine(start_x, start_y, end_x, end_y, color.into()) }
    }

    /// Draw a line (Vector version)
    #[inline]
    fn draw_line_v(&mut self, start: Vector2, end: Vector2, color: Color) {
        unsafe { ffi::DrawLineV(start.into(), end.into(), color.into()) }
    }

    /// Draw a line defining thickness
    #[inline]
    fn draw_line_ex(&mut self, start: Vector2, end: Vector2, thickness: f32, color: Color) {
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
    fn draw_circle(&mut self, center_x: i32, center_y: i32, radius: f32, color: Color) {
        unsafe { ffi::DrawCircle(center_x, center_y, radius, color.into()) }
    }

    /// Draw a piece of a circle
    #[inline]
    fn draw_circle_sector(
        &mut self,
        center: Vector2,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        segments: u32,
        color: Color,
    ) {
        unsafe {
            ffi::DrawCircleSector(
                center.into(),
                radius,
                start_angle,
                end_angle,
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
        start_angle: f32,
        end_angle: f32,
        segments: u32,
        color: Color,
    ) {
        unsafe {
            ffi::DrawCircleSectorLines(
                center.into(),
                radius,
                start_angle,
                end_angle,
                segments as _,
                color.into(),
            )
        }
    }

    /// Draw a gradient-filled circle
    #[inline]
    fn draw_circle_gradient(
        &mut self,
        center_x: i32,
        center_y: i32,
        radius: f32,
        color1: Color,
        color2: Color,
    ) {
        unsafe { ffi::DrawCircleGradient(center_x, center_y, radius, color1.into(), color2.into()) }
    }

    /// Draw a color-filled circle (Vector version)
    #[inline]
    fn draw_circle_v(&mut self, center: Vector2, radius: f32, color: Color) {
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
        center_x: i32,
        center_y: i32,
        radius_h: f32,
        radius_v: f32,
        color: Color,
    ) {
        unsafe { ffi::DrawEllipse(center_x, center_y, radius_h, radius_v, color.into()) }
    }

    /// Draw ellipse outline
    #[inline]
    fn draw_ellipse_lines(
        &mut self,
        center_x: i32,
        center_y: i32,
        radius_h: f32,
        radius_v: f32,
        color: Color,
    ) {
        unsafe { ffi::DrawEllipseLines(center_x, center_y, radius_h, radius_v, color.into()) }
    }

    /// Draw ring
    #[inline]
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
        inner_radius: f32,
        outer_radius: f32,
        start_angle: f32,
        end_angle: f32,
        segments: u32,
        color: Color,
    ) {
        unsafe {
            ffi::DrawRingLines(
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

    /// Draw a color-filled rectangle
    #[inline]
    fn draw_rectangle(&mut self, x: i32, y: i32, width: u32, height: u32, color: Color) {
        unsafe { ffi::DrawRectangle(x, y, width as _, height as _, color.into()) }
    }

    /// Draw a color-filled rectangle (Vector version)
    #[inline]
    fn draw_rectangle_v(&mut self, pos: Vector2, size: Vector2, color: Color) {
        unsafe { ffi::DrawRectangleV(pos.into(), size.into(), color.into()) }
    }

    /// Draw a color-filled rectangle
    #[inline]
    fn draw_rectangle_rect(&mut self, rect: Rectangle, color: Color) {
        unsafe { ffi::DrawRectangleRec(rect.into(), color.into()) }
    }

    /// Draw a color-filled rectangle with pro parameters
    #[inline]
    fn draw_rectangle_pro(
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
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        color1: Color,
        color2: Color,
    ) {
        unsafe {
            ffi::DrawRectangleGradientV(x, y, width as _, height as _, color1.into(), color2.into())
        }
    }

    /// Draw a horizontal-gradient-filled rectangle
    #[inline]
    fn draw_rectangle_gradient_horizontal(
        &mut self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        color1: Color,
        color2: Color,
    ) {
        unsafe {
            ffi::DrawRectangleGradientH(x, y, width as _, height as _, color1.into(), color2.into())
        }
    }

    /// Draw a gradient-filled rectangle with custom vertex colors
    #[inline]
    fn draw_rectangle_gradient_ex(
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

    /// Draw rectangle outline
    #[inline]
    fn draw_rectangle_lines(&mut self, x: i32, y: i32, width: u32, height: u32, color: Color) {
        unsafe { ffi::DrawRectangleLines(x, y, width as _, height as _, color.into()) }
    }

    /// Draw rectangle outline with extended parameters
    #[inline]
    fn draw_rectangle_lines_ex(&mut self, rect: Rectangle, line_thickness: f32, color: Color) {
        unsafe { ffi::DrawRectangleLinesEx(rect.into(), line_thickness, color.into()) }
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

    /// Draw a polygon outline of n sides with extended parameters
    #[inline]
    fn draw_polygon_lines_ex(
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
    fn draw_fps(&mut self, x: i32, y: i32) {
        unsafe { ffi::DrawFPS(x, y) }
    }

    /// Draw text (using default font)
    #[inline]
    fn draw_text(&mut self, text: &str, x: i32, y: i32, font_size: u32, color: Color) {
        let text = CString::new(text).unwrap();

        unsafe { ffi::DrawText(text.as_ptr(), x, y, font_size as _, color.into()) }
    }

    /// Draw text using font and additional parameters
    #[inline]
    fn draw_text_ex(
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
            ffi::DrawTextEx(
                font.raw.deref().clone(),
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
    fn draw_text_pro(
        &mut self,
        font: &Font,
        text: &str,
        pos: Vector2,
        origin: Vector2,
        rotation: f32,
        font_size: f32,
        spacing: f32,
        tint: Color,
    ) {
        let text = CString::new(text).unwrap();

        unsafe {
            ffi::DrawTextPro(
                font.raw.deref().clone(),
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

    /// Draw one character (codepoint)
    #[inline]
    fn draw_char(&mut self, font: &Font, ch: char, pos: Vector2, font_size: f32, tint: Color) {
        unsafe {
            ffi::DrawTextCodepoint(
                font.raw.deref().clone(),
                ch as _,
                pos.into(),
                font_size,
                tint.into(),
            )
        }
    }

    /// Draw multiple character (codepoint)
    #[inline]
    fn draw_chars(
        &mut self,
        font: &Font,
        chars: &[char],
        pos: Vector2,
        font_size: f32,
        spacing: f32,
        tint: Color,
    ) {
        unsafe {
            ffi::DrawTextCodepoints(
                font.raw.deref().clone(),
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
    fn draw_line_3d(&mut self, start_pos: Vector3, end_pos: Vector3, color: Color) {
        unsafe { ffi::DrawLine3D(start_pos.into(), end_pos.into(), color.into()) }
    }

    /// Draw a point in 3D space, actually a small line
    fn draw_point_3d(&mut self, position: Vector3, color: Color) {
        unsafe { ffi::DrawPoint3D(position.into(), color.into()) }
    }

    /// Draw a circle in 3D world space
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
    fn draw_triangle_3d(&mut self, v1: Vector3, v2: Vector3, v3: Vector3, color: Color) {
        unsafe { ffi::DrawTriangle3D(v1.into(), v2.into(), v3.into(), color.into()) }
    }

    /// Draw a triangle strip defined by points
    fn draw_triangle_strip_3d(&mut self, points: &[Vector3], color: Color) {
        unsafe {
            ffi::DrawTriangleStrip3D(points.as_ptr() as *mut _, points.len() as _, color.into())
        }
    }

    /// Draw cube
    fn draw_cube(&mut self, position: Vector3, width: f32, height: f32, length: f32, color: Color) {
        unsafe { ffi::DrawCube(position.into(), width, height, length, color.into()) }
    }

    /// Draw cube (Vector version)
    fn draw_cube_v(&mut self, position: Vector3, size: Vector3, color: Color) {
        unsafe { ffi::DrawCubeV(position.into(), size.into(), color.into()) }
    }

    /// Draw cube wires
    fn draw_cube_wires(
        &mut self,
        position: Vector3,
        width: f32,
        height: f32,
        length: f32,
        color: Color,
    ) {
        unsafe { ffi::DrawCubeWires(position.into(), width, height, length, color.into()) }
    }

    /// Draw cube wires (Vector version)
    fn draw_cube_wires_v(&mut self, position: Vector3, size: Vector3, color: Color) {
        unsafe { ffi::DrawCubeWiresV(position.into(), size.into(), color.into()) }
    }

    /// Draw sphere
    fn draw_sphere(&mut self, center_pos: Vector3, radius: f32, color: Color) {
        unsafe { ffi::DrawSphere(center_pos.into(), radius, color.into()) }
    }

    /// Draw sphere with extended parameters
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

    /// Draw a cylinder with base at start_pos and top at end_pos
    fn draw_cylinder_ex(
        &mut self,
        start_pos: Vector3,
        end_pos: Vector3,
        start_radius: f32,
        end_radius: f32,
        sides: u32,
        color: Color,
    ) {
        unsafe {
            ffi::DrawCylinderEx(
                start_pos.into(),
                end_pos.into(),
                start_radius,
                end_radius,
                sides as _,
                color.into(),
            )
        }
    }

    /// Draw a cylinder/cone wires
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

    /// Draw a cylinder wires with base at start_pos and top at end_pos
    fn draw_cylinder_wires_ex(
        &mut self,
        start_pos: Vector3,
        end_pos: Vector3,
        start_radius: f32,
        end_radius: f32,
        sides: u32,
        color: Color,
    ) {
        unsafe {
            ffi::DrawCylinderWiresEx(
                start_pos.into(),
                end_pos.into(),
                start_radius,
                end_radius,
                sides as _,
                color.into(),
            )
        }
    }

    /// Draw a capsule with the center of its sphere caps at start_pos and end_pos
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
    fn draw_plane(&mut self, center_pos: Vector3, size: Vector2, color: Color) {
        unsafe { ffi::DrawPlane(center_pos.into(), size.into(), color.into()) }
    }

    /// Draw a ray line
    fn draw_ray(&mut self, ray: Ray, color: Color) {
        unsafe { ffi::DrawRay(ray.into(), color.into()) }
    }

    /// Draw a grid (centered at (0, 0, 0))
    fn draw_grid(&mut self, slices: i32, spacing: f32) {
        unsafe { ffi::DrawGrid(slices, spacing) }
    }

    /// Draw a model (with texture if set)
    fn draw_model(&mut self, model: &Model, position: Vector3, scale: f32, tint: Color) {
        unsafe { ffi::DrawModel(model.raw.clone(), position.into(), scale, tint.into()) }
    }

    /// Draw a model with extended parameters
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
    fn draw_model_wires(&mut self, model: &Model, position: Vector3, scale: f32, tint: Color) {
        unsafe { ffi::DrawModelWires(model.raw.clone(), position.into(), scale, tint.into()) }
    }

    /// Draw a model wires (with texture if set) with extended parameters
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
    fn draw_bounding_box(&mut self, bbox: BoundingBox, color: Color) {
        unsafe { ffi::DrawBoundingBox(bbox.into(), color.into()) }
    }

    /// Draw a billboard texture
    fn draw_billboard(
        &mut self,
        camera: Camera,
        texture: Texture2D,
        position: Vector3,
        size: f32,
        tint: Color,
    ) {
        unsafe {
            ffi::DrawBillboard(
                camera.into(),
                texture.raw.deref().clone(),
                position.into(),
                size,
                tint.into(),
            )
        }
    }

    /// Draw a billboard texture defined by source
    fn draw_billboard_rect(
        &mut self,
        camera: Camera,
        texture: Texture2D,
        source: Rectangle,
        position: Vector3,
        size: Vector2,
        tint: Color,
    ) {
        unsafe {
            ffi::DrawBillboardRec(
                camera.into(),
                texture.raw.deref().clone(),
                source.into(),
                position.into(),
                size.into(),
                tint.into(),
            )
        }
    }

    /// Draw a billboard texture defined by source and rotation
    fn draw_billboard_pro(
        &mut self,
        camera: Camera,
        texture: Texture2D,
        source: Rectangle,
        position: Vector3,
        up: Vector3,
        size: Vector2,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    ) {
        unsafe {
            ffi::DrawBillboardPro(
                camera.into(),
                texture.raw.deref().clone(),
                source.into(),
                position.into(),
                up.into(),
                size.into(),
                origin.into(),
                rotation,
                tint.into(),
            )
        }
    }

    /// Draw a 3d mesh with material and transform
    fn draw_mesh(&mut self, mesh: &Mesh, material: &Material, transform: Matrix) {
        unsafe { ffi::DrawMesh(mesh.raw.clone(), material.raw.clone(), transform.into()) }
    }

    /// Draw multiple mesh instances with material and different transforms
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
