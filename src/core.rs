use crate::{color::Color, drawing::DrawHandle, ffi, math::Vector2, texture::Image};

use std::{
    ffi::{CStr, CString},
    time::Duration,
};

pub use ffi::{
    ConfigFlags, GamepadAxis, GamepadButton, Gesture, KeyboardKey, MouseButton, MouseCursor,
};

/// Main raylib handle
#[derive(Debug)]
pub struct Raylib(std::marker::PhantomData<*const ()>);

impl Raylib {
    /// Initialize window and OpenGL context
    #[inline]
    pub fn init_window(width: u32, height: u32, title: &str) -> Option<Self> {
        let title = CString::new(title).unwrap();

        unsafe {
            ffi::InitWindow(width as _, height as _, title.as_ptr());
        }

        if unsafe { ffi::IsWindowReady() } {
            Some(Self(std::marker::PhantomData))
        } else {
            None
        }
    }

    /// Initialize window and OpenGL context with config flags
    #[inline]
    pub fn init_window_ex(
        width: u32,
        height: u32,
        title: &str,
        flags: ConfigFlags,
    ) -> Option<Self> {
        unsafe {
            ffi::SetConfigFlags(flags.bits());
        }

        Self::init_window(width, height, title)
    }

    /// Check if Escape key or Close icon is pressed
    #[inline]
    pub fn window_should_close(&self) -> bool {
        unsafe { ffi::WindowShouldClose() }
    }

    /// Close window and unload OpenGL context
    #[inline]
    pub fn close_window(self) {
        drop(self)
    }

    /// Check if window is currently fullscreen
    #[inline]
    pub fn is_window_fullscreen(&self) -> bool {
        unsafe { ffi::IsWindowFullscreen() }
    }

    /// Check if window is currently hidden (only PLATFORM_DESKTOP)
    #[inline]
    pub fn is_window_hidden(&self) -> bool {
        unsafe { ffi::IsWindowHidden() }
    }

    /// Check if window is currently minimized (only PLATFORM_DESKTOP)
    #[inline]
    pub fn is_window_minimized(&self) -> bool {
        unsafe { ffi::IsWindowMinimized() }
    }

    /// Check if window is currently maximized (only PLATFORM_DESKTOP)
    #[inline]
    pub fn is_window_maximized(&self) -> bool {
        unsafe { ffi::IsWindowMaximized() }
    }

    /// Check if window is currently focused (only PLATFORM_DESKTOP)
    #[inline]
    pub fn is_window_focused(&self) -> bool {
        unsafe { ffi::IsWindowFocused() }
    }

    /// Check if window has been resized last frame
    #[inline]
    pub fn is_window_resized(&self) -> bool {
        unsafe { ffi::IsWindowResized() }
    }

    /// Check if one specific window flag is enabled
    #[inline]
    pub fn is_window_state(&self, flag: ConfigFlags) -> bool {
        unsafe { ffi::IsWindowState(flag.bits()) }
    }

    /// Set window configuration state using flags (only PLATFORM_DESKTOP)
    #[inline]
    pub fn set_window_state(&mut self, flags: ConfigFlags) {
        unsafe { ffi::SetWindowState(flags.bits()) }
    }

    /// Clear window configuration state flags
    #[inline]
    pub fn clear_window_state(&mut self, flags: ConfigFlags) {
        unsafe { ffi::ClearWindowState(flags.bits()) }
    }

    /// Toggle window state: fullscreen/windowed (only PLATFORM_DESKTOP)
    #[inline]
    pub fn toggle_fullscreen(&mut self) {
        unsafe { ffi::ToggleFullscreen() }
    }

    /// Set window state: maximized, if resizable (only PLATFORM_DESKTOP)
    #[inline]
    pub fn maximize_window(&mut self) {
        unsafe { ffi::MaximizeWindow() }
    }

    /// Set window state: minimized, if resizable (only PLATFORM_DESKTOP)
    #[inline]
    pub fn minimize_window(&mut self) {
        unsafe { ffi::MinimizeWindow() }
    }

    /// Set window state: not minimized/maximized (only PLATFORM_DESKTOP)
    #[inline]
    pub fn restore_window(&mut self) {
        unsafe { ffi::RestoreWindow() }
    }

