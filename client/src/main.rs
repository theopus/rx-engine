extern crate rx_engine;

use rx_engine::platform::{create_pm, WindowConfig};
use rx_engine::render::{BufferLayout, RendererType, VertexArray, VertexBuffer};
use rx_engine::render::shared_types;
use rx_engine::run::Layer;
use rx_engine::run::MutLayer;
use rx_engine::render::RendererApi;
use rx_engine::platform::PlatformManager;
use rx_engine::run::MutLayerBuilder;
use rx_engine::render::RendererConstructor;
use rx_engine::run::PushLayer;


struct TestLayer {
    vertex_array: Box<VertexArray>
}

impl TestLayer {
    pub fn new(api: &RendererApi, constructor: &RendererConstructor) -> Self {
        let mut vertex_array: Box<VertexArray> = constructor.vertex_array();
        let mut ib = constructor.index_buffer(&[0, 1, 3, 3, 1, 2]);
        let mut vb: Box<VertexBuffer> = constructor.vertex_buffer();

        vertex_array.set_index_buffer(ib);

        vb.buffer_data_f32(&[
            -0.5_f32, 0.5_f32, 0_f32, // 0

            -0.5_f32, -0.5_f32, 0_f32, // 1

            0.5_f32, -0.5_f32, 0_f32, // 2

            0.5_f32, 0.5_f32, 0_f32, //3
        ]);
        vb.set_buffer_layout(BufferLayout::with(shared_types::FLOAT_3));
        vertex_array.add_vertex_buffer(vb);

        api.set_clear_color(0.3, 0.3, 0.9, 1_f32);

        TestLayer { vertex_array: vertex_array }
    }
}

impl MutLayer for TestLayer {
    fn on_update(&self, delta: f64, renderer_api: &mut RendererApi, platform_manager: &mut PlatformManager) {

        platform_manager.process_events();


        renderer_api.clear_color();
        platform_manager.process_events();
        renderer_api.draw_indexed(self.vertex_array.as_ref());
        renderer_api.swap_buffer();
    }
}

struct TestBuilder;
impl MutLayerBuilder for TestBuilder {
    fn create_layer(&mut self, api: &RendererApi, constructor: &RendererConstructor) -> Box<MutLayer> {
        Box::new(TestLayer::new(api, constructor)) as Box<MutLayer>
    }
}

fn main() {
    let mut engine = rx_engine::run::build_engine(RendererType::OpenGL, WindowConfig { width: 600, height: 400 });
    engine.push_layer(&mut TestBuilder{} as &mut MutLayerBuilder);
    engine.run();
    println!("Bye!")

}
