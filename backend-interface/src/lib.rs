pub extern crate imgui;

use std::any::Any;
use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;
use std::path::Path;
use std::slice::Iter;
use std::sync::mpsc::Receiver;

use self::shared_types::TypeInfo;

pub mod utils;


pub trait Backend: 'static + Sized + Eq + Clone + Hash + fmt::Debug + Any + Send + Sync {
    type VertexArray: VertexArray<Self>;
    type VertexBuffer: VertexBuffer<Self>;
    type IndexBuffer: IndexBuffer<Self>;
    type Shader: Shader;
    type RendererApi: RendererApi<Self>;
    type RendererDevice: RendererDevice<Self>;
    type PlatformManager: PlatformManager<Self>;
    type ImGuiRenderer: ImGuiRenderer;
    //
    type Buffer: Send + Sync;
    type Pipeline: Send + Sync;
    type CommandBuffer: CommandBuffer<Self> + Send + Sync;
    type ShaderMod: Send + Sync + Debug;
    type DescriptorSet: Send + Sync + Debug;
    type DescriptorSetLayout: Send + Sync + Debug;
    type PipelineLayout: Send + Sync + Debug;
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

pub trait PlatformManager<B: Backend> {
    fn new(config: WindowConfig) -> B::PlatformManager;
    fn create_renderer(&mut self) -> (B::RendererApi, B::RendererDevice);
    fn should_close(&self) -> bool;
    fn poll_events(&self) -> Vec<Event>;
    fn current_time(&self) -> f64;
    fn current_time_ms(&self) -> f64 {
        self.current_time() * 1000f64
    }
    fn imgui_renderer(&mut self, imgui: &mut imgui::Context) -> B::ImGuiRenderer;
}

pub trait VertexArray<B: Backend>: Drop {
    fn id(&self) -> u32;
    fn bind(&self);
    fn add_vertex_buffer(&mut self, vertex_buffer: B::VertexBuffer);
    fn set_index_buffer(&mut self, index_buffer: B::IndexBuffer);
    fn get_index_buffer(&self) -> &B::IndexBuffer;
    fn unbind(&self);
}

pub trait VertexBuffer<B: Backend> {
    fn bind(&self);
    fn set_buffer_layout(&mut self, layout: BufferLayout);
    fn get_buffer_layout(&self) -> &BufferLayout;
    fn buffer_data_f32(&self, data: &[f32]);
    fn buffer_data_u32(&self, data: &[u32]);
    fn unbind(&self);
}

pub trait IndexBuffer<B: Backend> {
    fn bind(&self);
    fn unbind(&self);
    fn length(&self) -> u32;
    fn buffer_data(&self, data: &[u32]);
}

pub struct BufferLayout {
    elements: Vec<shared_types::TypeInfo>
}

pub trait Shader {
    fn bind(&self);
    fn load_mat4(&self, name: &str, mtx: &[f32]);
    fn load_vec3(&self, name: &str, vec: &[f32]);
    fn unbind(&self);
}

#[derive(Debug)]
pub struct BufferDescriptor {
    pub size: u32,
    pub usage: Usage,
}


#[derive(Debug)]
pub struct PipelineDescriptor<B: Backend> {
    pub primitives: Primitive,
    pub shader_set: ShaderSet<B>,
    pub layout: B::PipelineLayout,
    pub vertex_buffers: Vec<VertexBufferDescriptor>,
    pub vertex_attributes: Vec<AttributeDescriptor>,
}

