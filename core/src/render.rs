use crate::backend::VertexArray;
use crate::backend::RendererApi;
use crate::backend::Shader;
use crate::backend::interface::Shader as ShaderInterface;
use crate::backend::interface::RendererApi as RendererApiInterface;

pub struct Renderer {
    api: RendererApi
}

impl Renderer {
    pub fn new(api: RendererApi) -> Self {
        Renderer { api }
    }
}

impl Renderer {
    pub fn submit(&mut self, vertex_array: &VertexArray, shader: &Shader) {
        shader.bind();
        self.api.draw_indexed(vertex_array);
        shader.unbind();
    }

    pub fn start(&self){
        self.api.clear_color();
    }
    pub fn end(&mut self){
        self.api.swap_buffer();
    }

    pub fn api(&self) -> &RendererApi {
        &self.api
    }
}