use std::{ffi::CString, mem::ManuallyDrop};

use static_assertions::{assert_eq_size, assert_eq_align};

use crate::{
    ffi,
    math::{BoundingBox, Vector3, Vector4, Vector2, Matrix, Transform},
    texture::{Image, Texture2D}, color::Color, shader::Shader,
};

pub use crate::ffi::MaterialMapIndex;

/// Mesh, vertex data and vao/vbo
#[derive(Debug)]
#[repr(transparent)]
pub struct Mesh {
    pub(crate) raw: ffi::Mesh,
}

impl Mesh {
    /// Vertex positions (XYZ - 3 components per vertex) (shader-location = 0)
    #[inline]
    pub fn vertices(&self) -> &[Vector3] {
        unsafe {
            std::slice::from_raw_parts(self.raw.vertices as *const _, self.raw.vertexCount as _)
        }
    }

    /// Vertex positions (XYZ - 3 components per vertex) (shader-location = 0)
    #[inline]
    pub fn vertices_mut(&mut self) -> &mut [Vector3] {
        unsafe {
            std::slice::from_raw_parts_mut(self.raw.vertices as *mut _, self.raw.vertexCount as _)
        }
    }

    /// Vertex texture coordinates (UV - 2 components per vertex) (shader-location = 1)
    #[inline]
    pub fn texcoords(&self) -> &[Vector2] {
        unsafe {
            std::slice::from_raw_parts(self.raw.texcoords as *const _, self.raw.vertexCount as _)
        }
    }

    /// Vertex texture coordinates (UV - 2 components per vertex) (shader-location = 1)
    #[inline]
    pub fn texcoords_mut(&mut self) -> &mut [Vector2] {
        unsafe {
            std::slice::from_raw_parts_mut(self.raw.texcoords as *mut _, self.raw.vertexCount as _)
        }
    }

    /// Vertex texture second coordinates (UV - 2 components per vertex) (shader-location = 5)
    #[inline]
    pub fn texcoords2(&self) -> &[Vector2] {
        unsafe {
            std::slice::from_raw_parts(self.raw.texcoords as *const _, self.raw.vertexCount as _)
        }
    }

    /// Vertex texture second coordinates (UV - 2 components per vertex) (shader-location = 5)
    #[inline]
    pub fn texcoords2_mut(&mut self) -> &mut [Vector2] {
        unsafe {
            std::slice::from_raw_parts_mut(self.raw.texcoords as *mut _, self.raw.vertexCount as _)
        }
    }

    /// Vertex normals (XYZ - 3 components per vertex) (shader-location = 2)
    #[inline]
    pub fn normals(&self) -> &[Vector3] {
        unsafe {
            std::slice::from_raw_parts(self.raw.normals as *const _, self.raw.vertexCount as _)
        }
    }

    /// Vertex normals (XYZ - 3 components per vertex) (shader-location = 2)
    #[inline]
    pub fn normals_mut(&mut self) -> &mut [Vector3] {
        unsafe {
            std::slice::from_raw_parts_mut(self.raw.normals as *mut _, self.raw.vertexCount as _)
        }
    }

    /// Vertex tangents (XYZW - 4 components per vertex) (shader-location = 4)
    #[inline]
    pub fn tangents(&self) -> &[Vector4] {
        unsafe {
            std::slice::from_raw_parts(self.raw.tangents as *const _, self.raw.vertexCount as _)
        }
    }

    /// Vertex tangents (XYZW - 4 components per vertex) (shader-location = 4)
    #[inline]
    pub fn tangents_mut(&mut self) -> &mut [Vector4] {
        unsafe {
            std::slice::from_raw_parts_mut(self.raw.tangents as *mut _, self.raw.vertexCount as _)
        }
    }

    /// Vertex colors (RGBA - 4 components per vertex) (shader-location = 3)
    #[inline]
    pub fn colors(&self) -> &[Color] {
        unsafe {
            std::slice::from_raw_parts(self.raw.colors as *const _, self.raw.vertexCount as _)
        }
    }

    /// Vertex colors (RGBA - 4 components per vertex) (shader-location = 3)
    #[inline]
    pub fn colors_mut(&mut self) -> &mut [Color] {
        unsafe {
            std::slice::from_raw_parts_mut(self.raw.colors as *mut _, self.raw.vertexCount as _)
        }
    }

    /// Vertex indices (in case vertex data comes indexed)
    #[inline]
    pub fn indices(&self) -> &[u16] {
        unsafe {
            std::slice::from_raw_parts(self.raw.indices as *const _, self.raw.triangleCount as _)
        }
    }

    /// Vertex indices (in case vertex data comes indexed)
    #[inline]
    pub fn indices_mut(&mut self) -> &mut [u16] {
        unsafe {
            std::slice::from_raw_parts_mut(self.raw.indices as *mut _, self.raw.triangleCount as _)
        }
    }

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

