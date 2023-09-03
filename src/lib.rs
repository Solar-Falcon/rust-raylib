/// Raw ffi bindings
pub mod ffi;

pub use ffi::{RAYLIB_VERSION, RAYLIB_VERSION_MAJOR, RAYLIB_VERSION_MINOR, RAYLIB_VERSION_PATCH};

/// Color type and color constants
pub mod color;
/// Collision checks between different shapes
pub mod collision;
/// Drawing traits and functions
pub mod drawing;
/// Math types
pub mod math;
/// Shader type
pub mod shader;
/// Fonts and text related types and functions
pub mod text;
/// Images and textures
pub mod texture;
/// VR related types
pub mod vr;

mod core;
pub use crate::core::*;

/*
    // Loser List: functions that aren't included in the wrapper, because there are better and more idiomatic solutions available

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


*/
