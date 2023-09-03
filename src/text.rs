use crate::ffi;

pub use crate::ffi::FontType;

#[derive(Clone, Debug)]
pub struct Font {
    pub(crate) raw: ffi::Font,
}
