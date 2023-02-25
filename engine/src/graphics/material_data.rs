use std::{collections::HashMap, fmt::Debug};

use nalgebra::{Vector3, Scalar};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MaterialData<T>
    where T: Scalar + Clone + Debug,
{
    pub name: String,
    pub ambient: Vector3<T>,
    pub diffuse: Vector3<T>,
    pub specular: Vector3<T>,
    /// Material shininess attribute. Also called `glossiness`.
    pub shininess: T,
    /// Dissolve attribute is the alpha term for the material. Referred to as
    /// dissolve since that's what the `MTL` file format docs refer to it as.
    pub dissolve: T,
    /// Optical density also known as index of refraction. Called
    /// `optical_density` in the `MTL` specc. Takes on a value between 0.001
    /// and 10.0. 1.0 means light does not bend as it passes through
    /// the object.
    pub optical_density: T,
    /// Name of the ambient texture file for the material.
    pub ambient_texture: String,
    /// Name of the diffuse texture file for the material.
    pub diffuse_texture: String,
    /// Name of the specular texture file for the material.
    pub specular_texture: String,
    /// Name of the normal map texture file for the material.
    pub normal_texture: String,
    /// Name of the shininess map texture file for the material.
    pub shininess_texture: String,
    /// Name of the alpha/opacity map texture file for the material.
    ///
    /// Referred to as `dissolve` to match the `MTL` file format specification.
    pub dissolve_texture: String,
    /// The illumnination model to use for this material. The different
    /// illumnination models are specified in the [`MTL` spec](http://paulbourke.net/dataformats/mtl/).
    pub illumination_model: Option<u8>,
    // Key value pairs of any unrecognized parameters encountered while parsing
    // the material.
    // pub unknown_param: HashMap<String, String>,
    // TODO
}