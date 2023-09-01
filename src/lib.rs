#[allow(non_snake_case)]
pub mod ffi {
    include!(concat!(env!("OUT_DIR"), "/raylib_ffi.rs"));
}
