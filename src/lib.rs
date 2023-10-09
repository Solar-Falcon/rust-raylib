#![doc = include_str!("../README.md")]

/// Raw ffi bindings
pub mod ffi;
pub use ffi::{RAYLIB_VERSION, RAYLIB_VERSION_MAJOR, RAYLIB_VERSION_MINOR, RAYLIB_VERSION_PATCH};

/// Audio
pub mod audio;
/// Collision checks between different shapes
pub mod collision;
/// Color type and color constants
pub mod color;
/// Drawing traits and functions
pub mod drawing;
/// Math types
pub mod math;
/// 3D models
pub mod model;
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
        pub fn TraceLog(logLevel: u32, text: *const core::ffi::c_char, ...) {}

        /// Internal memory allocator
        pub fn MemAlloc(size: core::ffi::c_uint) -> *mut core::ffi::c_void;

        /// Internal memory reallocator
        pub fn MemRealloc(ptr: *mut core::ffi::c_void, size: core::ffi::c_uint) -> *mut core::ffi::c_void;

        /// Internal memory free
        pub fn MemFree(ptr: *mut core::ffi::c_void) {}

        /// Set custom trace log
        pub fn SetTraceLogCallback(callback: TraceLogCallback) {}

        /// Set custom file binary data loader
        pub fn SetLoadFileDataCallback(callback: LoadFileDataCallback) {}

        /// Set custom file binary data saver
        pub fn SetSaveFileDataCallback(callback: SaveFileDataCallback) {}

        /// Set custom file text data loader
        pub fn SetLoadFileTextCallback(callback: LoadFileTextCallback) {}

        /// Set custom file text data saver
        pub fn SetSaveFileTextCallback(callback: SaveFileTextCallback) {}

        /// Load file data as byte array (read)
        pub fn LoadFileData(fileName: *const core::ffi::c_char, bytesRead: *mut core::ffi::c_uint) -> *mut core::ffi::c_uchar;

        /// Unload file data allocated by LoadFileData()
        pub fn UnloadFileData(data: *mut core::ffi::c_uchar) {}

        /// Save data to file from byte array (write), returns true on success
        pub fn SaveFileData(fileName: *const core::ffi::c_char, data: *mut core::ffi::c_void, bytesToWrite: core::ffi::c_uint) -> bool;

        /// Export data to code (.h), returns true on success
        pub fn ExportDataAsCode(data: *const core::ffi::c_uchar, size: core::ffi::c_uint, fileName: *const core::ffi::c_char) -> bool;

        /// Load text data from file (read), returns a '\0' terminated string
        pub fn LoadFileText(fileName: *const core::ffi::c_char) -> *mut core::ffi::c_char;

        /// Unload file text data allocated by LoadFileText()
        pub fn UnloadFileText(text: *mut core::ffi::c_char) {}

        /// Save text data to file (write), string must be '\0' terminated, returns true on success
        pub fn SaveFileText(fileName: *const core::ffi::c_char, text: *mut core::ffi::c_char) -> bool;

        /// Check if file exists
        pub fn FileExists(fileName: *const core::ffi::c_char) -> bool;

        /// Check if a directory path exists
        pub fn DirectoryExists(dirPath: *const core::ffi::c_char) -> bool;

        /// Check file extension (including point: .png, .wav)
        pub fn IsFileExtension(fileName: *const core::ffi::c_char, ext: *const core::ffi::c_char) -> bool;

        /// Get file length in bytes (NOTE: GetFileSize() conflicts with windows.h)
        pub fn GetFileLength(fileName: *const core::ffi::c_char) -> u32;

        /// Get pointer to extension for a file_name string (includes dot: '.png')
        pub fn GetFileExtension(fileName: *const core::ffi::c_char) -> *const core::ffi::c_char;

        /// Get pointer to file_name for a path string
        pub fn GetFileName(filePath: *const core::ffi::c_char) -> *const core::ffi::c_char;

        /// Get file_name string without extension (uses static string)
        pub fn GetFileNameWithoutExt(filePath: *const core::ffi::c_char) -> *const core::ffi::c_char;

        /// Get full path for a given fileName with path (uses static string)
        pub fn GetDirectoryPath(filePath: *const core::ffi::c_char) -> *const core::ffi::c_char;

        /// Get previous directory path for a given path (uses static string)
        pub fn GetPrevDirectoryPath(dirPath: *const core::ffi::c_char) -> *const core::ffi::c_char;

        /// Get current working directory (uses static string)
        pub fn GetWorkingDirectory() -> *const core::ffi::c_char;

        /// Get the directory if the running application (uses static string)
        pub fn GetApplicationDirectory() -> *const core::ffi::c_char;

        /// Change working directory, return true on success
        pub fn ChangeDirectory(dir: *const core::ffi::c_char) -> bool;

        /// Check if a given path is a file or a directory
        pub fn IsPathFile(path: *const core::ffi::c_char) -> bool;

        /// Load directory filepaths
        pub fn LoadDirectoryFiles(dirPath: *const core::ffi::c_char) -> FilePathList;

        /// Load directory filepaths with extension filtering and recursive directory scan
        pub fn LoadDirectoryFilesEx(basePath: *const core::ffi::c_char, filter: *const core::ffi::c_char, scanSubdirs: bool) -> FilePathList;

        /// Unload filepaths
        pub fn UnloadDirectoryFiles(files: FilePathList) {}

        /// Get file modification time (last write time)
        pub fn GetFileModTime(fileName: *const core::ffi::c_char) -> core::ffi::c_long;

        /// Compress data (DEFLATE algorithm), memory must be MemFree()
        pub fn CompressData(data: *const core::ffi::c_uchar, dataSize: u32, compDataSize: *mut u32) -> *mut core::ffi::c_uchar;

        /// Decompress data (DEFLATE algorithm), memory must be MemFree()
        pub fn DecompressData(compData: *const core::ffi::c_uchar, compDataSize: u32, dataSize: *mut u32) -> *mut core::ffi::c_uchar;

        /// Encode data to Base64 string, memory must be MemFree()
        pub fn EncodeDataBase64(data: *const core::ffi::c_uchar, dataSize: u32, outputSize: *mut u32) -> *mut core::ffi::c_char;

        /// Decode Base64 string data, memory must be MemFree()
        pub fn DecodeDataBase64(data: *const core::ffi::c_uchar, outputSize: *mut u32) -> *mut core::ffi::c_uchar;

        /// Load UTF-8 text encoded from codepoints array
        pub fn LoadUTF8(codepoints: *const core::ffi::c_int, length: core::ffi::c_int, ) -> *mut core::ffi::c_char;

        /// Unload UTF-8 text encoded from codepoints array
        pub fn UnloadUTF8(text: *mut core::ffi::c_char, );

        /// Load all codepoints from a UTF-8 text string, codepoints count returned by parameter
        pub fn LoadCodepoints(text: *const core::ffi::c_char, count: *mut core::ffi::c_int, ) -> *mut core::ffi::c_int;

        /// Unload codepoints data from memory
        pub fn UnloadCodepoints(codepoints: *mut core::ffi::c_int, );

        /// Get total number of codepoints in a UTF-8 encoded string
        pub fn GetCodepointCount(text: *const core::ffi::c_char, ) -> core::ffi::c_int;

        /// Get next codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
        pub fn GetCodepoint(text: *const core::ffi::c_char, codepointSize: *mut core::ffi::c_int, ) -> core::ffi::c_int;

        /// Get next codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
        pub fn GetCodepointNext(text: *const core::ffi::c_char, codepointSize: *mut core::ffi::c_int, ) -> core::ffi::c_int;

        /// Get previous codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
        pub fn GetCodepointPrevious(text: *const core::ffi::c_char, codepointSize: *mut core::ffi::c_int, ) -> core::ffi::c_int;

        /// Encode one codepoint into UTF-8 byte array (array length returned as parameter)
        pub fn CodepointToUTF8(codepoint: core::ffi::c_int, utf8Size: *mut core::ffi::c_int, ) -> *const core::ffi::c_char;

        /// Copy one string to another, returns bytes copied
        pub fn TextCopy(dst: *mut core::ffi::c_char, src: *const core::ffi::c_char, ) -> core::ffi::c_int;

        /// Check if two text string are equal
        pub fn TextIsEqual(text1: *const core::ffi::c_char, text2: *const core::ffi::c_char, ) -> bool;

        /// Get text length, checks for '\0' ending
        pub fn TextLength(text: *const core::ffi::c_char, ) -> core::ffi::c_uint;

        /// Text formatting with variables (sprintf() style)
        pub fn TextFormat(text: *const core::ffi::c_char, ..., ) -> *const core::ffi::c_char;

        /// Get a piece of a text string
        pub fn TextSubtext(text: *const core::ffi::c_char, position: core::ffi::c_int, length: core::ffi::c_int, ) -> *const core::ffi::c_char;

        /// Replace text string (WARNING: memory must be freed!)
        pub fn TextReplace(text: *mut core::ffi::c_char, replace: *const core::ffi::c_char, by: *const core::ffi::c_char, ) -> *mut core::ffi::c_char;

        /// Insert text in a position (WARNING: memory must be freed!)
        pub fn TextInsert(text: *const core::ffi::c_char, insert: *const core::ffi::c_char, position: core::ffi::c_int, ) -> *mut core::ffi::c_char;

        /// Join text strings with delimiter
        pub fn TextJoin(textList: *const *const core::ffi::c_char, count: core::ffi::c_int, delimiter: *const core::ffi::c_char, ) -> *const core::ffi::c_char;

        /// Split text into multiple strings
        pub fn TextSplit(text: *const core::ffi::c_char, delimiter: core::ffi::c_char, count: *mut core::ffi::c_int, ) -> *const *const core::ffi::c_char;

        /// Append text at specific position and move cursor!
        pub fn TextAppend(text: *mut core::ffi::c_char, append: *const core::ffi::c_char, position: *mut core::ffi::c_int, );

        /// Find first text occurrence within a string
        pub fn TextFindIndex(text: *const core::ffi::c_char, find: *const core::ffi::c_char, ) -> core::ffi::c_int;

        /// Get upper case version of provided string
        pub fn TextToUpper(text: *const core::ffi::c_char, ) -> *const core::ffi::c_char;

        /// Get lower case version of provided string
        pub fn TextToLower(text: *const core::ffi::c_char, ) -> *const core::ffi::c_char;

        /// Get Pascal case notation version of provided string
        pub fn TextToPascal(text: *const core::ffi::c_char, ) -> *const core::ffi::c_char;

        /// Get integer value from text (negative values not supported)
        pub fn TextToInteger(text: *const core::ffi::c_char, ) -> core::ffi::c_int;
*/