    /// Get the 'raw' ffi type
    /// Take caution when cloning so it doesn't outlive the original
    #[inline]
    pub fn as_raw(&self) -> &ffi::Mesh {
        &self.raw
    }

    /// Get the 'raw' ffi type
    /// Take caution when cloning so it doesn't outlive the original
    #[inline]
    pub fn as_raw_mut(&mut self) -> &mut ffi::Mesh {
        &mut self.raw
    }

    /// Convert a 'raw' ffi object to a safe wrapper
    ///
    /// # Safety
    /// * The raw object must be correctly initialized
    /// * The raw object should be unique. Otherwise, make sure its clones don't outlive the newly created object.
    #[inline]
    pub unsafe fn from_raw(raw: ffi::Mesh) -> Self {
        Self { raw }
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
#[repr(transparent)]
pub struct Model {
    pub(crate) raw: ffi::Model,
}

impl Model {
	/// Local transform matrix
    #[inline]
    pub fn transform(&self) -> Matrix {
        self.raw.transform.clone().into()
    }
    
	/// Local transform matrix
    #[inline]
    pub fn set_transform(&mut self, mat: Matrix) {
        self.raw.transform = mat.into();
    }

	/// Meshes array
    #[inline]
    pub fn meshes(&self) -> &[ManuallyDrop<Mesh>] {
        unsafe { std::slice::from_raw_parts(self.raw.meshes as *const _, self.raw.meshCount as _) }
    }

    /// Meshes array
    #[inline]
    pub fn meshes_mut(&mut self) -> &mut [ManuallyDrop<Mesh>] {
        unsafe { std::slice::from_raw_parts_mut(self.raw.meshes as *mut _, self.raw.meshCount as _) }
    }

	/// Materials array
    #[inline]
    pub fn materials(&self) -> &[ManuallyDrop<Material>] {
        unsafe { std::slice::from_raw_parts(self.raw.materials as *const _, self.raw.materialCount as _) }
    }

	/// Materials array
    #[inline]
    pub fn materials_mut(&mut self) -> &mut [ManuallyDrop<Material>] {
        unsafe { std::slice::from_raw_parts_mut(self.raw.materials as *mut _, self.raw.materialCount as _) }
    }

    /// Bones information (skeleton)
    #[inline]
    pub fn bones(&self) -> &[ffi::BoneInfo] {
        unsafe { std::slice::from_raw_parts(self.raw.bones as *const _, self.raw.boneCount as _) }
    }

	/// Bones information (skeleton)
    #[inline]
    pub fn bones_mut(&mut self) -> &mut [ffi::BoneInfo] {
        unsafe { std::slice::from_raw_parts_mut(self.raw.bones, self.raw.boneCount as _) }
    }

	/// Bones base transformation (pose)
    #[inline]
    pub fn bind_pose(&self) -> &[Transform] {
        unsafe { std::slice::from_raw_parts(self.raw.bindPose as *const _, self.raw.boneCount as _) }
    }

	/// Bones base transformation (pose)
    #[inline]
    pub fn bind_pose_mut(&mut self) -> &mut [Transform] {
        unsafe { std::slice::from_raw_parts_mut(self.raw.bindPose as *mut _, self.raw.boneCount as _) }
    }

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
        let mesh = ManuallyDrop::new(mesh);

        Self {
            raw: unsafe { ffi::LoadModelFromMesh(mesh.raw.clone()) },
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

    /// Get the 'raw' ffi type
    /// Take caution when cloning so it doesn't outlive the original
    #[inline]
    pub fn as_raw(&self) -> &ffi::Model {
        &self.raw
    }

    /// Get the 'raw' ffi type
    /// Take caution when cloning so it doesn't outlive the original
    #[inline]
    pub fn as_raw_mut(&mut self) -> &mut ffi::Model {
        &mut self.raw
    }

    /// Convert a 'raw' ffi object to a safe wrapper
    ///
    /// # Safety
    /// * The raw object must be correctly initialized
    /// * The raw object should be unique. Otherwise, make sure its clones don't outlive the newly created object.
    #[inline]
    pub unsafe fn from_raw(raw: ffi::Model) -> Self {
        Self { raw }
    }
}

impl Drop for Model {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::UnloadModel(self.raw.clone()) }
    }
}

/// Material map
#[repr(C)]
#[derive(Debug)]
pub struct MaterialMap {
	/// Material map texture
	pub texture: ManuallyDrop<Texture2D>,
	/// Material map color
	pub color: Color,
	/// Material map value
	pub value: f32,
}

assert_eq_size!(MaterialMap, ffi::MaterialMap);
assert_eq_align!(MaterialMap, ffi::MaterialMap);

/// Material, includes shader and maps
#[derive(Debug)]
#[repr(transparent)]
pub struct Material {
    pub(crate) raw: ffi::Material,
}

impl Material {
	/// Material shader
    #[inline]
    pub fn shader(&self) -> &ManuallyDrop<Shader> {
        unsafe { std::mem::transmute(&self.raw.shader) }
    }

