extern crate backend_interface as interface;
extern crate itertools;

pub use api::{
    OpenGLRendererApi as RendererApi,
    OpenGLRendererDevice as RendererDevice,
};
pub use buffer::{
    OpenGLIndexBuffer as IndexBuffer,
    OpenGLVertexArray as VertexArray,
    OpenGLVertexBuffer as VertexBuffer,
};
pub use buffer_v2::OpenGlBuffer as Buffer;
pub use pipeline::{
    OpenGlPipeline as Pipeline,
    OpenGlDescriptorSetLayout as DescriptorSetLayout,
    OpenGlPipelineLayout as PipelineLayout,
    OpenGlDescriptorSet as DescriptorSet,
};
pub use platform_glfw::{
    GlfwImGuiRenderer as ImGuiRenderer,
    GlfwPlatformManager as PlatformManager,
};
#[cfg(not(feature = "hot_reload"))]
pub use shader::OpenGLShader as Shader;
#[cfg(feature = "hot_reload")]
pub use shader::ReloadableOpenGLShader as Shader;
pub use shader::Shader as ShaderInterface;

mod buffer;
mod buffer_v2;
mod shader;
mod api;
mod platform_glfw;
mod shader_mod;

mod imgui_glfw;
mod imgui_glfw_render;
mod pipeline;

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
    type RendererDevice = api::OpenGLRendererDevice;
    type PlatformManager = platform_glfw::GlfwPlatformManager;
    type ImGuiRenderer = platform_glfw::GlfwImGuiRenderer;

    type Buffer = buffer_v2::OpenGlBuffer;
    type Pipeline = pipeline::OpenGlPipeline;
    type CommandBuffer = pipeline::OpenGlCommandBuffer;
    type ShaderMod = shader_mod::OpenGlShaderMod;
    type DescriptorSet = pipeline::OpenGlDescriptorSet;
    type DescriptorSetLayout = pipeline::OpenGlDescriptorSetLayout;
    type PipelineLayout = pipeline::OpenGlPipelineLayout;
}