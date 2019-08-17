extern crate backend_interface as interface;

mod buffer;
mod shader;
mod api;
#[cfg(feature = "glfw_backend")]
mod platform_glfw;
#[cfg(feature = "glium")]
mod platform_glium;

pub use buffer::{
    OpenGLVertexArray as VertexArray,
    OpenGLVertexBuffer as VertexBuffer,
    OpenGLIndexBuffer as IndexBuffer,
};

#[cfg(not(feature = "hot_reload"))]
pub use shader::{
    OpenGLShader as Shader,
};

#[cfg(feature = "hot_reload")]
pub use shader::{
    ReloadableOpenGLShader as Shader,
};

pub use shader::{
    Shader as ShaderInterface
};
pub use api::{
    OpenGLRendererApi as RendererApi,
    OpenGLRendererConstructor as RendererConstructor
};

#[cfg(feature = "glfw_backend")]
pub use platform_glfw::{
    GlfwPlatformManager as PlatformManager,
    GlfwImGuiRenderer as ImGuiRenderer
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Backend {}
impl backend_interface::Backend for Backend {
    type VertexArray = buffer::OpenGLVertexArray;
    type VertexBuffer = buffer::OpenGLVertexBuffer;
    type IndexBuffer = buffer::OpenGLIndexBuffer;

    #[cfg(not(feature = "hot_reload"))]
    type Shader = shader::OpenGLShader;

    #[cfg(feature = "hot_reload")]
    type Shader = shader::ReloadableOpenGLShader;

    type RendererApi = api::OpenGLRendererApi;
    type RendererConstructor = api::OpenGLRendererConstructor;

    #[cfg(feature = "glfw_backend")]
    type PlatformManager = platform_glfw::GlfwPlatformManager;
    #[cfg(feature = "glfw_backend")]
    type ImGuiRenderer = platform_glfw::GlfwImGuiRenderer;
}