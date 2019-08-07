use std::fs::canonicalize;
use std::path::Path;
use std::thread;
use std::time::Duration;

use rx_engine::{
    backend,
    interface::{
        BufferLayout,
        IndexBuffer,
        PlatformManager,
        RendererApi,
        RendererConstructor,
        Shader,
        shared_types,
        VertexArray,
        VertexBuffer,
    },
    loader::Loader,
    render::Renderer,
    run::{
        Layer,
        LayerBuilder,
    },
    utils::{
        relative_path,
        relative_to_current_path,
        ResourceListener,
    },
};
use rx_engine::asset::{AssetHolder, AssetPtr, AssetStorage};
use rx_engine::run::EngineContext;

struct TestLayer {
    va_ptr: AssetPtr<backend::VertexArray>,
    shader: AssetPtr<backend::Shader>,
    rot: f32,
}

impl TestLayer {
    pub fn new(ctx: &mut EngineContext) -> Self {
        let mut path_buf = &relative_to_current_path(&vec!["resources", "tetrahedron.obj"]);
        let mut loader = Loader;
        let result = loader.load_obj(path_buf);

        let mut vertex_array: backend::VertexArray = ctx.renderer_constructor.vertex_array();
        let mut ib = ctx.renderer_constructor.index_buffer(&result.indices);
        let mut vb: Box<backend::VertexBuffer> = Box::new(ctx.renderer_constructor.vertex_buffer());

        vertex_array.set_index_buffer(ib);
        vb.buffer_data_f32(&result.positions);
        vb.set_buffer_layout(BufferLayout::with(shared_types::FLOAT_3));
        vertex_array.add_vertex_buffer(*vb);

        ctx.renderer.api().set_clear_color(0.3, 0.3, 0.9, 1_f32);

        let va_ptr: AssetPtr<backend::VertexArray> = ctx.asset_holder.storage_mut().put(vertex_array);


        let shader: backend::Shader = ctx.renderer_constructor.reloadable_shader(
            &relative_to_current_path(&vec!["src", "test", "vert.glsl"]),
            &relative_to_current_path(&vec!["src", "test", "frag.glsl"]),
            &BufferLayout::with(shared_types::FLOAT_3));

        let shader: AssetPtr<backend::Shader> = ctx.asset_holder.storage_mut().put(shader);

        TestLayer {
            va_ptr,
            shader,
            rot: 0.0,
        }
    }
}

impl Layer for TestLayer {
    fn on_update(&mut self, delta: f64, ctx: &mut EngineContext) {
        use rx_engine::na::Matrix4;
        use rx_engine::glm;

        let identity: Matrix4<f32> = glm::identity();
        let mtx: Matrix4<f32> = Matrix4::from_euler_angles(0f32, 0f32, self.rot);
        self.rot += 0.001f32;

        let shader = ctx.asset_holder.storage().get_ref(&self.shader.clone()).unwrap();
        shader.bind();
        shader.load_mat4("m", &mtx.as_slice());

        ctx.renderer.submit((self.va_ptr.clone(), self.shader.clone()));
    }
}

pub struct SandboxLayerBuilder;

impl<'l> LayerBuilder<'l> for SandboxLayerBuilder {
    fn build(&self, r: &mut EngineContext) -> Box<dyn Layer + 'l> {
        Box::new(TestLayer::new(r))
    }
}