use std::fmt::Debug;

use nalgebra::Scalar;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MeshData<T>
    where T: Scalar + Clone + Debug,
{
    /// Flattened 3 component floating point vectors, storing positions of
    /// vertices in the mesh.
    pub positions: Vec<T>,
    /// Flattened 3 component floating point vectors, storing the color
    /// associated with the vertices in the mesh.
    ///
    /// Most meshes do not have vertex colors. If no vertex colors are specified
    /// this will be empty.
    pub vertex_color: Vec<T>,
    /// Flattened 3 component floating point vectors, storing normals of
    /// vertices in the mesh.
    ///
    /// Not all meshes have normals. If no normals are specified this will
    /// be empty.
    pub normals: Vec<T>,
    /// Flattened 2 component floating point vectors, storing texture
    /// coordinates of vertices in the mesh.
    ///
    /// Not all meshes have texture coordinates. If no texture coordinates are
    /// specified this will be empty.
    pub texcoords: Vec<T>,
    /// Indices for vertices of each face. If loaded with
    /// [`triangulate`](LoadOptions::triangulate) set to `true` each face in the
    /// mesh is a triangle.
    ///
    /// Otherwise [`face_arities`](Mesh::face_arities) indicates how many
    /// indices are used by each face.
    ///
    /// When [`single_index`](LoadOptions::single_index) is set to `true`,
    /// these indices are for *all* of the data in the mesh. Positions,
    /// normals and texture coordinaes.
    /// Otherwise normals and texture coordinates have *their own* indices,
    /// each.
    pub indices: Vec<u32>,
    /// The number of vertices (arity) of each face. *Empty* if loaded with
    /// `triangulate` set to `true` or if the mesh constists *only* of
    /// triangles.
    ///
    /// The offset for the starting index of a face can be found by iterating
    /// through the `face_arities` until reaching the desired face, accumulating
    /// the number of vertices used so far.
    pub face_arities: Vec<u32>,
    /// The indices for vertex colors. Only present when the
    /// [`merging`](LoadOptions::merge_identical_points) feature is enabled, and
    /// empty unless the corresponding load option is set to `true`.
    #[cfg(feature = "merging")]
    pub vertex_color_indices: Vec<u32>,
    /// The indices for texture coordinates. Can be omitted by setting
    /// `single_index` to `true`.
    pub texcoord_indices: Vec<u32>,
    /// The indices for normals. Can be omitted by setting `single_index` to
    /// `true`.
    pub normal_indices: Vec<u32>,
    /// Optional material id associated with this mesh. The material id indexes
    /// into the Vec of Materials loaded from the associated `MTL` file
    pub material_id: Option<usize>,
}
