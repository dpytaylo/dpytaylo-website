pub mod resource;

use std::cell::RefCell;
use std::collections::HashMap;
use std::future::Future;
use std::io::{BufReader, Cursor};
use std::mem;

use anyhow::Context;
use gloo::net::http::Request;
use itertools::izip;
use nalgebra::{Vector3, Vector2};
use tobj::GPU_LOAD_OPTIONS;
use web_sys::WebGl2RenderingContext;

use crate::graphics::Graphics;
use crate::graphics::material_data::MaterialData;
use crate::graphics::mesh_data::MeshData;
use crate::graphics::model_data::ModelData;
use crate::graphics::pnt_vertex::PntVertex;
use crate::graphics::vertex::Vertex;
use crate::material_render_state::MaterialRenderState;
use crate::model3d::Model3d;
use crate::object::Object;
use crate::scene::Scene;
use crate::utils::smart_pointers::crc_vec::{RawCrcVec, CrcVec};

use self::resource::Resource;

pub struct ResourceManager {
    gl: WebGl2RenderingContext,
    vertices: RefCell<HashMap<String, Resource<RawCrcVec>>>,
}

impl ResourceManager {
    pub fn new(gl: WebGl2RenderingContext) -> Self {
        Self {
            gl,
            vertices: RefCell::default(),
        }
    }

    pub async fn get_vertices<Str, T, F, Fut>(&self, name: Str, mut if_not_found: F) -> Result<CrcVec<T>, ()>
        where Str: AsRef<str> + Into<String>,
              T: Vertex + 'static,
              F: FnMut() -> Fut,
              Fut: Future<Output = Result<Vec<T>, ()>>,
    {
        if let Some(val) = self.vertices.borrow().get(name.as_ref()) {
            return val.clone_ptr();
        };

        let value = if_not_found().await?;
        let (resource, pointer) = Resource::new(value);

        self.vertices.borrow_mut().insert(name.into(), resource);
        Ok(pointer)
    }

    pub async fn load(&self, path: &str) -> Result<Vec<u8>, gloo::net::Error> {
        Request::get(path).send().await?.binary().await
    }

    pub async fn load_str(&self, path: &str) -> anyhow::Result<String> {
        Ok(String::from_utf8(self.load(path).await?)?)
    }

    pub async fn load_obj_mtl(&self, path_to_obj: &str, path_to_mtl: &str) -> anyhow::Result<(Vec<ModelData<f32>>, Vec<MaterialData<f32>>)> {
        let obj = Request::get(path_to_obj).send().await?;
        let obj = obj.binary().await?;
        let mut obj_buf = BufReader::new(Cursor::new(obj));

        let mtl = Request::get(path_to_mtl).send().await?;
        let mtl = mtl.binary().await?;

        let (models, materials) = tobj::load_obj_buf_async(&mut obj_buf, &GPU_LOAD_OPTIONS, move |_| {
            let mtl = tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(&mtl)));
            async move { mtl }
        }).await?;

        let materials = materials?;

        let mut model_data = Vec::with_capacity(models.len());
        for model in models {
            model_data.push(
                ModelData {
                    name: model.name,
                    mesh: MeshData {
                        positions: model.mesh.positions,
                        vertex_color: model.mesh.vertex_color,
                        normals: model.mesh.normals,
                        texcoords: model.mesh.texcoords,
                        indices: model.mesh.indices,
                        face_arities: model.mesh.face_arities,
                        texcoord_indices: model.mesh.texcoord_indices,
                        normal_indices: model.mesh.normal_indices,
                        material_id: model.mesh.material_id,
                    }
                }
            )
        }

        let mut material_data = Vec::with_capacity(materials.len());
        for material in materials {
            material_data.push(
                MaterialData {
                    name: material.name,
                    ambient: Vector3::from_row_slice(&material.ambient),
                    diffuse: Vector3::from_row_slice(&material.diffuse),
                    specular: Vector3::from_row_slice(&material.specular),
                    shininess: material.shininess,
                    dissolve: material.dissolve,
                    optical_density: material.optical_density,
                    ambient_texture: material.ambient_texture,
                    diffuse_texture: material.diffuse_texture,
                    specular_texture: material.specular_texture,
                    normal_texture: material.normal_texture,
                    shininess_texture: material.shininess_texture,
                    dissolve_texture: material.dissolve_texture,
                    illumination_model: material.illumination_model,
                }
            )
        }        

        Ok((model_data, material_data))
    }

    pub async fn load_raw_scene_data(&self, path: &str) -> anyhow::Result<(Vec<ModelData<f32>>, Vec<MaterialData<f32>>)> {
        let data = Request::get(path).send().await?;
        let data = data.binary().await?;

        let result: (Vec<ModelData<f32>>, Vec<MaterialData<f32>>) = bincode::deserialize(&data)?;
        Ok(result)
    }

    pub async fn load_texture(&self, path: &str) {
        //Request::get(path)
    }

    pub async fn load_scene(&self, graphics: &Graphics, data: (Vec<ModelData<f32>>, Vec<MaterialData<f32>>)) -> anyhow::Result<Scene> {
        let (models, materials) = data;

        let mut objects: Vec<Box<dyn Object>> = Vec::with_capacity(models.len());
        for mut model in models {
            log::info!("{}", model.name);
            
            let mut vertices = Vec::with_capacity(model.mesh.positions.len() / 3);

            for (pos, norm, tex) in izip!(
                model.mesh.positions.chunks_exact(3),
                model.mesh.normals.chunks_exact(3),
                model.mesh.texcoords.chunks_exact(2)
            )
            {
                vertices.push(PntVertex::new(
                    Vector3::from_row_slice(pos),
                    Vector3::from_row_slice(norm),
                    Vector2::from_row_slice(tex),
                ));
            }

            let indices = mem::replace(&mut model.mesh.indices, Vec::new());

            let material_id = model.mesh.material_id.context("No material id")?;
            objects.push(
                Box::new(Model3d::new(
                    vertices,
                    indices,
                    MaterialRenderState::new(self, graphics, materials[material_id].clone()).await?,
                )),
            );
        }

        Ok(Scene::from_vec(objects))
    }
}