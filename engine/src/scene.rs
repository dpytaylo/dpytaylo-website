use crate::graphics::render_config::RenderConfig;
use crate::graphics::scene_render_data::SceneRenderData;
use crate::graphics::render_data::RenderData;
use crate::object::Object;

#[derive(Default)]
pub struct Scene {
    pub objects: Vec<Box<dyn Object>>,
    pub(crate) render_info: Vec<RenderInfo>,
}

pub(crate) struct RenderInfo {
    owner_id: u64,
    pub render_data: SceneRenderData,
}

impl Scene {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_vec(objects: Vec<Box<dyn Object>>) -> Self {
        let mut render_info = Vec::with_capacity(objects.len()); // TODO
        
        for object in &objects {
            let data = object.on_add_in_scene();
            
            if let Some(render_data) = data.render_data {
                let RenderData { config, mesh, render_state } = render_data;

                let scene_render_data = SceneRenderData::new(
                    config,
                    vec![mesh],
                    render_state,
                );

                let raw_object: *const dyn Object = &**object;
                render_info.push(RenderInfo::new(
                    // Casting from the fat pointer to the thin pointer
                    raw_object as *const () as u64,
                    scene_render_data,
                ));
            }
        }

        log::info!("{}", render_info.len());

        Self {
            objects,
            render_info,
        }
    }

    pub fn add_object<T>(&mut self, object: Box<T>)
        where T: Object + 'static,
    {
        let data = object.on_add_in_scene();
        
        if let Some(render_data) = data.render_data {
            let RenderData { config, mesh, render_state } = render_data;

            let layer_render_data = SceneRenderData::new(
                config,
                vec![mesh],
                render_state,
            );

            let raw_object: *const T = &*object;
            self.render_info.push(RenderInfo::new(
                raw_object as u64,
                layer_render_data,
            ));
        }

        self.objects.push(object);
    }
}

impl RenderInfo {
    fn new(owner_id: u64, render_data: SceneRenderData) -> Self {
        Self {
            owner_id,
            render_data,
        }
    }
}