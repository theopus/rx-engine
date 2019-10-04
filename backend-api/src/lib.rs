pub extern crate imgui;

use core::borrow::Borrow;
use std::any::Any;
use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;
use std::path::Path;
use std::slice::Iter;
use std::sync::mpsc::Receiver;

pub mod utils;


pub trait Backend: 'static + Sized + Eq + Clone + Hash + fmt::Debug + Any + Send + Sync {
    type RendererApi: RendererApi<Self>;
    type RendererDevice: RendererDevice<Self>;
    type PlatformManager: PlatformManager<Self>;
    type ImGuiRenderer: ImGuiRenderer;
    //
    type Memory: Send + Sync;
    type Buffer: Send + Sync;
    type Image: Send + Sync;
    type ImageView: Send + Sync;
    type Pipeline: Send + Sync;
    type CommandBuffer: CommandBuffer<Self> + Send + Sync;
    type ShaderMod: Send + Sync + Debug;
    type DescriptorSet: Send + Sync + Debug;
    type DescriptorSetLayout: Send + Sync + Debug;
    type PipelineLayout: Send + Sync + Debug;
    //
    type Surface;
    type Swapchain: Swapchain<Self>;
    type Framebuffer: Send + Sync + Debug;
}

pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
}

pub type Code = u32;

#[derive(Clone, Debug)]
pub enum Action {
    Press,
    Release,
    Repeat,
}

#[derive(Clone, Debug)]
pub enum Event {
    Resize(i32, i32),
    Key(Code, Action),
    Unhandled,
}

pub trait ImGuiRenderer {
    fn new_frame<'im>(&mut self, imgui: &'im mut imgui::Context) -> imgui::Ui<'im>;
    fn render(&self, ui: imgui::Ui);
    fn handle_events(&mut self, imgui: &mut imgui::Context);
}


#[derive(Debug)]
pub struct BufferDescriptor {
    pub size: u32,
    pub usage: Usage,
}


#[derive(Debug)]
pub struct PipelineDescriptor<'a, B: Backend> {
    pub primitives: Primitive,
    pub shader_set: ShaderSet<B>,
    pub layout: &'a B::PipelineLayout,
    pub vertex_buffers: Vec<VertexBufferDescriptor>,
    pub vertex_attributes: Vec<AttributeDescriptor>,
}

impl<'a, B> PipelineDescriptor<'a, B> where B: Backend {
    pub fn new(primitive: Primitive, shader_set: ShaderSet<B>, layout: &B::PipelineLayout) -> PipelineDescriptor<B> {
        PipelineDescriptor {
            primitives: primitive,
            shader_set: shader_set,
            layout: layout,
            vertex_buffers: Vec::new(),
            vertex_attributes: Vec::new(),
        }
    }

    pub fn push_vb(&mut self, desc: VertexBufferDescriptor) {
        self.vertex_buffers.push(desc);
    }
    pub fn push_attr(&mut self, desc: AttributeDescriptor) {
        self.vertex_attributes.push(desc);
    }
}

#[derive(Debug, Clone)]
pub struct VertexBufferDescriptor {
    pub binding: u8,
    pub stride: usize,
}

#[derive(Debug, Clone)]
pub struct AttributeDescriptor {
    pub binding: u16,
    pub location: u32,
    pub data: VertexData,
}


#[derive(Debug)]
pub enum Usage {
    Vertex,
    Index,
    Uniform,
}

#[derive(Debug, Clone)]
pub struct VertexData {
    pub offset: usize,
    pub data_type: DataType,
}

#[derive(Debug)]
pub enum Primitive {
    Triangles,
    TrianglesFan,
    TrianglesStrip,
    Quads,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Vec3f32,
    Vec2f32,
    Mat4f32,
}

pub struct ShaderModDescriptor {
    pub stype: ShaderType,
    pub source: String,
}

pub enum ShaderType {
    Vertex,
    Fragment,
}


#[derive(Debug)]
pub struct ShaderSet<B: Backend> {
    pub vertex: B::ShaderMod,
    pub fragment: B::ShaderMod,
}

#[derive(Debug, Clone)]
pub struct DescriptorSetLayoutBinding {
    pub binding: u32,
    pub desc: DescriptorType,
}

