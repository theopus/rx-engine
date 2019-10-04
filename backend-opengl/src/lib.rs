extern crate backend_api as api;
extern crate itertools;

pub use crate::core::{
    OpenGLRendererApi as RendererApi,
    OpenGLRendererDevice as RendererDevice,
};
pub use buffer_v2::OpenGlBuffer as Buffer;
pub use image::{
    OpenGlImage as Image,
    OpenGlImageView as ImageView,
};
pub use pipeline::{
    OpenGlDescriptorSet as DescriptorSet,
    OpenGlDescriptorSetLayout as DescriptorSetLayout,
    OpenGlPipeline as Pipeline,
    OpenGlPipelineLayout as PipelineLayout,
};
pub use platform_glfw::{
    GlfwImGuiRenderer as ImGuiRenderer,
    GlfwPlatformManager as PlatformManager,
};

mod image;
mod buffer_v2;
mod core;
mod platform_glfw;
mod shader_mod;
mod memory;

mod imgui_glfw;
mod imgui_glfw_render;
mod pipeline;
mod swapchain;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Backend {}

impl backend_api::Backend for Backend {
    type RendererApi = core::OpenGLRendererApi;
    type RendererDevice = core::OpenGLRendererDevice;
    type PlatformManager = platform_glfw::GlfwPlatformManager;
    type ImGuiRenderer = platform_glfw::GlfwImGuiRenderer;
    type Memory = memory::OpenGlMemory;
    type Buffer = buffer_v2::OpenGlBuffer;
    type Image = image::OpenGlImage;
    type ImageView = image::OpenGlImageView;
    type Pipeline = pipeline::OpenGlPipeline;
    type CommandBuffer = pipeline::OpenGlCommandBuffer;
    type ShaderMod = shader_mod::OpenGlShaderMod;
    type DescriptorSet = pipeline::OpenGlDescriptorSet;
    type DescriptorSetLayout = pipeline::OpenGlDescriptorSetLayout;
    type PipelineLayout = pipeline::OpenGlPipelineLayout;
    type Surface = swapchain::OpenGlSurface;
    type Swapchain = swapchain::OpenGlSwapchain;
    type Framebuffer = swapchain::OpenGlFramebuffer;
}