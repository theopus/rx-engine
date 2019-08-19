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
    },
};
use rx_engine::asset::{AssetHolder, AssetPtr, AssetStorage};
use rx_engine::imgui;
use rx_engine::run::{EngineContext, FrameContext};

struct TestLayer {
    va_ptr: AssetPtr<backend::VertexArray>,
    shader: AssetPtr<backend::Shader>,
    rot: f64,
}

impl TestLayer {
    pub fn new(ctx: &mut EngineContext) -> Self {
        let mut path_buf = &relative_to_current_path(&vec!["client", "resources", "cube.obj"]);
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


        let shader: backend::Shader = ctx.renderer_constructor.shader(
            &relative_to_current_path(&vec!["client", "src", "test", "vert.glsl"]),
            &relative_to_current_path(&vec!["client", "src", "test", "frag.glsl"]),
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
    fn on_update(&mut self, frame: &mut FrameContext, ctx: &mut EngineContext) {
        use rx_engine::na::Matrix4;
        use rx_engine::glm;

        let identity: Matrix4<f32> = glm::identity();
        let proj = glm::perspective(
            6./4.,
            glm::radians(&glm::vec1(30.)).x,
            0.1,
                1000.,
        );
        self.rot += 0.5 * frame.elapsed;

        let view: Matrix4<f32> = glm::look_at(
            &glm::vec3(0.,0.,5.),
            &glm::vec3(0.,0.,0.),
            &glm::vec3(0.,1.,0.),
        );
        let mut mtx: Matrix4<f32> = Matrix4::from_euler_angles(0f32, self.rot as f32, 0.);
        mtx = glm::translate(&mut mtx, &glm::vec3(0., 0., 0.));

        frame.frame.set_projection_matrix(proj);
        frame.frame.set_view_matrix(view);

        let ui = &frame.ui;
        ui.window(imgui::im_str!("Info"))
            .size([300.0, 300.0], imgui::Condition::Once)
            .position([1.0, 1.0], imgui::Condition::Always)
            .build(|| {
                let io: &imgui::Io = ui.io();
                ui.text(imgui::im_str!("{:.1} fps", ui.imgui().get_frame_rate()));
                ui.text(imgui::im_str!("{:.1} ms/f", io.delta_time * 1000.));
                let mouse_pos = ui.imgui().mouse_pos();

                let [w, h] = io.display_size;
                let (fw, fh) = {
                    let [ws, hs] = io.display_framebuffer_scale;
                    (w * ws, h * hs)
                };
                ui.separator();
                ui.text(imgui::im_str!("{:.0} x {:.0} window", w,h));
                ui.text(imgui::im_str!("{:.0} x {:.0} framebuffer", fw,fh));
                ui.text(imgui::im_str!("Mouse Position: ({:.1},{:.1})", mouse_pos.0, mouse_pos.1));
            });
        let shader = ctx.asset_holder.storage().get_ref(&self.shader.clone()).unwrap();
        shader.bind();
        shader.load_mat4("m", mtx.as_slice());

        ctx.renderer.submit((self.va_ptr.clone(), self.shader.clone()));
    }
}

pub struct SandboxLayerBuilder;

impl<'l> LayerBuilder<'l> for SandboxLayerBuilder {
    fn build(&self, r: &mut EngineContext) -> Box<dyn Layer + 'l> {
        Box::new(TestLayer::new(r))
    }
}