    /// Set icon for window (single image, RGBA 32bit, only PLATFORM_DESKTOP)
    #[inline]
    pub fn set_window_icon(&mut self, image: &Image) {
        unsafe { ffi::SetWindowIcon(image.raw.clone()) }
    }

    /// Set icon for window (multiple images, RGBA 32bit, only PLATFORM_DESKTOP)
    #[inline]
    pub fn set_window_icons(&mut self, images: &[&Image]) {
        let mut images: Vec<_> = images.iter().map(|img| img.raw.clone()).collect();

        unsafe { ffi::SetWindowIcons(images.as_mut_ptr(), images.len() as _) }
    }

    /// Set title for window (only PLATFORM_DESKTOP)
    #[inline]
    pub fn set_window_title(&mut self, title: &str) {
        let title = CString::new(title).unwrap();

        unsafe { ffi::SetWindowTitle(title.as_ptr()) }
    }

    /// Set window position on screen (only PLATFORM_DESKTOP)
    #[inline]
    pub fn set_window_position(&mut self, x: i32, y: i32) {
        unsafe { ffi::SetWindowPosition(x, y) }
    }

    /// Set monitor for the current window (fullscreen mode)
    #[inline]
    pub fn set_window_monitor(&mut self, monitor: u32) {
        unsafe { ffi::SetWindowMonitor(monitor as _) }
    }

    /// Set window minimum dimensions (for FLAG_WINDOW_RESIZABLE)
    #[inline]
    pub fn set_window_min_size(&mut self, width: u32, height: u32) {
        unsafe { ffi::SetWindowMinSize(width as _, height as _) }
    }

    /// Set window dimensions
    #[inline]
    pub fn set_window_size(&mut self, width: u32, height: u32) {
        unsafe { ffi::SetWindowSize(width as _, height as _) }
    }

    /// Set window opacity [0.0f..1.0f] (only PLATFORM_DESKTOP)
    #[inline]
    pub fn set_window_opacity(&mut self, opacity: f32) {
        unsafe { ffi::SetWindowOpacity(opacity) }
    }

    /// Get native window handle
    #[inline]
    pub unsafe fn get_window_handle(&self) -> *mut core::ffi::c_void {
        ffi::GetWindowHandle()
    }

    /// Get current screen width
    #[inline]
    pub fn get_screen_width(&self) -> u32 {
        unsafe { ffi::GetScreenWidth() as _ }
    }

    /// Get current screen height
    #[inline]
    pub fn get_screen_height(&self) -> u32 {
        unsafe { ffi::GetScreenHeight() as _ }
    }

    /// Get current render width (it considers HiDPI)
    #[inline]
    pub fn get_render_width(&self) -> u32 {
        unsafe { ffi::GetRenderWidth() as _ }
    }

    /// Get current render height (it considers HiDPI)
    #[inline]
    pub fn get_render_height(&self) -> u32 {
        unsafe { ffi::GetRenderHeight() as _ }
    }

    /// Get number of connected monitors
    #[inline]
    pub fn get_monitor_count(&self) -> u32 {
        unsafe { ffi::GetMonitorCount() as _ }
    }

    /// Get current connected monitor
    #[inline]
    pub fn get_current_monitor(&self) -> u32 {
        unsafe { ffi::GetCurrentMonitor() as _ }
    }

    /// Get specified monitor position
    #[inline]
    pub fn get_monitor_position(&self, monitor: u32) -> Vector2 {
        unsafe { ffi::GetMonitorPosition(monitor as _).into() }
    }

    /// Get specified monitor width (current video mode used by monitor)
    #[inline]
    pub fn get_monitor_width(&self, monitor: u32) -> u32 {
        unsafe { ffi::GetMonitorWidth(monitor as _) as _ }
    }

    /// Get specified monitor height (current video mode used by monitor)
    #[inline]
    pub fn get_monitor_height(&self, monitor: u32) -> u32 {
        unsafe { ffi::GetMonitorHeight(monitor as _) as _ }
    }

    /// Get specified monitor physical width in millimetres
    #[inline]
    pub fn get_monitor_physical_width(&self, monitor: u32) -> u32 {
        unsafe { ffi::GetMonitorPhysicalWidth(monitor as _) as _ }
    }

