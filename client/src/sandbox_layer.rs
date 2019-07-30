use std::fs::canonicalize;
use std::path::Path;
use std::thread;
use std::time::Duration;

use rx_engine::{
    backend::{
        IndexBuffer,
        interface::{
            shared_types,
            BufferLayout,
            IndexBuffer as IBI,
            PlatformManager as PMI,
            RendererApi as PAI,
            RendererConstructor as RCI,
            Shader as SI,
            VertexArray as VAI,
            VertexBuffer as VBI,
        },
        PlatformManager,
        RendererApi,
        RendererConstructor,
        Shader,
        VertexArray,
        VertexBuffer,
    },
    loader::Loader,
    render::Renderer,
    run::{
        Layer,
        MutLayer,
        MutLayerBuilder,
        PushLayer,
    },
    utils::{
        relative_path,
        relative_to_current_path,
        ResourceListener,
    },
};

struct TestLayer {
    vertex_array: Box<VertexArray>,
    shader: Box<Shader>,
    rot: f32,
}

impl TestLayer {
    pub fn new(renderer: &Renderer, constructor: &RendererConstructor) -> Self {
        let mut path_buf = &relative_to_current_path(&vec!["resources", "tetrahedron.obj"]);
        let mut loader = Loader;
        let result = loader.load_obj(path_buf);

        let mut vertex_array: Box<VertexArray> = Box::new(constructor.vertex_array());
        let mut ib = constructor.index_buffer(&result.indices);
        let mut vb: Box<VertexBuffer> = Box::new(constructor.vertex_buffer());

        vertex_array.set_index_buffer(ib);
        vb.buffer_data_f32(&result.positions);
        vb.set_buffer_layout(BufferLayout::with(shared_types::FLOAT_3));
        vertex_array.add_vertex_buffer(*vb);

        renderer.api().set_clear_color(0.3, 0.3, 0.9, 1_f32);


        let shader: Box<Shader> = Box::from(constructor.reloadable_shader(
            &relative_to_current_path(&vec!["src", "test", "vert.glsl"]),
            &relative_to_current_path(&vec!["src", "test", "frag.glsl"]),
            &BufferLayout::with(shared_types::FLOAT_3))) as Box<Shader>;

        TestLayer { vertex_array: vertex_array, shader, rot: 0.0 }
    }
}

impl MutLayer for TestLayer {
    fn on_update(&mut self, delta: f64, renderer: &mut Renderer, platform_manager: &mut PlatformManager) {
        use rx_engine::na::Matrix4;
        use rx_engine::glm;

        let identity: Matrix4<f32> = glm::identity();
        let mtx: Matrix4<f32> = Matrix4::from_euler_angles(0f32, 0f32, self.rot);
        self.rot += 0.001f32;

        self.shader.bind();
        self.shader.load_mat4(&mtx.as_slice());
        renderer.submit(self.vertex_array.as_ref(), self.shader.as_ref());
    }
}

pub fn get_layer(r: &Renderer, rc: &RendererConstructor) -> Box<MutLayer> {
    Box::new(TestLayer::new(r, rc))
}