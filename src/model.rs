use std::ffi::CString;

use crate::{ffi, math::BoundingBox};

pub use crate::ffi::MaterialMapIndex;

#[derive(Debug)]
pub struct Mesh {
    pub(crate) raw: ffi::Mesh,
}

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
        Self {
            raw: unsafe { ffi::LoadModelFromMesh(mesh.raw) },
        }
    }

    /// Compute model bounding box limits (considers all meshes)
    #[inline]
    pub fn get_bounding_box(&self) -> BoundingBox {
        unsafe { ffi::GetModelBoundingBox(self.raw.clone()).into() }
    }
}

impl Drop for Model {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::UnloadModel(self.raw.clone()) }
    }
}

#[derive(Debug)]
pub struct Material {
    pub(crate) raw: ffi::Material
}

/*
    /// Upload mesh vertex data in GPU and provide VAO/VBO ids
    #[inline]
    pub fn UploadMesh(mesh: *mut Mesh, dynamic: bool);

    /// Update mesh vertex data in GPU for a specific buffer index
    #[inline]
    pub fn UpdateMeshBuffer(mesh: Mesh, index: i32, data: *const core::ffi::c_void, dataSize: i32, offset: i32);

    /// Unload mesh data from CPU and GPU
    #[inline]
    pub fn UnloadMesh(mesh: Mesh);

    /// Export mesh data to file, returns true on success
    #[inline]
    pub fn ExportMesh(mesh: Mesh, fileName: *const core::ffi::c_char) -> bool;

    /// Compute mesh bounding box limits
    #[inline]
    pub fn GetMeshBoundingBox(mesh: Mesh) -> BoundingBox;

    /// Compute mesh tangents
    #[inline]
    pub fn GenMeshTangents(mesh: *mut Mesh);

    /// Generate polygonal mesh
    #[inline]
    pub fn GenMeshPoly(sides: i32, radius: f32) -> Mesh;

    /// Generate plane mesh (with subdivisions)
    #[inline]
    pub fn GenMeshPlane(width: f32, length: f32, resX: i32, resZ: i32) -> Mesh;

    /// Generate cuboid mesh
    #[inline]
    pub fn GenMeshCube(width: f32, height: f32, length: f32) -> Mesh;

    /// Generate sphere mesh (standard sphere)
    #[inline]
    pub fn GenMeshSphere(radius: f32, rings: i32, slices: i32) -> Mesh;

    /// Generate half-sphere mesh (no bottom cap)
    #[inline]
    pub fn GenMeshHemiSphere(radius: f32, rings: i32, slices: i32) -> Mesh;

    /// Generate cylinder mesh
    #[inline]
    pub fn GenMeshCylinder(radius: f32, height: f32, slices: i32) -> Mesh;

    /// Generate cone/pyramid mesh
    #[inline]
    pub fn GenMeshCone(radius: f32, height: f32, slices: i32) -> Mesh;

    /// Generate torus mesh
    #[inline]
    pub fn GenMeshTorus(radius: f32, size: f32, radSeg: i32, sides: i32) -> Mesh;

    /// Generate trefoil knot mesh
    #[inline]
    pub fn GenMeshKnot(radius: f32, size: f32, radSeg: i32, sides: i32) -> Mesh;

    /// Generate heightmap mesh from image data
    #[inline]
    pub fn GenMeshHeightmap(heightmap: Image, size: Vector3) -> Mesh;

    /// Generate cubes-based map mesh from image data
    #[inline]
    pub fn GenMeshCubicmap(cubicmap: Image, cubeSize: Vector3) -> Mesh;

    /// Load materials from model file
    #[inline]
    pub fn LoadMaterials(fileName: *const core::ffi::c_char, materialCount: *mut i32) -> *mut Material;

    /// Load default material (Supports: DIFFUSE, SPECULAR, NORMAL maps)
    #[inline]
    pub fn LoadMaterialDefault() -> Material;

    /// Check if a material is ready
    #[inline]
    pub fn IsMaterialReady(material: Material) -> bool;

    /// Unload material from GPU memory (VRAM)
    #[inline]
    pub fn UnloadMaterial(material: Material);

    /// Set texture for a material map type (MATERIAL_MAP_DIFFUSE, MATERIAL_MAP_SPECULAR...)
    #[inline]
    pub fn SetMaterialTexture(material: *mut Material, mapType: i32, texture: Texture2D);

    /// Set material for a mesh
    #[inline]
    pub fn SetModelMeshMaterial(model: *mut Model, meshId: i32, materialId: i32);

    /// Load model animations from file
    #[inline]
    pub fn LoadModelAnimations(fileName: *const core::ffi::c_char, animCount: *mut u32) -> *mut ModelAnimation;

    /// Update model animation pose
    #[inline]
    pub fn UpdateModelAnimation(model: Model, anim: ModelAnimation, frame: i32);

    /// Unload animation data
    #[inline]
    pub fn UnloadModelAnimation(anim: ModelAnimation);

    /// Unload animation array data
    #[inline]
    pub fn UnloadModelAnimations(animations: *mut ModelAnimation, count: u32);

    /// Check model animation skeleton match
    #[inline]
    pub fn IsModelAnimationValid(model: Model, anim: ModelAnimation) -> bool;
*/