    /// Get specified monitor physical height in millimetres
    #[inline]
    pub fn get_monitor_physical_height(&self, monitor: u32) -> u32 {
        unsafe { ffi::GetMonitorPhysicalHeight(monitor as _) as _ }
    }

    /// Get specified monitor refresh rate
    #[inline]
    pub fn get_monitor_refresh_rate(&self, monitor: u32) -> u32 {
        unsafe { ffi::GetMonitorRefreshRate(monitor as _) as _ }
    }

    /// Get window position XY on monitor
    #[inline]
    pub fn get_window_position(&self) -> Vector2 {
        unsafe { ffi::GetWindowPosition().into() }
    }

    /// Get window scale DPI factor
    #[inline]
    pub fn get_window_scale_dpi(&self) -> Vector2 {
        unsafe { ffi::GetWindowScaleDPI().into() }
    }

    /// Get the human-readable, UTF-8 encoded name of the primary monitor
    #[inline]
    pub fn get_monitor_name(&self, monitor: u32) -> String {
        let name = unsafe { ffi::GetMonitorName(monitor as _) };

        if name.is_null() {
            String::new()
        } else {
            unsafe { CStr::from_ptr(name) }
                .to_string_lossy()
                .into_owned()
        }
    }

    /// Set clipboard text content
    #[inline]
    pub fn set_clipboard_text(&mut self, text: &str) {
        let text = CString::new(text).unwrap();

        unsafe { ffi::SetClipboardText(text.as_ptr()) }
    }

    /// Get clipboard text content
    #[inline]
    pub fn get_clipboard_text(&self) -> String {
        let text = unsafe { ffi::GetClipboardText() };

        if text.is_null() {
            String::new()
        } else {
            unsafe { CStr::from_ptr(text) }
                .to_string_lossy()
                .into_owned()
        }
    }

    /// Enable waiting for events on EndDrawing(), no automatic event polling
    #[inline]
    pub fn enable_event_waiting(&mut self) {
        unsafe { ffi::EnableEventWaiting() }
    }

    /// Disable waiting for events on EndDrawing(), automatic events polling
    #[inline]
    pub fn disable_event_waiting(&mut self) {
        unsafe { ffi::DisableEventWaiting() }
    }

    /// Swap back buffer with front buffer (screen drawing)
    /// NOTE: Those functions are intended for advance users that want full control over the frame processing
    #[inline]
    pub fn swap_screen_buffer(&mut self) {
        unsafe { ffi::SwapScreenBuffer() }
    }

    /// Register all input events
    /// NOTE: Those functions are intended for advance users that want full control over the frame processing
    #[inline]
    pub fn poll_input_events(&mut self) {
        unsafe { ffi::PollInputEvents() }
    }

    /// Wait for some time (halt program execution)
    /// NOTE: Those functions are intended for advance users that want full control over the frame processing
    #[inline]
    pub fn wait_time(&mut self, duration: Duration) {
        unsafe { ffi::WaitTime(duration.as_secs_f64()) }
    }

    /// Shows cursor
    #[inline]
    pub fn show_cursor(&mut self) {
        unsafe { ffi::ShowCursor() }
    }

    /// Hides cursor
    #[inline]
    pub fn hide_cursor(&mut self) {
        unsafe { ffi::HideCursor() }
    }

    /// Check if cursor is not visible
    #[inline]
    pub fn is_cursor_hidden(&self) -> bool {
        unsafe { ffi::IsCursorHidden() }
    }

    /// Enables cursor (unlock cursor)
    #[inline]
    pub fn enable_cursor(&mut self) {
        unsafe { ffi::EnableCursor() }
    }

    /// Disables cursor (lock cursor)
    #[inline]
    pub fn disable_cursor(&mut self) {
        unsafe { ffi::DisableCursor() }
    }

    /// Check if cursor is on the screen
    #[inline]
    pub fn is_cursor_on_screen(&self) -> bool {
        unsafe { ffi::IsCursorOnScreen() }
    }