impl<B> PipelineDescriptor<B> where B: Backend {
    pub fn new(primitive: Primitive, shader_set: ShaderSet<B>, layout: B::PipelineLayout) -> PipelineDescriptor<B> {
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

pub struct DescriptorSetLayoutBinding {
    pub binding: u8,
    pub desc: DescriptorType,
}

pub enum DescriptorType {
    UniformBuffer,
    Sampler,
}

pub trait RendererDevice<B: Backend> {
    fn vertex_array(&self) -> B::VertexArray;
    fn vertex_buffer(&self) -> B::VertexBuffer;
    fn index_buffer(&self, indexes: &[u32]) -> B::IndexBuffer;
    fn shader(&self, vertex: &Path, fragment: &Path, mem_layout: &BufferLayout) -> B::Shader;
    ///
    fn create_buffer(&self, desc: BufferDescriptor) -> B::Buffer;
    fn map_buffer(&self, buffer: &B::Buffer) -> *mut u8;
    fn unmap_buffer(&self, buffer: &B::Buffer);
    fn create_pipeline(&self, desc: PipelineDescriptor<B>) -> B::Pipeline;

    //make pooled
    fn create_cmd_buffer(&self) -> B::CommandBuffer;
    //make pooled
    fn allocate_descriptor_set(&self, desc: B::DescriptorSetLayout) -> B::DescriptorSet;
    fn execute(&self, cmd: B::CommandBuffer);

    fn create_shader_mod(&self, desc: ShaderModDescriptor) -> B::ShaderMod;
    fn create_descriptor_set_layout(&self, bindings: &[DescriptorSetLayoutBinding]) -> B::DescriptorSetLayout;
    fn create_pipeline_layout(&self, bindings: B::DescriptorSetLayout) -> B::PipelineLayout;


    fn write_descriptor_set(&self, desc_set_write: DescriptorSetWrite<B>);
}

pub struct DescriptorSetWrite<'a, B: Backend> {
    pub binding: u32,
    pub descriptor: Descriptor<'a, B>,
}

pub enum Descriptor<'a, B: Backend> {
    Buffer(&'a B::Buffer)
}

pub trait CommandBuffer<B: Backend> {
    fn prepare_pipeline(&mut self, pipeline: &B::Pipeline);
    fn bind_vertex_buffer(&mut self, binding: usize, buffer: &B::Buffer);
    fn bind_index_buffer(&mut self, buffer: &B::Buffer);
    fn buffer_data(&mut self, buffer: &B::Buffer, data: &[u8]);
    fn draw_indexed(&mut self, count: u32, offset: u32);
    fn bind_descriptor_set(&self, pipeline_layout: &B::PipelineLayout, desc_set: &B::DescriptorSet);
    fn clear_screen(&mut self, color: (f32, f32, f32, f32));
}

pub trait RendererApi<B: Backend> {
    fn swap_buffer(&mut self);
    fn draw_indexed(&self, vertex_array: &B::VertexArray);
    fn clear_color(&self);
    fn set_clear_color(&self, r: f32, g: f32, b: f32, a: f32);
    fn viewport(&self, w: i32, h: i32);
}

impl BufferLayout {
    pub fn with(element: shared_types::TypeInfo) -> Self {
        let mut layout = BufferLayout { elements: Vec::new() };
        layout.elements.push(element);
        layout
    }

    pub fn and(mut self, element: shared_types::TypeInfo) -> Self {
        self.elements.push(element);
        self
    }

    pub fn elements(&self) -> Iter<TypeInfo> {
        self.elements.iter()
    }

    pub fn stride(&self) -> u64 {
        self.elements.iter().map(|e| { e.2 }).sum()
    }
}

pub mod shared_types {
    use std::mem::size_of;

    pub struct TypeInfo(pub u8, pub usize, pub u64, pub Type);

    pub enum Type {
        Float,
        Float2,
        Float3,
        Float4,
        Mat4,
        Int,
        Int2,
        Int3,
    }

    pub const FLOAT: TypeInfo = TypeInfo(1, size_of::<f32>(), 1 * size_of::<f32>() as u64, Type::Float);
    pub const FLOAT_2: TypeInfo = TypeInfo(2, size_of::<f32>(), 2 * size_of::<f32>() as u64, Type::Float2);
    pub const FLOAT_3: TypeInfo = TypeInfo(3, size_of::<f32>(), 3 * size_of::<f32>() as u64, Type::Float3);
    pub const FLOAT_4: TypeInfo = TypeInfo(4, size_of::<f32>(), 4 * size_of::<f32>() as u64, Type::Float4);
    pub const MAT_4: TypeInfo = TypeInfo(4 * 4, size_of::<f32>(), 4 * 4 * size_of::<f32>() as u64, Type::Mat4);
    pub const INT: TypeInfo = TypeInfo(1, size_of::<u32>(), 1 * size_of::<f32>() as u64, Type::Int);
    pub const INT_2: TypeInfo = TypeInfo(2, size_of::<u32>(), 2 * size_of::<f32>() as u64, Type::Int2);
    pub const INT_3: TypeInfo = TypeInfo(3, size_of::<u32>(), 3 * size_of::<f32>() as u64, Type::Int3);
}