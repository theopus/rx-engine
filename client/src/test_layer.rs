use std::fs::canonicalize;
use std::path::Path;
use std::thread;
use std::time::Duration;

use rx_engine::{
    platform::{
        create_pm,
        PlatformManager,
        WindowConfig,
    },
    render::{
        BufferLayout,
        Reloadable,
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
use rx_engine::render::ReloadableShader;
use rx_engine::utils::{relative_path, ResourceListener};

struct TestLayer {
    vertex_array: Box<VertexArray>,
    shader: Box<ReloadableShader>,
    rot: f32
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


        let shader = constructor.reloadable_shader(
            &relative_path(file!(), "/test/vert.glsl"),
            &relative_path(file!(), "/test/frag.glsl"),
            &BufferLayout::with(shared_types::FLOAT_3));

        TestLayer { vertex_array: vertex_array, shader, rot: 0.0 }
    }
}

impl MutLayer for TestLayer {
    fn on_update(&mut self, delta: f64, renderer_api: &mut RendererApi, platform_manager: &mut PlatformManager) {
        self.shader.bind();

        use rx_engine::na::Matrix4;
        use rx_engine::glm;

        let identity: Matrix4<f32> = glm::identity();
        let mtx: Matrix4<f32> = Matrix4::from_euler_angles(0f32, 0f32, self.rot);
        self.rot+=0.001f32;

        self.shader.loadMat4(mtx);


        renderer_api.draw_indexed(self.vertex_array.as_ref());
        //TODO: Unbinding
        self.shader.unbind();
    }
}

pub fn get_layer(r: &RendererApi, rc: &RendererConstructor) -> Box<MutLayer> {
    Box::new(TestLayer::new(r, rc))
}