    /// Set background color (framebuffer clear color)
    #[inline]
    pub fn clear_background(&mut self, color: Color) {
        unsafe { ffi::ClearBackground(color.into()) }
    }

    /// Set target FPS (maximum)
    #[inline]
    pub fn set_target_fps(&mut self, fps: u32) {
        unsafe { ffi::SetTargetFPS(fps as _) }
    }

    /// Get current FPS
    #[inline]
    pub fn get_fps(&self) -> u32 {
        unsafe { ffi::GetFPS() as _ }
    }

    /// Get time for last frame drawn (delta time)
    #[inline]
    pub fn get_frame_time(&self) -> Duration {
        Duration::from_secs_f32(unsafe { ffi::GetFrameTime() })
    }

    /// Get elapsed time since InitWindow()
    #[inline]
    pub fn get_time(&self) -> Duration {
        Duration::from_secs_f64(unsafe { ffi::GetTime() })
    }

    /// Get a random value between min and max (both included)
    #[inline]
    pub fn get_random_value(&self, min: i32, max: i32) -> i32 {
        unsafe { ffi::GetRandomValue(min, max) }
    }

    /// Set the seed for the random number generator
    #[inline]
    pub fn set_random_seed(&mut self, seed: u32) {
        unsafe { ffi::SetRandomSeed(seed) }
    }

    /// Takes a screenshot of current screen (file_name extension defines format)
    #[inline]
    pub fn take_screenshot(&mut self, file_name: &str) {
        let file_name = CString::new(file_name).unwrap();

        unsafe { ffi::TakeScreenshot(file_name.as_ptr()) }
    }

    /// Open URL with default system browser (if available)
    #[inline]
    pub fn open_url(&self, url: &str) {
        let url = CString::new(url).unwrap();

        unsafe { ffi::OpenURL(url.as_ptr()) }
    }

    /// Check if a file has been dropped into window
    #[inline]
    pub fn is_file_dropped(&self) -> bool {
        unsafe { ffi::IsFileDropped() }
    }

    /// Load dropped filepaths
    #[inline]
    pub fn get_dropped_files(&self) -> Vec<String> {
        let path_list = unsafe { ffi::LoadDroppedFiles() };
        let mut paths = Vec::new();

        for i in 0..(path_list.count as usize) {
            let path = unsafe { CStr::from_ptr(path_list.paths.add(i).read()) };

            paths.push(path.to_string_lossy().into_owned());
        }

        unsafe {
            ffi::UnloadDroppedFiles(path_list);
        }

        paths
    }

    /// Check if a key has been pressed once
    #[inline]
    pub fn is_key_pressed(&self, key: KeyboardKey) -> bool {
        unsafe { ffi::IsKeyPressed(key as _) }
    }

    /// Check if a key is being pressed
    #[inline]
    pub fn is_key_down(&self, key: KeyboardKey) -> bool {
        unsafe { ffi::IsKeyDown(key as _) }
    }

    /// Check if a key has been released once
    #[inline]
    pub fn is_key_released(&self, key: KeyboardKey) -> bool {
        unsafe { ffi::IsKeyReleased(key as _) }
    }

    /// Check if a key is NOT being pressed
    #[inline]
    pub fn is_key_up(&self, key: KeyboardKey) -> bool {
        unsafe { ffi::IsKeyUp(key as _) }
    }

    /// Set a custom key to exit program (default is ESC)
    #[inline]
    pub fn set_exit_key(&mut self, key: KeyboardKey) {
        unsafe { ffi::SetExitKey(key as _) }
    }

    /// Get key pressed (keycode), call it multiple times for keys queued, returns [`KeyboardKey::Null`] when the queue is empty
    #[inline]
    pub fn get_key_pressed(&self) -> KeyboardKey {
        unsafe { std::mem::transmute(ffi::GetKeyPressed()) }
    }

    /// Get char pressed (unicode), call it multiple times for chars queued, returns `None` when the queue is empty
    #[inline]
    pub fn get_char_pressed(&self) -> Option<char> {
        let ch = unsafe { ffi::GetCharPressed() as u32 };

        if ch != 0 {
            char::from_u32(ch)
        } else {
            None
        }
    }

