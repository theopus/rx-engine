use crate::{
    backend::{
        Shader,
        RendererApi,
        VertexArray,
        interface::Shader as ShaderInterface,
        interface::RendererApi as RendererApiInterface,
    }
};

use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;

pub type DrawIndexed<'d> = (&'d VertexArray, &'d Shader);

pub struct Renderer<'d> {
    api: RendererApi,
    sender: Sender<DrawIndexed<'d>>,
    receiver: Receiver<DrawIndexed<'d>>,
}

impl<'d> Renderer<'d> {
    pub fn new(api: RendererApi) -> Self {
        let (s, r) = mpsc::channel();
        Renderer { api, sender: s, receiver: r }
    }
}

impl<'d> Renderer<'d> {
    pub fn submit(&mut self, vertex_array: &VertexArray, shader: &Shader) {
        shader.bind();
        self.api.draw_indexed(vertex_array);
        shader.unbind();
    }

    pub fn start(&self) {
        self.api.clear_color();
    }
    pub fn end(&mut self) {
        self.api.swap_buffer();
    }

    pub fn get_submitter(&self) -> Sender<DrawIndexed<'d>> {
        self.sender.clone()
    }

    pub fn api(&self) -> &RendererApi {
        &self.api
    }
}