#[derive(Debug, Clone)]
pub enum DescriptorType {
    UniformBuffer,
    Sampler,
}

#[derive(Debug, Clone)]
pub struct PipelineLayoutHint {
    pub location: u32,
    pub hint: LayoutHint,
}

#[derive(Debug, Clone)]
pub enum LayoutHint {
    Name(&'static str)
}

pub struct DescriptorSetWrite<'a, B: Backend> {
    pub set: &'a B::DescriptorSet,
    pub binding: u32,
    pub descriptor: Descriptor<'a, B>,
}

pub enum Descriptor<'a, B: Backend> {
    Buffer(&'a B::Buffer)
}

pub trait PlatformManager<B: Backend> {
    fn new(config: WindowConfig) -> B::PlatformManager;
    fn create_renderer(&mut self) -> (B::RendererApi, B::RendererDevice);
    fn should_close(&self) -> bool;
    fn poll_events(&self) -> Vec<Event>;
    fn current_time(&self) -> f64;
    fn current_time_ms(&self) -> f64 {
        self.current_time() * 1000f64
    }
    fn create_surface(&self) -> B::Surface;
    fn imgui_renderer(&mut self, imgui: &mut imgui::Context) -> B::ImGuiRenderer;
}

pub trait RendererDevice<B: Backend> {
    //mem
    fn allocate_memory(&self, size: u32) -> B::Memory;
    fn map_memory(&self, memory: &B::Memory) -> *mut u8;
    fn flush_memory(&self, memory: &B::Memory);
    fn unmap_memory(&self, memory: &B::Memory);
    fn bind_buffer_memory(&self, memory: &mut B::Memory, buffer: &B::Buffer);
    //buffer
    fn create_buffer(&self, desc: BufferDescriptor) -> B::Buffer;
    fn create_pipeline(&self, desc: PipelineDescriptor<B>) -> B::Pipeline;
    //make pooled
    fn create_cmd_buffer(&self) -> B::CommandBuffer;
    //make pooled
    fn allocate_descriptor_set(&self, desc: &B::DescriptorSetLayout) -> B::DescriptorSet;
    fn execute(&self, cmd: B::CommandBuffer);

    fn create_shader_mod(&self, desc: ShaderModDescriptor) -> B::ShaderMod;
    fn create_descriptor_set_layout(&self, bindings: &[DescriptorSetLayoutBinding]) -> B::DescriptorSetLayout;

    fn create_pipeline_layout<I>(
        &self,
        desc_layout: &B::DescriptorSetLayout,
        hints: I,
    ) -> B::PipelineLayout
        where
            I: IntoIterator<Item=PipelineLayoutHint>;


    fn write_descriptor_set(&self, desc_set_write: DescriptorSetWrite<B>);

    fn create_swapchain(
        &self,
        surface: &B::Surface,
    ) -> (
        B::Swapchain,
        Vec<B::Image>
    );

    fn create_image(
        kind: image::Kind
    ) -> B::Image;
}

pub mod image {
    pub type Size = u32;
    pub type Level = u16;

    pub enum Kind {
        D1(Size, Level),
        D2(Size, Size, Level),
        D3(Size, Size, Size),
    }
}


pub trait Swapchain<B: Backend> {
    fn present(&mut self, frame_index: u32);
}

pub trait CommandBuffer<B: Backend> {
    fn prepare_pipeline(&mut self, pipeline: &B::Pipeline);
    fn bind_vertex_buffer(&mut self, binding: u32, buffer: &B::Buffer);
    fn bind_index_buffer(&mut self, buffer: &B::Buffer);
    fn buffer_data(&mut self, buffer: &B::Buffer, data: &[u8]);
    fn draw_indexed(&mut self, count: u32, offset: u32, number: u32);
    fn bind_descriptor_set(&mut self, pipeline_layout: &B::PipelineLayout, desc_set: &B::DescriptorSet);
    fn clear_screen(&mut self, color: (f32, f32, f32, f32));
}

pub trait RendererApi<B: Backend> {
    fn swap_buffer(&mut self);
    fn clear_color(&self);
    fn set_clear_color(&self, r: f32, g: f32, b: f32, a: f32);
    fn viewport(&self, w: i32, h: i32);
}