    /// Material shader
    #[inline]
    pub fn shader_mut(&mut self) -> &mut ManuallyDrop<Shader> {
        unsafe { std::mem::transmute(&mut self.raw.shader) }
    }

    /// Material maps array
    #[inline]
    pub fn maps(&self) -> &[MaterialMap] {
        unsafe { std::slice::from_raw_parts(self.raw.maps as *const _, ffi::MAX_MATERIAL_MAPS) }
    }

    /// Material maps array
    #[inline]
    pub fn maps_mut(&mut self) -> &mut [MaterialMap] {
        unsafe { std::slice::from_raw_parts_mut(self.raw.maps as *mut _, ffi::MAX_MATERIAL_MAPS) }
    }

    /// Material generic parameters (if required)
    #[inline]
    pub fn params(&self) -> &[f32; 4] {
        &self.raw.params
    }

    /// Material generic parameters (if required)
    #[inline]
    pub fn params_mut(&mut self) -> &mut [f32; 4] {
        &mut self.raw.params
    }

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
    #[inline]
    pub fn set_texture(&mut self, map_type: MaterialMapIndex, texture: Texture2D) {
        let texture = ManuallyDrop::new(texture);

        unsafe {
            ffi::SetMaterialTexture(
                &mut self.raw as *mut _,
                map_type as _,
                texture.raw.clone(),
            );
        }
    }

    /// Get the 'raw' ffi type
    /// Take caution when cloning so it doesn't outlive the original
    #[inline]
    pub fn as_raw(&self) -> &ffi::Material {
        &self.raw
    }

    /// Get the 'raw' ffi type
    /// Take caution when cloning so it doesn't outlive the original
    #[inline]
    pub fn as_raw_mut(&mut self) -> &mut ffi::Material {
        &mut self.raw
    }

    /// Convert a 'raw' ffi object to a safe wrapper
    ///
    /// # Safety
    /// * The raw object must be correctly initialized
    /// * The raw object should be unique. Otherwise, make sure its clones don't outlive the newly created object.
    #[inline]
    pub unsafe fn from_raw(raw: ffi::Material) -> Self {
        Self { raw }
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
#[repr(transparent)]
pub struct ModelAnimation {
    raw: ffi::ModelAnimation,
}

impl ModelAnimation {
    /// Bones information (skeleton)
    #[inline]
    pub fn bones(&self) -> &[ffi::BoneInfo] {
        unsafe { std::slice::from_raw_parts(self.raw.bones as *const _, self.raw.boneCount as _) }
    }

	/// Bones information (skeleton)
    #[inline]
    pub fn bones_mut(&mut self) -> &mut [ffi::BoneInfo] {
        unsafe { std::slice::from_raw_parts_mut(self.raw.bones, self.raw.boneCount as _) }
    }

	/// Poses array by frame
    #[inline]
    pub fn frame_poses(&self) -> Vec<&[Transform]> {
        let mut vec = Vec::new();

        for i in 0..(self.raw.frameCount as usize) {
            vec.push(unsafe { std::slice::from_raw_parts(self.raw.framePoses.add(i).read() as *const _, self.raw.boneCount as _) })
        }

        vec
    }

	/// Poses array by frame
    #[inline]
    pub fn frame_poses_mut(&mut self) -> Vec<&mut [Transform]> {
        let mut vec = Vec::new();

        for i in 0..(self.raw.frameCount as usize) {
            vec.push(unsafe { std::slice::from_raw_parts_mut(self.raw.framePoses.add(i).read() as *mut _, self.raw.boneCount as _) })
        }

        vec
    }
    
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

    /// Get the 'raw' ffi type
    /// Take caution when cloning so it doesn't outlive the original
    #[inline]
    pub fn as_raw(&self) -> &ffi::ModelAnimation {
        &self.raw
    }

    /// Get the 'raw' ffi type
    /// Take caution when cloning so it doesn't outlive the original
    #[inline]
    pub fn as_raw_mut(&mut self) -> &mut ffi::ModelAnimation {
        &mut self.raw
    }

    /// Convert a 'raw' ffi object to a safe wrapper
    ///
    /// # Safety
    /// * The raw object must be correctly initialized
    /// * The raw object should be unique. Otherwise, make sure its clones don't outlive the newly created object.
    #[inline]
    pub unsafe fn from_raw(raw: ffi::ModelAnimation) -> Self {
        Self { raw }
    }
}

impl Drop for ModelAnimation {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::UnloadModelAnimation(self.raw.clone()) }
    }
}
