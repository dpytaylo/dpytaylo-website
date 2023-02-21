pub mod resource;

use std::cell::RefCell;
use std::collections::HashMap;
use std::future::Future;
use std::io::{BufReader, Cursor};
use std::mem;
use std::rc::Rc;

use anyhow::Context;
use gloo::net::http::Request;
use itertools::izip;
use nalgebra::{Vector3, Vector2};
use tobj::{GPU_LOAD_OPTIONS, Model, LoadError};
use web_sys::WebGl2RenderingContext;

use crate::graphics::graphics_context::GraphicsContext;
use crate::graphics::pnt_vertex::PntVertex;
use crate::graphics::vertex::Vertex;
use crate::material_render_state::MaterialRenderState;
use crate::model3d::Model3d;
use crate::object::Object;
use crate::utils::smart_pointers::crc_vec::{RawCrcVec, CrcVec};
use crate::world::World;

use self::resource::Resource;

pub struct ResourceManager {
    gl: WebGl2RenderingContext,

    //raw_data: RefCell<HashMap<TypeId, RefCell<Vec<Box<dyn Any>>>>>,
    vertices: RefCell<HashMap<String, Resource<RawCrcVec>>>,
    //shader_programs: HashMap<String, Resource>,

    // TODO (?)
    // graphics_context: GraphicsContext,
}

impl ResourceManager {
    pub fn new(gl: WebGl2RenderingContext) -> Rc<Self> {
        Rc::new(Self {
            gl,

            //raw_data: RefCell::default(),
            vertices: RefCell::default(),
            //shader_programs: HashMap::default(),
        })
    }

    // pub fn get<T>(&self) -> Option<&T> {
    //     self.raw_data
    //         .borrow()
    //         .get(&TypeId::of::<T>())
    //         .map(|val| val.downcast_ref())
    //         .flatten()
    // }

    // pub fn get_mut<T>(&self) {
    //     unimplemented!();
    // }

    // TODO replace this function: make get_rc_vector of vertices not mesh (!)
    // Issue: mvp matrix
    // pub async fn get_mesh<Str, T, F, Fut>(&self, name: Str, mut if_not_found: F) -> Result<Res<Mesh<T>>, ()>
    //     where Str: AsRef<str> + ToString,
    //           T: Vertex + 'static,
    //           F: FnMut() -> Fut,
    //           Fut: Future<Output = Result<Vec<T>, ()>>,
    // {
    //     if let Some(val) = self.meshes.borrow().get(name.as_ref()) {
    //         return Ok(val.new_pointer::<Mesh<T>>()?);
    //     };

    //     let value = if_not_found().await?;
    //     let resource = Resource::new(Mesh::new(&self.gl, value));
    //     let result = resource.new_pointer::<Mesh<T>>()?;

    //     self.meshes.borrow_mut().insert(name.to_string(), resource);
    //     Ok(result)
    // }

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

    pub async fn load_obj_mtl(&self, path_to_obj: &str, path_to_mtl: &str) -> anyhow::Result<(Vec<Model>, Result<Vec<tobj::Material>, LoadError>)> {
        let obj = Request::get(path_to_obj).send().await?;
        let obj = obj.binary().await?;
        let mut obj_buf = BufReader::new(Cursor::new(obj));

        let mtl = Request::get(path_to_mtl).send().await?;
        let mtl = mtl.binary().await?;

        Ok(tobj::load_obj_buf_async(&mut obj_buf, &GPU_LOAD_OPTIONS, move |_| {
            let mtl = tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(&mtl)));
            async move { mtl }
        }).await?)
    }

    pub async fn load_texture(&self, path: &str) {
        //Request::get(path)
    }

    pub async fn load_world(&self, graphics: &GraphicsContext, data: (Vec<Model>, Result<Vec<tobj::Material>, LoadError>)) -> anyhow::Result<World> {
        let (models, materials) = (data.0, data.1?);

        let mut objects: Vec<Rc<dyn Object>> = Vec::with_capacity(models.len());
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
                Rc::new(Model3d::new(
                    vertices,
                    indices,
                    MaterialRenderState::new(self, graphics, materials[material_id].clone()).await?,
                )),
            );
        }

        Ok(World::from_vec(objects))
    }
}