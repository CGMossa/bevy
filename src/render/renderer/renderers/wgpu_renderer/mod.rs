mod wgpu_render_pass;
mod wgpu_renderer;
mod wgpu_resources;
mod wgpu_type_converter;

pub use wgpu_render_pass::*;
pub use wgpu_renderer::*;
pub use wgpu_resources::*;

use crate::{
    app::{plugin::AppPlugin, system_stage, AppBuilder},
    core::{Event, WindowCreated, WindowResized},
    render::renderer::Renderer,
};

use legion::prelude::*;

#[derive(Default)]
pub struct WgpuRendererPlugin;

impl AppPlugin for WgpuRendererPlugin {
    fn build(&self, app: AppBuilder) -> AppBuilder {
        let render_system = wgpu_render_system(&app.resources);
        app.add_thread_local_to_stage(system_stage::RENDER, render_system)
    }
    fn name(&self) -> &'static str {
        "WgpuRenderer"
    }
}

pub fn wgpu_render_system(resources: &Resources) -> impl FnMut(&mut World, &mut Resources) {
    let window_resized_event = resources.get::<Event<WindowResized>>().unwrap();
    let window_created_event = resources.get::<Event<WindowCreated>>().unwrap();
    let mut wgpu_renderer = futures::executor::block_on(WgpuRenderer::new(
        window_resized_event.get_handle(),
        window_created_event.get_handle(),
    ));
    move |world, resources| {
        wgpu_renderer.update(world, resources);
    }
}