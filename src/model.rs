use std::{ffi::CString, rc::Rc};

use crate::{
    ffi,
    math::{BoundingBox, Vector3},
    texture::{Image, Texture2D},
};

pub use crate::ffi::MaterialMapIndex;

/// Mesh, vertex data and vao/vbo
#[derive(Debug)]
pub struct Mesh {
    pub(crate) raw: ffi::Mesh,
}

impl Mesh {
    /// Upload mesh vertex data in GPU and provide VAO/VBO ids
    #[inline]
    pub fn upload(&mut self, dynamic: bool) {
        unsafe { ffi::UploadMesh(&mut self.raw as *mut _, dynamic) }
    }

    /// Update mesh vertex data in GPU for a specific buffer index
    #[inline]
    pub fn update_buffer(&self, index: u32, data: &[u8], offset: u32) {
        unsafe {
            ffi::UpdateMeshBuffer(
                self.raw.clone(),
                index as _,
                data.as_ptr() as *const _,
                data.len() as _,
                offset as _,
            )
        }
    }

    /// Export mesh data to file, returns true on success
    #[inline]
    pub fn export(&self, file_name: &str) -> bool {
        let file_name = CString::new(file_name).unwrap();

        unsafe { ffi::ExportMesh(self.raw.clone(), file_name.as_ptr()) }
    }

    /// Compute mesh bounding box limits
    #[inline]
    pub fn get_bounding_box(&self) -> BoundingBox {
        unsafe { ffi::GetMeshBoundingBox(self.raw.clone()).into() }
    }

    /// Compute mesh tangents
    #[inline]
    pub fn generate_tangents(&mut self) {
        unsafe { ffi::GenMeshTangents(&mut self.raw as *mut _) }
    }

    /// Generate polygonal mesh
    #[inline]
    pub fn generate_polygon(sides: u32, radius: f32) -> Self {
        Self {
            raw: unsafe { ffi::GenMeshPoly(sides as _, radius) },
        }
    }

    /// Generate plane mesh (with subdivisions)
    #[inline]
    pub fn generate_plane(width: f32, length: f32, res_x: u32, res_z: u32) -> Self {
        Self {
            raw: unsafe { ffi::GenMeshPlane(width, length, res_x as _, res_z as _) },
        }
    }

    /// Generate cuboid mesh
    #[inline]
    pub fn generate_cube(width: f32, height: f32, length: f32) -> Self {
        Self {
            raw: unsafe { ffi::GenMeshCube(width, height, length) },
        }
    }

    /// Generate sphere mesh (standard sphere)
    #[inline]
    pub fn generate_sphere(radius: f32, rings: u32, slices: u32) -> Self {
        Self {
            raw: unsafe { ffi::GenMeshSphere(radius, rings as _, slices as _) },
        }
    }

    /// Generate half-sphere mesh (no bottom cap)
    #[inline]
    pub fn generate_hemisphere(radius: f32, rings: u32, slices: u32) -> Self {
        Self {
            raw: unsafe { ffi::GenMeshHemiSphere(radius, rings as _, slices as _) },
        }
    }

    /// Generate cylinder mesh
    #[inline]
    pub fn generate_cylinder(radius: f32, height: f32, slices: u32) -> Self {
        Self {
            raw: unsafe { ffi::GenMeshCylinder(radius, height, slices as _) },
        }
    }

    /// Generate cone/pyramid mesh
    #[inline]
    pub fn generate_cone(radius: f32, height: f32, slices: u32) -> Self {
        Self {
            raw: unsafe { ffi::GenMeshCone(radius, height, slices as _) },
        }
    }

    /// Generate torus mesh
    #[inline]
    pub fn generate_torus(radius: f32, size: f32, rad_seg: u32, sides: u32) -> Self {
        Self {
            raw: unsafe { ffi::GenMeshTorus(radius, size, rad_seg as _, sides as _) },
        }
    }

    /// Generate trefoil knot mesh
    #[inline]
    pub fn generate_knot(radius: f32, size: f32, rad_seg: u32, sides: u32) -> Self {
        Self {
            raw: unsafe { ffi::GenMeshKnot(radius, size, rad_seg as _, sides as _) },
        }
    }

    /// Generate heightmap mesh from image data
    #[inline]
    pub fn generate_heightmap(heightmap: &Image, size: Vector3) -> Self {
        Self {
            raw: unsafe { ffi::GenMeshHeightmap(heightmap.raw.clone(), size.into()) },
        }
    }

    /// Generate cubes-based map mesh from image data
    #[inline]
    pub fn generate_cubicmap(cubicmap: &Image, cube_size: Vector3) -> Self {
        Self {
            raw: unsafe { ffi::GenMeshCubicmap(cubicmap.raw.clone(), cube_size.into()) },
        }
    }
}

