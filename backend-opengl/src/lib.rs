extern crate backend_interface as interface;

mod buffer;
mod shader;
mod api;
mod platform;

pub use buffer::{
    OpenGLVertexArray as VertexArray,
    OpenGLVertexBuffer as VertexBuffer,
    OpenGLIndexBuffer as IndexBuffer,
};
pub use shader::{
    OpenGLShader as Shader,
    Shader as ShaderInterface
};
pub use api::{
    OpenGLRendererApi as RendererApi,
    OpenGLRendererConstructor as RendererConstructor
};
pub use platform::{
    GlfwPlatformManager as PlatformManager
};

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