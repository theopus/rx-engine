use crate::{
    backend
};

use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use interface::{RendererApi, Shader};

pub type DrawIndexed<'d> = (&'d backend::VertexArray, &'d backend::Shader);

pub struct Renderer<'d> {
    api: backend::RendererApi,
    sender: Sender<DrawIndexed<'d>>,
    receiver: Receiver<DrawIndexed<'d>>,
}

impl<'d> Renderer<'d> {
    pub fn new(api: backend::RendererApi) -> Self {
        let (s, r) = mpsc::channel();
        Renderer { api, sender: s, receiver: r }
    }
}

impl<'d> Renderer<'d> {
    pub fn submit(&mut self, vertex_array: &backend::VertexArray, shader: &backend::Shader) {
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

    pub fn api(&self) -> &backend::RendererApi {
        &self.api
    }
}