impl Drop for Mesh {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::UnloadMesh(self.raw.clone()) }
    }
}

/// Model, meshes, materials and animation data
#[derive(Debug)]
pub struct Model {
    pub(crate) raw: ffi::Model,
}

impl Model {
    /// Load model from files (meshes and materials)
    #[inline]
    pub fn from_file(file_name: &str) -> Option<Self> {
        let file_name = CString::new(file_name).unwrap();

        let raw = unsafe { ffi::LoadModel(file_name.as_ptr()) };

        if unsafe { ffi::IsModelReady(raw.clone()) } {
            Some(Self { raw })
        } else {
            None
        }
    }

    /// Load model from generated mesh (default material)
    #[inline]
    pub fn from_mesh(mesh: Mesh) -> Self {
        let raw_mesh = mesh.raw.clone();
        // LoadModelFromMesh 'takes ownership' of the mesh
        std::mem::forget(mesh);

        Self {
            raw: unsafe { ffi::LoadModelFromMesh(raw_mesh) },
        }
    }

    /// Compute model bounding box limits (considers all meshes)
    #[inline]
    pub fn get_bounding_box(&self) -> BoundingBox {
        unsafe { ffi::GetModelBoundingBox(self.raw.clone()).into() }
    }

    /// Set material for a mesh
    #[inline]
    pub fn set_mesh_material(&mut self, mesh_id: u32, material_id: u32) {
        unsafe {
            ffi::SetModelMeshMaterial(&mut self.raw as *mut _, mesh_id as _, material_id as _)
        }
    }

    /// Update model animation pose
    #[inline]
    pub fn update_animation(&self, anim: &ModelAnimation, frame: u32) {
        unsafe { ffi::UpdateModelAnimation(self.raw.clone(), anim.raw.clone(), frame as _) }
    }

    /// Check model animation skeleton match
    #[inline]
    pub fn is_animation_valid(&self, anim: &ModelAnimation) -> bool {
        unsafe { ffi::IsModelAnimationValid(self.raw.clone(), anim.raw.clone()) }
    }
}

impl Drop for Model {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::UnloadModel(self.raw.clone()) }
    }
}

/// Material, includes shader and maps
#[derive(Debug)]
pub struct Material {
    pub(crate) raw: ffi::Material,
}

impl Material {
    /// Load materials from model file
    #[inline]
    pub fn from_file(file_name: &str) -> Vec<Self> {
        let file_name = CString::new(file_name).unwrap();
        let mut count: i32 = 0;

        let mats = unsafe { ffi::LoadMaterials(file_name.as_ptr(), &mut count as *mut _) };

        let mut vec = Vec::new();

        for i in 0..(count as usize) {
            let mat = unsafe { mats.add(i).read() };

            if unsafe { ffi::IsMaterialReady(mat.clone()) } {
                vec.push(Self { raw: mat });
            }
        }

        vec
    }

    /// Set texture for a material map type
    ///
    /// Returns `true` on success (if `texture` hasn't been cloned or all of them have been dropped, i.e. if its the underlying `Rc` has only 1 strong reference)
    #[inline]
    pub fn set_texture(&mut self, map_type: MaterialMapIndex, texture: Texture2D) -> bool {
        if Rc::strong_count(&texture.raw) == 1 {
            let raw = texture.raw.clone();
            drop(texture);

            unsafe {
                ffi::SetMaterialTexture(
                    &mut self.raw as *mut _,
                    map_type as _,
                    Rc::into_inner(raw).unwrap(),
                );
            }

            true
        } else {
            false
        }
    }
}

impl Default for Material {
    #[inline]
    fn default() -> Self {
        Self {
            raw: unsafe { ffi::LoadMaterialDefault() },
        }
    }
}

impl Drop for Material {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::UnloadMaterial(self.raw.clone()) }
    }
}

/// Model animation
#[derive(Debug)]
pub struct ModelAnimation {
    raw: ffi::ModelAnimation,
}

impl ModelAnimation {
    /// Load model animations from file
    #[inline]
    pub fn from_file(file_name: &str) -> Vec<Self> {
        let file_name = CString::new(file_name).unwrap();
        let mut count: u32 = 0;

        let anims = unsafe { ffi::LoadModelAnimations(file_name.as_ptr(), &mut count as *mut _) };

        let mut vec = Vec::new();

        for i in 0..(count as usize) {
            vec.push(ModelAnimation {
                raw: unsafe { anims.add(i).read() },
            })
        }

        unsafe {
            ffi::UnloadModelAnimations(anims, count);
        }

        vec
    }
}

impl Drop for ModelAnimation {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::UnloadModelAnimation(self.raw.clone()) }
    }
}
