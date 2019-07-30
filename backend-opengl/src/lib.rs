pub extern crate backend_interface;

mod buffer;
mod shader;
mod api;
mod platform;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Backend {}
impl backend_interface::Backend for Backend {
    type VertexArray = buffer::OpenGLVertexArray;
    type VertexBuffer = buffer::OpenGLVertexBuffer;
    type IndexBuffer = buffer::OpenGLIndexBuffer;
    type Shader = shader::OpenGLShader;
    type RendererApi = api::OpenGLRendererApi;
    type RendererConstructor = api::OpenGLRendererConstructor;
    type PlatformManager = platform::GlfwPlatformManager;
}