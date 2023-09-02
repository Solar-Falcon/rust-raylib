use crate::ffi;

#[derive(Clone, Debug)]
pub struct Font {
    pub(crate) raw: ffi::Font,
}