    /// Check if a gamepad is available
    #[inline]
    pub fn is_gamepad_available(&self, gamepad: u32) -> bool {
        unsafe { ffi::IsGamepadAvailable(gamepad as _) }
    }

    /// Get gamepad internal name id
    #[inline]
    pub fn get_gamepad_name(&self, gamepad: u32) -> String {
        let name = unsafe { ffi::GetGamepadName(gamepad as _) };

        if !name.is_null() {
            let name = unsafe { CStr::from_ptr(name) };

            name.to_string_lossy().into_owned()
        } else {
            String::new()
        }
    }

    /// Check if a gamepad button has been pressed once
    #[inline]
    pub fn is_gamepad_button_pressed(&self, gamepad: u32, button: GamepadButton) -> bool {
        unsafe { ffi::IsGamepadButtonPressed(gamepad as _, button as _) }
    }

    /// Check if a gamepad button is being pressed
    #[inline]
    pub fn is_gamepad_button_down(&self, gamepad: u32, button: GamepadButton) -> bool {
        unsafe { ffi::IsGamepadButtonDown(gamepad as _, button as _) }
    }

    /// Check if a gamepad button has been released once
    #[inline]
    pub fn is_gamepad_button_released(&self, gamepad: u32, button: GamepadButton) -> bool {
        unsafe { ffi::IsGamepadButtonReleased(gamepad as _, button as _) }
    }

    /// Check if a gamepad button is NOT being pressed
    #[inline]
    pub fn is_gamepad_button_up(&self, gamepad: u32, button: GamepadButton) -> bool {
        unsafe { ffi::IsGamepadButtonUp(gamepad as _, button as _) }
    }

    /// Get the last gamepad button pressed
    #[inline]
    pub fn get_gamepad_button_pressed(&self) -> GamepadButton {
        unsafe { std::mem::transmute(ffi::GetGamepadButtonPressed()) }
    }

    /// Get gamepad axis count for a gamepad
    #[inline]
    pub fn get_gamepad_axis_count(&self, gamepad: u32) -> u32 {
        unsafe { ffi::GetGamepadAxisCount(gamepad as _) as _ }
    }

    /// Get axis movement value for a gamepad axis
    #[inline]
    pub fn get_gamepad_axis_movement(&self, gamepad: u32, axis: GamepadAxis) -> f32 {
        unsafe { ffi::GetGamepadAxisMovement(gamepad as _, axis as _) }
    }

    /// Set internal gamepad mappings (SDL_GameControllerDB)
    #[inline]
    pub fn set_gamepad_mappings(&mut self, mappings: &str) -> i32 {
        let mappings = CString::new(mappings).unwrap();

        unsafe { ffi::SetGamepadMappings(mappings.as_ptr()) }
    }

