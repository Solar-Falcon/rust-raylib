use crate::{
    ffi,
    math::{Matrix, Vector2, Vector3, Vector4},
    texture::Texture2D,
};
use std::ffi::CString;

pub use crate::ffi::{ShaderAttributeDataType, ShaderLocationIndex, ShaderUniformDataType};

/// Shader
#[derive(Debug)]
#[repr(transparent)]
pub struct Shader {
    pub(crate) raw: ffi::Shader,
}

impl Shader {
    /// Shader locations array
    #[inline]
    pub fn locations(&self) -> &[u32] {
        unsafe {
            std::slice::from_raw_parts(self.raw.locs as *const u32, ffi::MAX_SHADER_LOCATIONS)
        }
    }

    /// Shader locations array
    #[inline]
    pub fn locations_mut(&mut self) -> &mut [u32] {
        unsafe {
            std::slice::from_raw_parts_mut(self.raw.locs as *mut _, ffi::MAX_SHADER_LOCATIONS)
        }
    }

    /// Load shader from files and bind default locations
    #[inline]
    pub fn from_file(vs_filename: Option<&str>, fs_filename: Option<&str>) -> Option<Self> {
        let vs_filename = vs_filename.map(|s| CString::new(s).unwrap());
        let fs_filename = fs_filename.map(|s| CString::new(s).unwrap());

        let raw = unsafe {
            ffi::LoadShader(
                match vs_filename {
                    Some(vs) => vs.as_ptr(),
                    None => std::ptr::null(),
                },
                match fs_filename {
                    Some(fs) => fs.as_ptr(),
                    None => std::ptr::null(),
                },
            )
        };

        if unsafe { ffi::IsShaderReady(raw.clone()) } {
            Some(Self { raw })
        } else {
            None
        }
    }

    /// Load shader from code strings and bind default locations
    #[inline]
    pub fn from_memory(vs_code: Option<&str>, fs_code: Option<&str>) -> Option<Self> {
        let vs_code = vs_code.map(|s| CString::new(s).unwrap());
        let fs_code = fs_code.map(|s| CString::new(s).unwrap());

        let raw = unsafe {
            ffi::LoadShaderFromMemory(
                match vs_code {
                    Some(vs) => vs.as_ptr(),
                    None => std::ptr::null(),
                },
                match fs_code {
                    Some(fs) => fs.as_ptr(),
                    None => std::ptr::null(),
                },
            )
        };

        if unsafe { ffi::IsShaderReady(raw.clone()) } {
            Some(Self { raw })
        } else {
            None
        }
    }

    /// Get shader uniform location
    #[inline]
    pub fn get_location(&self, uniform_name: &str) -> u32 {
        let uniform_name = CString::new(uniform_name).unwrap();

        unsafe { ffi::GetShaderLocation(self.raw.clone(), uniform_name.as_ptr()) as _ }
    }

    /// Get shader attribute location
    #[inline]
    pub fn get_location_attribute(&self, attribute_name: &str) -> u32 {
        let attribute_name = CString::new(attribute_name).unwrap();

        unsafe {
            ffi::GetShaderLocationAttrib(self.raw.clone(), attribute_name.as_ptr()) as _
        }
    }

    /// Set shader uniform value
    #[inline]
    pub fn set_value<S: ShaderValue>(&mut self, loc_index: u32, value: S) {
        unsafe {
            ffi::SetShaderValue(
                self.raw.clone(),
                loc_index as _,
                value.raw_value(),
                S::UNIFORM_TYPE as _,
            )
        }
    }

    /// Set shader uniform value vector
    #[inline]
    pub fn set_value_vec<S: ShaderValue>(&mut self, loc_index: u32, values: &[S]) {
        unsafe {
            ffi::SetShaderValueV(
                self.raw.clone(),
                loc_index as _,
                values.as_ptr() as *const _,
                S::UNIFORM_TYPE as _,
                values.len() as _,
            )
        }
    }

    /// Set shader uniform value (matrix 4x4)
    #[inline]
    pub fn set_value_matrix(&mut self, loc_index: u32, mat: Matrix) {
        unsafe { ffi::SetShaderValueMatrix(self.raw.clone(), loc_index as _, mat.into()) }
    }

    /// Set shader uniform value for texture (sampler2d)
    #[inline]
    pub fn set_value_texture(&mut self, loc_index: u32, texture: &Texture2D) {
        unsafe {
            ffi::SetShaderValueTexture(
                self.raw.clone(),
                loc_index as _,
                texture.raw.clone(),
            )
        }
    }

    /// Get the 'raw' ffi type
    /// Take caution when cloning so it doesn't outlive the original
    #[inline]
    pub fn as_raw(&self) -> &ffi::Shader {
        &self.raw
    }

    /// Get the 'raw' ffi type
    /// Take caution when cloning so it doesn't outlive the original
    #[inline]
    pub fn as_raw_mut(&mut self) -> &mut ffi::Shader {
        &mut self.raw
    }

    /// Convert a 'raw' ffi object to a safe wrapper
    ///
    /// # Safety
    /// * The raw object must be correctly initialized
    /// * The raw object should be unique. Otherwise, make sure its clones don't outlive the newly created object.
    #[inline]
    pub unsafe fn from_raw(raw: ffi::Shader) -> Self {
        Self { raw }
    }
}

impl Drop for Shader {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::UnloadShader(self.raw.clone()) }
    }
}

/// Shader uniform value
/// You shouldn't need to implement this trait yourself.
pub trait ShaderValue
where
    Self: Sized,
{
    /// Uniform type assiciated with the value
    const UNIFORM_TYPE: ShaderUniformDataType;

    /// Get the value as a `void*`
    unsafe fn raw_value(&self) -> *const core::ffi::c_void {
        self as *const Self as *const _
    }
}

impl ShaderValue for f32 {
    const UNIFORM_TYPE: ShaderUniformDataType = ShaderUniformDataType::Float;
}

impl ShaderValue for Vector2 {
    const UNIFORM_TYPE: ShaderUniformDataType = ShaderUniformDataType::Vec2;
}

impl ShaderValue for Vector3 {
    const UNIFORM_TYPE: ShaderUniformDataType = ShaderUniformDataType::Vec3;
}

impl ShaderValue for Vector4 {
    const UNIFORM_TYPE: ShaderUniformDataType = ShaderUniformDataType::Vec4;
}

impl ShaderValue for i32 {
    const UNIFORM_TYPE: ShaderUniformDataType = ShaderUniformDataType::Int;
}

impl ShaderValue for mint::Vector2<i32> {
    const UNIFORM_TYPE: ShaderUniformDataType = ShaderUniformDataType::IVec2;
}

impl ShaderValue for mint::Vector3<i32> {
    const UNIFORM_TYPE: ShaderUniformDataType = ShaderUniformDataType::IVec3;
}

impl ShaderValue for mint::Vector4<i32> {
    const UNIFORM_TYPE: ShaderUniformDataType = ShaderUniformDataType::IVec4;
}
