use crate::ffi;

use std::ffi::CString;

use static_assertions::{assert_eq_align, assert_eq_size};

// <temporary>
// remaining enum imports
// TODO: move to respective modules
// pub use ffi::{
//     BlendMode, FontType,
//     MaterialMapIndex,
//     ShaderAttributeDataType, ShaderLocationIndex, ShaderUniformDataType,
// };
// </temporary>

pub use ffi::{
    CameraMode, CameraProjection, ConfigFlags, GamepadAxis, GamepadButton, Gesture, KeyboardKey,
    MouseButton, MouseCursor,
};

pub type Vector2 = mint::Vector2<f32>;
assert_eq_size!(Vector2, ffi::Vector2);
assert_eq_align!(Vector2, ffi::Vector2);

pub type Vector3 = mint::Vector3<f32>;
assert_eq_size!(Vector3, ffi::Vector3);
assert_eq_align!(Vector3, ffi::Vector3);

pub type Vector4 = mint::Vector4<f32>;
assert_eq_size!(Vector4, ffi::Vector4);
assert_eq_align!(Vector4, ffi::Vector4);

pub type Quaternion = mint::Quaternion<f32>;
assert_eq_size!(Quaternion, ffi::Quaternion);
assert_eq_align!(Quaternion, ffi::Quaternion);

pub type Matrix = mint::RowMatrix4<f32>;
assert_eq_size!(Matrix, ffi::Matrix);
assert_eq_align!(Matrix, ffi::Matrix);

/// Rectangle, 4 components
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rectangle {
    /// Rectangle top-left corner position x
    pub x: f32,
    /// Rectangle top-left corner position y
    pub y: f32,
    /// Rectangle width
    pub width: f32,
    /// Rectangle height
    pub height: f32,
}

assert_eq_size!(Rectangle, ffi::Rectangle);
assert_eq_align!(Rectangle, ffi::Rectangle);

