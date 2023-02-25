use std::{fmt::Debug, mem::ManuallyDrop};

use nalgebra::Scalar;
use serde::{Deserialize, Serialize};

use super::mesh_data::MeshData;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ModelData<T>
    where T: Scalar + Clone + Debug,
{
    pub name: String,
    pub mesh: MeshData<T>,
}