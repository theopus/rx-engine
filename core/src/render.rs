use std::{
    sync::{
        mpsc,
        mpsc::Receiver,
        mpsc::Sender,
    }
};

use interface::{RendererApi, Shader};

use crate::asset::{AssetHolder, AssetPtr};
use crate::Matrix4f;

pub type DrawIndexed = (AssetPtr<backend::VertexArray>, AssetPtr<backend::Shader>, Matrix4f);

pub struct Renderer {
    api: backend::RendererApi,
    sender: Sender<DrawIndexed>,
    receiver: Receiver<DrawIndexed>,

    last_frame: Frame,
}

impl Renderer {
    pub fn new(api: backend::RendererApi) -> Self {
        let (s, r) = mpsc::channel();
        Renderer {
            api,
            sender: s.clone(),
            receiver: r,
            last_frame: Frame {
                queue: s.clone(),
                view: glm::identity(),
                projection: glm::identity()
            },
        }
    }
}

pub struct Frame {
    queue: Sender<DrawIndexed>,
    view: Matrix4f,
    projection: Matrix4f,
}

impl Frame {
    pub fn queue(&self) -> Sender<DrawIndexed> {
        self.queue.clone()
    }

    pub fn set_view_matrix(&mut self, mtx: Matrix4f) {
        self.view = mtx
    }

    pub fn set_projection_matrix(&mut self, mtx: Matrix4f) {
        self.projection = mtx
    }
}

impl Renderer {
    pub fn submit(&mut self, cmd: DrawIndexed) {
        self.sender.send(cmd);
    }

    pub fn process(&self, ctx: &mut AssetHolder, frame: &mut Frame) {
        for cmd in self.receiver.try_iter() {

            let va: &backend::VertexArray = ctx.storage().get_ref(&cmd.0).unwrap();
            let shader: &backend::Shader = ctx.storage().get_ref(&cmd.1).unwrap();

            shader.bind();
            shader.load_mat4("r_view", frame.view.as_slice());
            shader.load_mat4("r_projection", frame.projection.as_slice());
            shader.load_mat4("r_vp", (frame.projection * frame.view).as_slice());
            shader.load_mat4("r_transformation", cmd.2.as_slice());
            self.api.draw_indexed(va);
            shader.unbind();
        }
    }

    pub fn viewport(&self, w: i32, h: i32) {
        self.api.viewport(w, h);
    }

    pub fn start(&mut self) -> Frame {
        self.api.clear_color();
        Frame {
            queue: self.sender.clone(),
            view: self.last_frame.view,
            projection: self.last_frame.projection,
        }
    }

    pub fn process_frame(&mut self, frame: &mut Frame, ctx: &mut AssetHolder) {
        self.process(ctx, frame);
    }

    pub fn end(&mut self, frame: Frame) {
        self.api.swap_buffer();
        self.last_frame = frame;
    }

    pub fn get_submitter(&self) -> Sender<DrawIndexed> {
        self.sender.clone()
    }

    pub fn api(&self) -> &backend::RendererApi {
        &self.api
    }
}