    /// Check if a mouse button has been pressed once
    #[inline]
    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        unsafe { ffi::IsMouseButtonPressed(button as _) }
    }

    /// Check if a mouse button is being pressed
    #[inline]
    pub fn is_mouse_button_down(&self, button: MouseButton) -> bool {
        unsafe { ffi::IsMouseButtonDown(button as _) }
    }

    /// Check if a mouse button has been released once
    #[inline]
    pub fn is_mouse_button_released(&self, button: MouseButton) -> bool {
        unsafe { ffi::IsMouseButtonReleased(button as _) }
    }

    /// Check if a mouse button is NOT being pressed
    #[inline]
    pub fn is_mouse_button_up(&self, button: MouseButton) -> bool {
        unsafe { ffi::IsMouseButtonUp(button as _) }
    }

    /// Get mouse position X
    #[inline]
    pub fn get_mouse_x(&self) -> i32 {
        unsafe { ffi::GetMouseX() }
    }

    /// Get mouse position Y
    #[inline]
    pub fn get_mouse_y(&self) -> i32 {
        unsafe { ffi::GetMouseY() }
    }

    /// Get mouse position XY
    #[inline]
    pub fn get_mouse_position(&self) -> Vector2 {
        unsafe { ffi::GetMousePosition().into() }
    }

    /// Get mouse delta between frames
    #[inline]
    pub fn get_mouse_delta(&self) -> Vector2 {
        unsafe { ffi::GetMouseDelta().into() }
    }

    /// Set mouse position XY
    #[inline]
    pub fn set_mouse_position(&mut self, x: i32, y: i32) {
        unsafe { ffi::SetMousePosition(x, y) }
    }

    /// Set mouse offset
    #[inline]
    pub fn set_mouse_offset(&mut self, offset_x: i32, offset_y: i32) {
        unsafe { ffi::SetMouseOffset(offset_x, offset_y) }
    }

    /// Set mouse scaling
    #[inline]
    pub fn set_mouse_scale(&mut self, scale_x: f32, scale_y: f32) {
        unsafe { ffi::SetMouseScale(scale_x, scale_y) }
    }

    /// Get mouse wheel movement for X or Y, whichever is larger
    #[inline]
    pub fn get_mouse_wheel_move(&self) -> f32 {
        unsafe { ffi::GetMouseWheelMove() }
    }

    /// Get mouse wheel movement for both X and Y
    #[inline]
    pub fn get_mouse_wheel_move_vec(&self) -> Vector2 {
        unsafe { ffi::GetMouseWheelMoveV().into() }
    }

    /// Set mouse cursor
    #[inline]
    pub fn set_mouse_cursor(&mut self, cursor: MouseCursor) {
        unsafe { ffi::SetMouseCursor(cursor as _) }
    }

    /// Get touch position X for touch point 0 (relative to screen size)
    #[inline]
    pub fn get_touch_x(&self) -> i32 {
        unsafe { ffi::GetTouchX() }
    }

    /// Get touch position Y for touch point 0 (relative to screen size)
    #[inline]
    pub fn get_touch_y(&self) -> i32 {
        unsafe { ffi::GetTouchY() }
    }

    /// Get touch position XY for a touch point index (relative to screen size)
    #[inline]
    pub fn get_touch_position(&self, index: u32) -> Vector2 {
        unsafe { ffi::GetTouchPosition(index as _).into() }
    }

    /// Get touch point identifier for given index
    #[inline]
    pub fn get_touch_point_id(&self, index: u32) -> u32 {
        unsafe { ffi::GetTouchPointId(index as _) as _ }
    }

    /// Get number of touch points
    #[inline]
    pub fn get_touch_point_count(&self) -> u32 {
        unsafe { ffi::GetTouchPointCount() as _ }
    }

    /// Enable a set of gestures using flags
    #[inline]
    pub fn set_gestures_enabled(&mut self, flags: Gesture) {
        unsafe { ffi::SetGesturesEnabled(flags.bits()) }
    }

    /// Check if a gesture have been detected
    #[inline]
    pub fn is_gesture_detected(&self, gesture: Gesture) -> bool {
        unsafe { ffi::IsGestureDetected(gesture.bits() as _) }
    }

    /// Get latest detected gesture
    #[inline]
    pub fn get_gesture_detected(&self) -> Gesture {
        unsafe { Gesture(ffi::GetGestureDetected() as _) }
    }

    /// Get gesture hold time
    #[inline]
    pub fn get_gesture_hold_duration(&self) -> Duration {
        Duration::from_micros(unsafe { ffi::GetGestureHoldDuration() * 1000. } as u64)
    }

    /// Get gesture drag vector
    #[inline]
    pub fn get_gesture_drag_vector(&self) -> Vector2 {
        unsafe { ffi::GetGestureDragVector().into() }
    }

    /// Get gesture drag angle
    #[inline]
    pub fn get_gesture_drag_angle(&self) -> f32 {
        unsafe { ffi::GetGestureDragAngle() }
    }

    /// Get gesture pinch delta
    #[inline]
    pub fn get_gesture_pinch_vector(&self) -> Vector2 {
        unsafe { ffi::GetGesturePinchVector().into() }
    }

    /// Get gesture pinch angle
    #[inline]
    pub fn get_gesture_pinch_angle(&self) -> f32 {
        unsafe { ffi::GetGesturePinchAngle() }
    }

    /// Setup canvas (framebuffer) to start drawing
    #[inline]
    pub fn begin_drawing(&mut self) -> DrawHandle {
        unsafe {
            ffi::BeginDrawing();
        }

        DrawHandle(self)
    }
}

impl Drop for Raylib {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::CloseWindow() }
    }
}
