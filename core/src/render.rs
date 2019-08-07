use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use interface::{RendererApi, Shader};

use crate::asset::{AssetPtr, AssetHolder};
use crate::run::EngineContext;

pub type DrawIndexed = (AssetPtr<backend::VertexArray>, AssetPtr<backend::Shader>);

pub struct Renderer {
    api: backend::RendererApi,
    sender: Sender<DrawIndexed>,
    receiver: Receiver<DrawIndexed>,
}

impl Renderer {
    pub fn new(api: backend::RendererApi) -> Self {
        let (s, r) = mpsc::channel();
        Renderer { api, sender: s, receiver: r }
    }
}

impl Renderer {
    pub fn submit(&mut self, cmd: DrawIndexed) {
        self.sender.send(cmd);
    }

    pub fn process(&self, ctx: &mut AssetHolder) {
        for cmd in self.receiver.try_iter() {
            let va: &backend::VertexArray = ctx.storage().get_ref(&cmd.0).unwrap();
            let shader: &backend::Shader = ctx.storage().get_ref(&cmd.1).unwrap();

            shader.bind();
            self.api.draw_indexed(va);
            shader.unbind();
        }
    }

    pub fn start(&self) {
        self.api.clear_color();
    }
    pub fn end(&mut self) {
        self.api.swap_buffer();
    }

    pub fn get_submitter(&self) -> Sender<DrawIndexed> {
        self.sender.clone()
    }

    pub fn api(&self) -> &backend::RendererApi {
        &self.api
    }
}