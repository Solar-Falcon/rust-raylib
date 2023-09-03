use crate::{color::Color, ffi, math::Vector2, texture::Image};

use std::ffi::{CStr, CString};

// TODO: move Texture::draw_* funcs to Draw trait in drawing.rs

pub use ffi::{
    ConfigFlags, GamepadAxis, GamepadButton, Gesture, KeyboardKey, MouseButton, MouseCursor,
};

/// Main raylib handle
#[derive(Debug)]
pub struct Raylib(());

impl Raylib {
    /// Initialize window and OpenGL context
    #[inline]
    pub fn init_window(width: u32, height: u32, title: &str) -> Self {
        let title = CString::new(title).unwrap();

        unsafe {
            ffi::InitWindow(width as _, height as _, title.as_ptr());
        }

        Self(())
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

    /// Check if window has been initialized successfully
    #[inline]
    pub fn is_window_ready(&self) -> bool {
        unsafe { ffi::IsWindowReady() }
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
    pub fn wait_time(&mut self, seconds: f64) {
        unsafe { ffi::WaitTime(seconds) }
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
    /*
        /// Set target FPS (maximum)
        #[inline]
        pub fn SetTargetFPS(fps: u32) {}

        /// Get current FPS
        #[inline]
        pub fn GetFPS() -> u32;

        /// Get time in seconds for last frame drawn (delta time)
        #[inline]
        pub fn GetFrameTime() -> f32;

        /// Get elapsed time in seconds since InitWindow()
        #[inline]
        pub fn GetTime() -> core::ffi::c_double;

        /// Get a random value between min and max (both included)
        #[inline]
        pub fn GetRandomValue(min: u32, max: u32) -> u32;

        /// Set the seed for the random number generator
        #[inline]
        pub fn SetRandomSeed(seed: core::ffi::c_uint) {}

        /// Takes a screenshot of current screen (filename extension defines format)
        #[inline]
        pub fn TakeScreenshot(fileName: *const core::ffi::c_char) {}

        /// Setup init configuration flags (view FLAGS)
        #[inline]
        pub fn SetConfigFlags(flags: core::ffi::c_uint) {}

        /// Show trace log messages (LOG_DEBUG, LOG_INFO, LOG_WARNING, LOG_ERROR...)
        #[inline]
        pub fn TraceLog(logLevel: u32, text: *const core::ffi::c_char, ...) {}

        /// Set the current threshold (minimum) log level
        #[inline]
        pub fn SetTraceLogLevel(logLevel: u32) {}

        /// Internal memory allocator
        #[inline]
        pub fn MemAlloc(size: core::ffi::c_uint) -> *mut core::ffi::c_void;

        /// Internal memory reallocator
        #[inline]
        pub fn MemRealloc(ptr: *mut core::ffi::c_void, size: core::ffi::c_uint) -> *mut core::ffi::c_void;

        /// Internal memory free
        #[inline]
        pub fn MemFree(ptr: *mut core::ffi::c_void) {}

        /// Open URL with default system browser (if available)
        #[inline]
        pub fn OpenURL(url: *const core::ffi::c_char) {}

        /// Set custom trace log
        #[inline]
        pub fn SetTraceLogCallback(callback: TraceLogCallback) {}

        /// Set custom file binary data loader
        #[inline]
        pub fn SetLoadFileDataCallback(callback: LoadFileDataCallback) {}

        /// Set custom file binary data saver
        #[inline]
        pub fn SetSaveFileDataCallback(callback: SaveFileDataCallback) {}

        /// Set custom file text data loader
        #[inline]
        pub fn SetLoadFileTextCallback(callback: LoadFileTextCallback) {}

        /// Set custom file text data saver
        #[inline]
        pub fn SetSaveFileTextCallback(callback: SaveFileTextCallback) {}

        /// Load file data as byte array (read)
        #[inline]
        pub fn LoadFileData(fileName: *const core::ffi::c_char, bytesRead: *mut core::ffi::c_uint) -> *mut core::ffi::c_uchar;

        /// Unload file data allocated by LoadFileData()
        #[inline]
        pub fn UnloadFileData(data: *mut core::ffi::c_uchar) {}

        /// Save data to file from byte array (write), returns true on success
        #[inline]
        pub fn SaveFileData(fileName: *const core::ffi::c_char, data: *mut core::ffi::c_void, bytesToWrite: core::ffi::c_uint) -> bool;

        /// Export data to code (.h), returns true on success
        #[inline]
        pub fn ExportDataAsCode(data: *const core::ffi::c_uchar, size: core::ffi::c_uint, fileName: *const core::ffi::c_char) -> bool;

        /// Load text data from file (read), returns a '\0' terminated string
        #[inline]
        pub fn LoadFileText(fileName: *const core::ffi::c_char) -> *mut core::ffi::c_char;

        /// Unload file text data allocated by LoadFileText()
        #[inline]
        pub fn UnloadFileText(text: *mut core::ffi::c_char) {}

        /// Save text data to file (write), string must be '\0' terminated, returns true on success
        #[inline]
        pub fn SaveFileText(fileName: *const core::ffi::c_char, text: *mut core::ffi::c_char) -> bool;

        /// Check if file exists
        #[inline]
        pub fn FileExists(fileName: *const core::ffi::c_char) -> bool;

        /// Check if a directory path exists
        #[inline]
        pub fn DirectoryExists(dirPath: *const core::ffi::c_char) -> bool;

        /// Check file extension (including point: .png, .wav)
        #[inline]
        pub fn IsFileExtension(fileName: *const core::ffi::c_char, ext: *const core::ffi::c_char) -> bool;

        /// Get file length in bytes (NOTE: GetFileSize() conflicts with windows.h)
        #[inline]
        pub fn GetFileLength(fileName: *const core::ffi::c_char) -> u32;

        /// Get pointer to extension for a filename string (includes dot: '.png')
        #[inline]
        pub fn GetFileExtension(fileName: *const core::ffi::c_char) -> *const core::ffi::c_char;

        /// Get pointer to filename for a path string
        #[inline]
        pub fn GetFileName(filePath: *const core::ffi::c_char) -> *const core::ffi::c_char;

        /// Get filename string without extension (uses static string)
        #[inline]
        pub fn GetFileNameWithoutExt(filePath: *const core::ffi::c_char) -> *const core::ffi::c_char;

        /// Get full path for a given fileName with path (uses static string)
        #[inline]
        pub fn GetDirectoryPath(filePath: *const core::ffi::c_char) -> *const core::ffi::c_char;

        /// Get previous directory path for a given path (uses static string)
        #[inline]
        pub fn GetPrevDirectoryPath(dirPath: *const core::ffi::c_char) -> *const core::ffi::c_char;

        /// Get current working directory (uses static string)
        #[inline]
        pub fn GetWorkingDirectory() -> *const core::ffi::c_char;

        /// Get the directory if the running application (uses static string)
        #[inline]
        pub fn GetApplicationDirectory() -> *const core::ffi::c_char;

        /// Change working directory, return true on success
        #[inline]
        pub fn ChangeDirectory(dir: *const core::ffi::c_char) -> bool;

        /// Check if a given path is a file or a directory
        #[inline]
        pub fn IsPathFile(path: *const core::ffi::c_char) -> bool;

        /// Load directory filepaths
        #[inline]
        pub fn LoadDirectoryFiles(dirPath: *const core::ffi::c_char) -> FilePathList;

        /// Load directory filepaths with extension filtering and recursive directory scan
        #[inline]
        pub fn LoadDirectoryFilesEx(basePath: *const core::ffi::c_char, filter: *const core::ffi::c_char, scanSubdirs: bool) -> FilePathList;

        /// Unload filepaths
        #[inline]
        pub fn UnloadDirectoryFiles(files: FilePathList) {}

        /// Check if a file has been dropped into window
        #[inline]
        pub fn IsFileDropped() -> bool;

        /// Load dropped filepaths
        #[inline]
        pub fn LoadDroppedFiles() -> FilePathList;

        /// Unload dropped filepaths
        #[inline]
        pub fn UnloadDroppedFiles(files: FilePathList) {}

        /// Get file modification time (last write time)
        #[inline]
        pub fn GetFileModTime(fileName: *const core::ffi::c_char) -> core::ffi::c_long;

        /// Compress data (DEFLATE algorithm), memory must be MemFree()
        #[inline]
        pub fn CompressData(data: *const core::ffi::c_uchar, dataSize: u32, compDataSize: *mut u32) -> *mut core::ffi::c_uchar;

        /// Decompress data (DEFLATE algorithm), memory must be MemFree()
        #[inline]
        pub fn DecompressData(compData: *const core::ffi::c_uchar, compDataSize: u32, dataSize: *mut u32) -> *mut core::ffi::c_uchar;

        /// Encode data to Base64 string, memory must be MemFree()
        #[inline]
        pub fn EncodeDataBase64(data: *const core::ffi::c_uchar, dataSize: u32, outputSize: *mut u32) -> *mut core::ffi::c_char;

        /// Decode Base64 string data, memory must be MemFree()
        #[inline]
        pub fn DecodeDataBase64(data: *const core::ffi::c_uchar, outputSize: *mut u32) -> *mut core::ffi::c_uchar;

        /// Check if a key has been pressed once
        #[inline]
        pub fn IsKeyPressed(key: u32) -> bool;

        /// Check if a key is being pressed
        #[inline]
        pub fn IsKeyDown(key: u32) -> bool;

        /// Check if a key has been released once
        #[inline]
        pub fn IsKeyReleased(key: u32) -> bool;

        /// Check if a key is NOT being pressed
        #[inline]
        pub fn IsKeyUp(key: u32) -> bool;

        /// Set a custom key to exit program (default is ESC)
        #[inline]
        pub fn SetExitKey(key: u32) {}

        /// Get key pressed (keycode), call it multiple times for keys queued, returns 0 when the queue is empty
        #[inline]
        pub fn GetKeyPressed() -> u32;

        /// Get char pressed (unicode), call it multiple times for chars queued, returns 0 when the queue is empty
        #[inline]
        pub fn GetCharPressed() -> u32;

        /// Check if a gamepad is available
        #[inline]
        pub fn IsGamepadAvailable(gamepad: u32) -> bool;

        /// Get gamepad internal name id
        #[inline]
        pub fn GetGamepadName(gamepad: u32) -> *const core::ffi::c_char;

        /// Check if a gamepad button has been pressed once
        #[inline]
        pub fn IsGamepadButtonPressed(gamepad: u32, button: u32) -> bool;

        /// Check if a gamepad button is being pressed
        #[inline]
        pub fn IsGamepadButtonDown(gamepad: u32, button: u32) -> bool;

        /// Check if a gamepad button has been released once
        #[inline]
        pub fn IsGamepadButtonReleased(gamepad: u32, button: u32) -> bool;

        /// Check if a gamepad button is NOT being pressed
        #[inline]
        pub fn IsGamepadButtonUp(gamepad: u32, button: u32) -> bool;

        /// Get the last gamepad button pressed
        #[inline]
        pub fn GetGamepadButtonPressed() -> u32;

        /// Get gamepad axis count for a gamepad
        #[inline]
        pub fn GetGamepadAxisCount(gamepad: u32) -> u32;

        /// Get axis movement value for a gamepad axis
        #[inline]
        pub fn GetGamepadAxisMovement(gamepad: u32, axis: u32) -> f32;

        /// Set internal gamepad mappings (SDL_GameControllerDB)
        #[inline]
        pub fn SetGamepadMappings(mappings: *const core::ffi::c_char) -> u32;

        /// Check if a mouse button has been pressed once
        #[inline]
        pub fn IsMouseButtonPressed(button: u32) -> bool;

        /// Check if a mouse button is being pressed
        #[inline]
        pub fn IsMouseButtonDown(button: u32) -> bool;

        /// Check if a mouse button has been released once
        #[inline]
        pub fn IsMouseButtonReleased(button: u32) -> bool;

        /// Check if a mouse button is NOT being pressed
        #[inline]
        pub fn IsMouseButtonUp(button: u32) -> bool;

        /// Get mouse position X
        #[inline]
        pub fn GetMouseX() -> u32;

        /// Get mouse position Y
        #[inline]
        pub fn GetMouseY() -> u32;

        /// Get mouse position XY
        #[inline]
        pub fn GetMousePosition() -> Vector2;

        /// Get mouse delta between frames
        #[inline]
        pub fn GetMouseDelta() -> Vector2;

        /// Set mouse position XY
        #[inline]
        pub fn SetMousePosition(x: u32, y: u32) {}

        /// Set mouse offset
        #[inline]
        pub fn SetMouseOffset(offsetX: u32, offsetY: u32) {}

        /// Set mouse scaling
        #[inline]
        pub fn SetMouseScale(scaleX: f32, scaleY: f32) {}

        /// Get mouse wheel movement for X or Y, whichever is larger
        #[inline]
        pub fn GetMouseWheelMove() -> f32;

        /// Get mouse wheel movement for both X and Y
        #[inline]
        pub fn GetMouseWheelMoveV() -> Vector2;

        /// Set mouse cursor
        #[inline]
        pub fn SetMouseCursor(cursor: u32) {}

        /// Get touch position X for touch point 0 (relative to screen size)
        #[inline]
        pub fn GetTouchX() -> u32;

        /// Get touch position Y for touch point 0 (relative to screen size)
        #[inline]
        pub fn GetTouchY() -> u32;

        /// Get touch position XY for a touch point index (relative to screen size)
        #[inline]
        pub fn GetTouchPosition(index: u32) -> Vector2;

        /// Get touch point identifier for given index
        #[inline]
        pub fn GetTouchPointId(index: u32) -> u32;

        /// Get number of touch points
        #[inline]
        pub fn GetTouchPointCount() -> u32;

        /// Enable a set of gestures using flags
        #[inline]
        pub fn SetGesturesEnabled(flags: core::ffi::c_uint) {}

        /// Check if a gesture have been detected
        #[inline]
        pub fn IsGestureDetected(gesture: u32) -> bool;

        /// Get latest detected gesture
        #[inline]
        pub fn GetGestureDetected() -> u32;

        /// Get gesture hold time in milliseconds
        #[inline]
        pub fn GetGestureHoldDuration() -> f32;

        /// Get gesture drag vector
        #[inline]
        pub fn GetGestureDragVector() -> Vector2;

        /// Get gesture drag angle
        #[inline]
        pub fn GetGestureDragAngle() -> f32;

        /// Get gesture pinch delta
        #[inline]
        pub fn GetGesturePinchVector() -> Vector2;

        /// Get gesture pinch angle
        #[inline]
        pub fn GetGesturePinchAngle() -> f32;
    // */
}

impl Drop for Raylib {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::CloseWindow() }
    }
}