impl Rectangle {
    #[inline]
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

// /// Camera type fallback, defaults to Camera3D
// pub type Camera = Camera3D;

#[derive(Debug)]
pub struct Raylib(());

impl Raylib {
    /// Initialize window and OpenGL context
    #[inline]
    pub fn init_window(width: u32, height: u32, title: &str) -> Self {
        let title = CString::new(title).unwrap();

        unsafe {
            ffi::InitWindow(width as i32, height as i32, title.as_ptr());
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
    pub fn is_window_minimized() -> bool {
        unsafe { ffi::IsWindowMinimized() }
    }

    /// Check if window is currently maximized (only PLATFORM_DESKTOP)
    #[inline]
    pub fn is_window_maximized() -> bool {
        unsafe { ffi::IsWindowMaximized() }
    }

    /// Check if window is currently focused (only PLATFORM_DESKTOP)
    #[inline]
    pub fn is_window_focused() -> bool {
        unsafe { ffi::IsWindowFocused() }
    }

    /// Check if window has been resized last frame
    #[inline]
    pub fn is_window_resized() -> bool {
        unsafe { ffi::IsWindowResized() }
    }
}

/*
    /// Check if one specific window flag is enabled
    pub fn IsWindowState(flag: core::ffi::c_uint, ) -> bool;
    /// Set window configuration state using flags (only PLATFORM_DESKTOP)
    pub fn SetWindowState(flags: core::ffi::c_uint, );
    /// Clear window configuration state flags
    pub fn ClearWindowState(flags: core::ffi::c_uint, );
    /// Toggle window state: fullscreen/windowed (only PLATFORM_DESKTOP)
    pub fn ToggleFullscreen();
    /// Set window state: maximized, if resizable (only PLATFORM_DESKTOP)
    pub fn MaximizeWindow();
    /// Set window state: minimized, if resizable (only PLATFORM_DESKTOP)
    pub fn MinimizeWindow();
    /// Set window state: not minimized/maximized (only PLATFORM_DESKTOP)
    pub fn RestoreWindow();
    /// Set icon for window (single image, RGBA 32bit, only PLATFORM_DESKTOP)
    pub fn SetWindowIcon(image: Image, );
    /// Set icon for window (multiple images, RGBA 32bit, only PLATFORM_DESKTOP)
    pub fn SetWindowIcons(images: *mut Image, count: core::ffi::c_int, );
    /// Set title for window (only PLATFORM_DESKTOP)
    pub fn SetWindowTitle(title: *const core::ffi::c_char, );
    /// Set window position on screen (only PLATFORM_DESKTOP)
    pub fn SetWindowPosition(x: core::ffi::c_int, y: core::ffi::c_int, );
    /// Set monitor for the current window (fullscreen mode)
    pub fn SetWindowMonitor(monitor: core::ffi::c_int, );
    /// Set window minimum dimensions (for FLAG_WINDOW_RESIZABLE)
    pub fn SetWindowMinSize(width: core::ffi::c_int, height: core::ffi::c_int, );
    /// Set window dimensions
    pub fn SetWindowSize(width: core::ffi::c_int, height: core::ffi::c_int, );
    /// Set window opacity [0.0f..1.0f] (only PLATFORM_DESKTOP)
    pub fn SetWindowOpacity(opacity: core::ffi::c_float, );
    /// Get native window handle
    pub fn GetWindowHandle() -> *mut core::ffi::c_void;
    /// Get current screen width
    pub fn GetScreenWidth() -> core::ffi::c_int;
    /// Get current screen height
    pub fn GetScreenHeight() -> core::ffi::c_int;
    /// Get current render width (it considers HiDPI)
    pub fn GetRenderWidth() -> core::ffi::c_int;
    /// Get current render height (it considers HiDPI)
    pub fn GetRenderHeight() -> core::ffi::c_int;
    /// Get number of connected monitors
    pub fn GetMonitorCount() -> core::ffi::c_int;
    /// Get current connected monitor
    pub fn GetCurrentMonitor() -> core::ffi::c_int;
    /// Get specified monitor position
    pub fn GetMonitorPosition(monitor: core::ffi::c_int, ) -> Vector2;
    /// Get specified monitor width (current video mode used by monitor)
    pub fn GetMonitorWidth(monitor: core::ffi::c_int, ) -> core::ffi::c_int;
    /// Get specified monitor height (current video mode used by monitor)
    pub fn GetMonitorHeight(monitor: core::ffi::c_int, ) -> core::ffi::c_int;
    /// Get specified monitor physical width in millimetres
    pub fn GetMonitorPhysicalWidth(monitor: core::ffi::c_int, ) -> core::ffi::c_int;
    /// Get specified monitor physical height in millimetres
    pub fn GetMonitorPhysicalHeight(monitor: core::ffi::c_int, ) -> core::ffi::c_int;
    /// Get specified monitor refresh rate
    pub fn GetMonitorRefreshRate(monitor: core::ffi::c_int, ) -> core::ffi::c_int;
    /// Get window position XY on monitor
    pub fn GetWindowPosition() -> Vector2;
    /// Get window scale DPI factor
    pub fn GetWindowScaleDPI() -> Vector2;
    /// Get the human-readable, UTF-8 encoded name of the primary monitor
    pub fn GetMonitorName(monitor: core::ffi::c_int, ) -> *const core::ffi::c_char;
    /// Set clipboard text content
    pub fn SetClipboardText(text: *const core::ffi::c_char, );
    /// Get clipboard text content
    pub fn GetClipboardText() -> *const core::ffi::c_char;
    /// Enable waiting for events on EndDrawing(), no automatic event polling
    pub fn EnableEventWaiting();
    /// Disable waiting for events on EndDrawing(), automatic events polling
    pub fn DisableEventWaiting();
    /// Swap back buffer with front buffer (screen drawing)
    pub fn SwapScreenBuffer();
    /// Register all input events
    pub fn PollInputEvents();
    /// Wait for some time (halt program execution)
    pub fn WaitTime(seconds: core::ffi::c_double, );
    /// Shows cursor
    pub fn ShowCursor();
    /// Hides cursor
    pub fn HideCursor();
    /// Check if cursor is not visible
    pub fn IsCursorHidden() -> bool;
    /// Enables cursor (unlock cursor)
    pub fn EnableCursor();
    /// Disables cursor (lock cursor)
    pub fn DisableCursor();
    /// Check if cursor is on the screen
    pub fn IsCursorOnScreen() -> bool;
    /// Set background color (framebuffer clear color)
    pub fn ClearBackground(color: Color, );
    /// Setup canvas (framebuffer) to start drawing
    pub fn BeginDrawing();
    /// End canvas drawing and swap buffers (double buffering)
    pub fn EndDrawing();
    /// Begin 2D mode with custom camera (2D)
    pub fn BeginMode2D(camera: Camera2D, );
    /// Ends 2D mode with custom camera
    pub fn EndMode2D();
    /// Begin 3D mode with custom camera (3D)
    pub fn BeginMode3D(camera: Camera3D, );
    /// Ends 3D mode and returns to default 2D orthographic mode
    pub fn EndMode3D();
    /// Begin drawing to render texture
    pub fn BeginTextureMode(target: RenderTexture2D, );
    /// Ends drawing to render texture
    pub fn EndTextureMode();
    /// Begin custom shader drawing
    pub fn BeginShaderMode(shader: Shader, );
    /// End custom shader drawing (use default shader)
    pub fn EndShaderMode();
    /// Begin blending mode (alpha, additive, multiplied, subtract, custom)
    pub fn BeginBlendMode(mode: core::ffi::c_int, );
    /// End blending mode (reset to default: alpha blending)
    pub fn EndBlendMode();
    /// Begin scissor mode (define screen area for following drawing)
    pub fn BeginScissorMode(x: core::ffi::c_int, y: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int, );
    /// End scissor mode
    pub fn EndScissorMode();
    /// Begin stereo rendering (requires VR simulator)
    pub fn BeginVrStereoMode(config: VrStereoConfig, );
    /// End stereo rendering (requires VR simulator)
    pub fn EndVrStereoMode();
    /// Load VR stereo config for VR simulator device parameters
    pub fn LoadVrStereoConfig(device: VrDeviceInfo, ) -> VrStereoConfig;
    /// Unload VR stereo config
    pub fn UnloadVrStereoConfig(config: VrStereoConfig, );
    /// Load shader from files and bind default locations
    pub fn LoadShader(vsFileName: *const core::ffi::c_char, fsFileName: *const core::ffi::c_char, ) -> Shader;
    /// Load shader from code strings and bind default locations
    pub fn LoadShaderFromMemory(vsCode: *const core::ffi::c_char, fsCode: *const core::ffi::c_char, ) -> Shader;
    /// Check if a shader is ready
    pub fn IsShaderReady(shader: Shader, ) -> bool;
    /// Get shader uniform location
    pub fn GetShaderLocation(shader: Shader, uniformName: *const core::ffi::c_char, ) -> core::ffi::c_int;
    /// Get shader attribute location
    pub fn GetShaderLocationAttrib(shader: Shader, attribName: *const core::ffi::c_char, ) -> core::ffi::c_int;
    /// Set shader uniform value
    pub fn SetShaderValue(shader: Shader, locIndex: core::ffi::c_int, value: *const core::ffi::c_void, uniformType: core::ffi::c_int, );
    /// Set shader uniform value vector
    pub fn SetShaderValueV(shader: Shader, locIndex: core::ffi::c_int, value: *const core::ffi::c_void, uniformType: core::ffi::c_int, count: core::ffi::c_int, );
    /// Set shader uniform value (matrix 4x4)
    pub fn SetShaderValueMatrix(shader: Shader, locIndex: core::ffi::c_int, mat: Matrix, );
    /// Set shader uniform value for texture (sampler2d)
    pub fn SetShaderValueTexture(shader: Shader, locIndex: core::ffi::c_int, texture: Texture2D, );
    /// Unload shader from GPU memory (VRAM)
    pub fn UnloadShader(shader: Shader, );
    /// Get a ray trace from mouse position
    pub fn GetMouseRay(mousePosition: Vector2, camera: Camera, ) -> Ray;
    /// Get camera transform matrix (view matrix)
    pub fn GetCameraMatrix(camera: Camera, ) -> Matrix;
    /// Get camera 2d transform matrix
    pub fn GetCameraMatrix2D(camera: Camera2D, ) -> Matrix;
    /// Get the screen space position for a 3d world space position
    pub fn GetWorldToScreen(position: Vector3, camera: Camera, ) -> Vector2;
    /// Get the world space position for a 2d camera screen space position
    pub fn GetScreenToWorld2D(position: Vector2, camera: Camera2D, ) -> Vector2;
    /// Get size position for a 3d world space position
    pub fn GetWorldToScreenEx(position: Vector3, camera: Camera, width: core::ffi::c_int, height: core::ffi::c_int, ) -> Vector2;
    /// Get the screen space position for a 2d camera world space position
    pub fn GetWorldToScreen2D(position: Vector2, camera: Camera2D, ) -> Vector2;
    /// Set target FPS (maximum)
    pub fn SetTargetFPS(fps: core::ffi::c_int, );
    /// Get current FPS
    pub fn GetFPS() -> core::ffi::c_int;
    /// Get time in seconds for last frame drawn (delta time)
    pub fn GetFrameTime() -> core::ffi::c_float;
    /// Get elapsed time in seconds since InitWindow()
    pub fn GetTime() -> core::ffi::c_double;
    /// Get a random value between min and max (both included)
    pub fn GetRandomValue(min: core::ffi::c_int, max: core::ffi::c_int, ) -> core::ffi::c_int;
    /// Set the seed for the random number generator
    pub fn SetRandomSeed(seed: core::ffi::c_uint, );
    /// Takes a screenshot of current screen (filename extension defines format)
    pub fn TakeScreenshot(fileName: *const core::ffi::c_char, );
    /// Setup init configuration flags (view FLAGS)
    pub fn SetConfigFlags(flags: core::ffi::c_uint, );
    /// Show trace log messages (LOG_DEBUG, LOG_INFO, LOG_WARNING, LOG_ERROR...)
    pub fn TraceLog(logLevel: core::ffi::c_int, text: *const core::ffi::c_char, ..., );
    /// Set the current threshold (minimum) log level
    pub fn SetTraceLogLevel(logLevel: core::ffi::c_int, );
    /// Internal memory allocator
    pub fn MemAlloc(size: core::ffi::c_uint, ) -> *mut core::ffi::c_void;
    /// Internal memory reallocator
    pub fn MemRealloc(ptr: *mut core::ffi::c_void, size: core::ffi::c_uint, ) -> *mut core::ffi::c_void;
    /// Internal memory free
    pub fn MemFree(ptr: *mut core::ffi::c_void, );
    /// Open URL with default system browser (if available)
    pub fn OpenURL(url: *const core::ffi::c_char, );
    /// Set custom trace log
    pub fn SetTraceLogCallback(callback: TraceLogCallback, );
    /// Set custom file binary data loader
    pub fn SetLoadFileDataCallback(callback: LoadFileDataCallback, );
    /// Set custom file binary data saver
    pub fn SetSaveFileDataCallback(callback: SaveFileDataCallback, );
    /// Set custom file text data loader
    pub fn SetLoadFileTextCallback(callback: LoadFileTextCallback, );
    /// Set custom file text data saver
    pub fn SetSaveFileTextCallback(callback: SaveFileTextCallback, );
    /// Load file data as byte array (read)
    pub fn LoadFileData(fileName: *const core::ffi::c_char, bytesRead: *mut core::ffi::c_uint, ) -> *mut core::ffi::c_uchar;
    /// Unload file data allocated by LoadFileData()
    pub fn UnloadFileData(data: *mut core::ffi::c_uchar, );
    /// Save data to file from byte array (write), returns true on success
    pub fn SaveFileData(fileName: *const core::ffi::c_char, data: *mut core::ffi::c_void, bytesToWrite: core::ffi::c_uint, ) -> bool;
    /// Export data to code (.h), returns true on success
    pub fn ExportDataAsCode(data: *const core::ffi::c_uchar, size: core::ffi::c_uint, fileName: *const core::ffi::c_char, ) -> bool;
    /// Load text data from file (read), returns a '\0' terminated string
    pub fn LoadFileText(fileName: *const core::ffi::c_char, ) -> *mut core::ffi::c_char;
    /// Unload file text data allocated by LoadFileText()
    pub fn UnloadFileText(text: *mut core::ffi::c_char, );
    /// Save text data to file (write), string must be '\0' terminated, returns true on success
    pub fn SaveFileText(fileName: *const core::ffi::c_char, text: *mut core::ffi::c_char, ) -> bool;
    /// Check if file exists
    pub fn FileExists(fileName: *const core::ffi::c_char, ) -> bool;
    /// Check if a directory path exists
    pub fn DirectoryExists(dirPath: *const core::ffi::c_char, ) -> bool;
    /// Check file extension (including point: .png, .wav)
    pub fn IsFileExtension(fileName: *const core::ffi::c_char, ext: *const core::ffi::c_char, ) -> bool;
    /// Get file length in bytes (NOTE: GetFileSize() conflicts with windows.h)
    pub fn GetFileLength(fileName: *const core::ffi::c_char, ) -> core::ffi::c_int;
    /// Get pointer to extension for a filename string (includes dot: '.png')
    pub fn GetFileExtension(fileName: *const core::ffi::c_char, ) -> *const core::ffi::c_char;
    /// Get pointer to filename for a path string
    pub fn GetFileName(filePath: *const core::ffi::c_char, ) -> *const core::ffi::c_char;
    /// Get filename string without extension (uses static string)
    pub fn GetFileNameWithoutExt(filePath: *const core::ffi::c_char, ) -> *const core::ffi::c_char;
    /// Get full path for a given fileName with path (uses static string)
    pub fn GetDirectoryPath(filePath: *const core::ffi::c_char, ) -> *const core::ffi::c_char;
    /// Get previous directory path for a given path (uses static string)
    pub fn GetPrevDirectoryPath(dirPath: *const core::ffi::c_char, ) -> *const core::ffi::c_char;
    /// Get current working directory (uses static string)
    pub fn GetWorkingDirectory() -> *const core::ffi::c_char;
    /// Get the directory if the running application (uses static string)
    pub fn GetApplicationDirectory() -> *const core::ffi::c_char;
    /// Change working directory, return true on success
    pub fn ChangeDirectory(dir: *const core::ffi::c_char, ) -> bool;
    /// Check if a given path is a file or a directory
    pub fn IsPathFile(path: *const core::ffi::c_char, ) -> bool;
    /// Load directory filepaths
    pub fn LoadDirectoryFiles(dirPath: *const core::ffi::c_char, ) -> FilePathList;
    /// Load directory filepaths with extension filtering and recursive directory scan
    pub fn LoadDirectoryFilesEx(basePath: *const core::ffi::c_char, filter: *const core::ffi::c_char, scanSubdirs: bool, ) -> FilePathList;
    /// Unload filepaths
    pub fn UnloadDirectoryFiles(files: FilePathList, );
    /// Check if a file has been dropped into window
    pub fn IsFileDropped() -> bool;
    /// Load dropped filepaths
    pub fn LoadDroppedFiles() -> FilePathList;
    /// Unload dropped filepaths
    pub fn UnloadDroppedFiles(files: FilePathList, );
    /// Get file modification time (last write time)
    pub fn GetFileModTime(fileName: *const core::ffi::c_char, ) -> core::ffi::c_long;
    /// Compress data (DEFLATE algorithm), memory must be MemFree()
    pub fn CompressData(data: *const core::ffi::c_uchar, dataSize: core::ffi::c_int, compDataSize: *mut core::ffi::c_int, ) -> *mut core::ffi::c_uchar;
    /// Decompress data (DEFLATE algorithm), memory must be MemFree()
    pub fn DecompressData(compData: *const core::ffi::c_uchar, compDataSize: core::ffi::c_int, dataSize: *mut core::ffi::c_int, ) -> *mut core::ffi::c_uchar;
    /// Encode data to Base64 string, memory must be MemFree()
    pub fn EncodeDataBase64(data: *const core::ffi::c_uchar, dataSize: core::ffi::c_int, outputSize: *mut core::ffi::c_int, ) -> *mut core::ffi::c_char;
    /// Decode Base64 string data, memory must be MemFree()
    pub fn DecodeDataBase64(data: *const core::ffi::c_uchar, outputSize: *mut core::ffi::c_int, ) -> *mut core::ffi::c_uchar;
    /// Check if a key has been pressed once
    pub fn IsKeyPressed(key: core::ffi::c_int, ) -> bool;
    /// Check if a key is being pressed
    pub fn IsKeyDown(key: core::ffi::c_int, ) -> bool;
    /// Check if a key has been released once
    pub fn IsKeyReleased(key: core::ffi::c_int, ) -> bool;
    /// Check if a key is NOT being pressed
    pub fn IsKeyUp(key: core::ffi::c_int, ) -> bool;
    /// Set a custom key to exit program (default is ESC)
    pub fn SetExitKey(key: core::ffi::c_int, );
    /// Get key pressed (keycode), call it multiple times for keys queued, returns 0 when the queue is empty
    pub fn GetKeyPressed() -> core::ffi::c_int;
    /// Get char pressed (unicode), call it multiple times for chars queued, returns 0 when the queue is empty
    pub fn GetCharPressed() -> core::ffi::c_int;
    /// Check if a gamepad is available
    pub fn IsGamepadAvailable(gamepad: core::ffi::c_int, ) -> bool;
    /// Get gamepad internal name id
    pub fn GetGamepadName(gamepad: core::ffi::c_int, ) -> *const core::ffi::c_char;
    /// Check if a gamepad button has been pressed once
    pub fn IsGamepadButtonPressed(gamepad: core::ffi::c_int, button: core::ffi::c_int, ) -> bool;
    /// Check if a gamepad button is being pressed
    pub fn IsGamepadButtonDown(gamepad: core::ffi::c_int, button: core::ffi::c_int, ) -> bool;
    /// Check if a gamepad button has been released once
    pub fn IsGamepadButtonReleased(gamepad: core::ffi::c_int, button: core::ffi::c_int, ) -> bool;
    /// Check if a gamepad button is NOT being pressed
    pub fn IsGamepadButtonUp(gamepad: core::ffi::c_int, button: core::ffi::c_int, ) -> bool;
    /// Get the last gamepad button pressed
    pub fn GetGamepadButtonPressed() -> core::ffi::c_int;
    /// Get gamepad axis count for a gamepad
    pub fn GetGamepadAxisCount(gamepad: core::ffi::c_int, ) -> core::ffi::c_int;
    /// Get axis movement value for a gamepad axis
    pub fn GetGamepadAxisMovement(gamepad: core::ffi::c_int, axis: core::ffi::c_int, ) -> core::ffi::c_float;
    /// Set internal gamepad mappings (SDL_GameControllerDB)
    pub fn SetGamepadMappings(mappings: *const core::ffi::c_char, ) -> core::ffi::c_int;
    /// Check if a mouse button has been pressed once
    pub fn IsMouseButtonPressed(button: core::ffi::c_int, ) -> bool;
    /// Check if a mouse button is being pressed
    pub fn IsMouseButtonDown(button: core::ffi::c_int, ) -> bool;
    /// Check if a mouse button has been released once
    pub fn IsMouseButtonReleased(button: core::ffi::c_int, ) -> bool;
    /// Check if a mouse button is NOT being pressed
    pub fn IsMouseButtonUp(button: core::ffi::c_int, ) -> bool;
    /// Get mouse position X
    pub fn GetMouseX() -> core::ffi::c_int;
    /// Get mouse position Y
    pub fn GetMouseY() -> core::ffi::c_int;
    /// Get mouse position XY
    pub fn GetMousePosition() -> Vector2;
    /// Get mouse delta between frames
    pub fn GetMouseDelta() -> Vector2;
    /// Set mouse position XY
    pub fn SetMousePosition(x: core::ffi::c_int, y: core::ffi::c_int, );
    /// Set mouse offset
    pub fn SetMouseOffset(offsetX: core::ffi::c_int, offsetY: core::ffi::c_int, );
    /// Set mouse scaling
    pub fn SetMouseScale(scaleX: core::ffi::c_float, scaleY: core::ffi::c_float, );
    /// Get mouse wheel movement for X or Y, whichever is larger
    pub fn GetMouseWheelMove() -> core::ffi::c_float;
    /// Get mouse wheel movement for both X and Y
    pub fn GetMouseWheelMoveV() -> Vector2;
    /// Set mouse cursor
    pub fn SetMouseCursor(cursor: core::ffi::c_int, );
    /// Get touch position X for touch point 0 (relative to screen size)
    pub fn GetTouchX() -> core::ffi::c_int;
    /// Get touch position Y for touch point 0 (relative to screen size)
    pub fn GetTouchY() -> core::ffi::c_int;
    /// Get touch position XY for a touch point index (relative to screen size)
    pub fn GetTouchPosition(index: core::ffi::c_int, ) -> Vector2;
    /// Get touch point identifier for given index
    pub fn GetTouchPointId(index: core::ffi::c_int, ) -> core::ffi::c_int;
    /// Get number of touch points
    pub fn GetTouchPointCount() -> core::ffi::c_int;
    /// Enable a set of gestures using flags
    pub fn SetGesturesEnabled(flags: core::ffi::c_uint, );
    /// Check if a gesture have been detected
    pub fn IsGestureDetected(gesture: core::ffi::c_int, ) -> bool;
    /// Get latest detected gesture
    pub fn GetGestureDetected() -> core::ffi::c_int;
    /// Get gesture hold time in milliseconds
    pub fn GetGestureHoldDuration() -> core::ffi::c_float;
    /// Get gesture drag vector
    pub fn GetGestureDragVector() -> Vector2;
    /// Get gesture drag angle
    pub fn GetGestureDragAngle() -> core::ffi::c_float;
    /// Get gesture pinch delta
    pub fn GetGesturePinchVector() -> Vector2;
    /// Get gesture pinch angle
    pub fn GetGesturePinchAngle() -> core::ffi::c_float;
    /// Update camera position for selected mode
    pub fn UpdateCamera(camera: *mut Camera, mode: core::ffi::c_int, );
    /// Update camera movement/rotation
    pub fn UpdateCameraPro(camera: *mut Camera, movement: Vector3, rotation: Vector3, zoom: core::ffi::c_float, );
*/

impl Drop for Raylib {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ffi::CloseWindow();
        }
    }
}
