extern crate rx_engine;

use rx_engine::{
    platform::{
        create_pm,
        PlatformManager,
        WindowConfig,
    },
    render::{
        BufferLayout,
        RendererApi,
        RendererConstructor,
        RendererType,
        Shader,
        shared_types,
        VertexArray,
        VertexBuffer,
    },
    run::{
        Layer,
        MutLayer,
        MutLayerBuilder,
        PushLayer,
    },
};

struct TestLayer {
    vertex_array: Box<VertexArray>,
    shader: Box<Shader>,
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

        let vert_shader = r#"
            #version 330 core
            layout (location = 0) in vec3 Position;

            void main() {
                gl_Position = vec4(Position, 1.0);
            }
        "#;

        let frag_shader = r#"
            #version 330 core
            out vec4 Color;

            void main(){
                Color = vec4(1.0f, 0.5f, 0.2f, 1.0f);
            }
        "#;

        let shader = constructor.shader(vert_shader,
                                        frag_shader,
                                        &BufferLayout::with(shared_types::FLOAT_3));

        TestLayer { vertex_array: vertex_array, shader }
    }
}

impl MutLayer for TestLayer {
    fn on_update(&self, delta: f64, renderer_api: &mut RendererApi, platform_manager: &mut PlatformManager) {
        platform_manager.process_events();
        renderer_api.clear_color();
        platform_manager.process_events();

        self.shader.bind();
        renderer_api.draw_indexed(self.vertex_array.as_ref());
        //TODO: Unbinding
        self.shader.unbind();

        renderer_api.swap_buffer();
    }
}

struct TestBuilder;

impl TestBuilder {}

impl MutLayerBuilder for TestBuilder {
    fn create_layer(&mut self, api: &RendererApi, constructor: &RendererConstructor) -> Box<MutLayer> {
        Box::new(TestLayer::new(api, constructor)) as Box<MutLayer>
    }
}

fn main() {
    let mut engine = rx_engine::run::build_engine(RendererType::OpenGL, WindowConfig { width: 600, height: 400 });
    engine.push_layer(&mut TestBuilder {} as &mut MutLayerBuilder);
    engine.run();
    println!("Bye!")
}
