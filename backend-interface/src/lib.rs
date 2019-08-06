use std::path::Path;
use std::slice::Iter;
use std::sync::mpsc::Receiver;

use self::shared_types::TypeInfo;
use std::hash::Hash;
use std::fmt;
use std::any::Any;

pub trait Backend: 'static + Sized + Eq + Clone + Hash + fmt::Debug + Any + Send + Sync {
    type VertexArray;
    type VertexBuffer;
    type IndexBuffer;
    type Shader;
    type RendererApi;
    type RendererConstructor;
    type PlatformManager;
}

pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
}

pub trait PlatformManager<B: Backend> {
    fn new(config: WindowConfig) -> B::PlatformManager;
    fn create_renderer(&self) -> (B::RendererApi, B::RendererConstructor);
    fn should_close(&self) -> bool;
    fn process_events(&self);
    fn current_time(&self) -> f64;
    fn current_time_ms(&self) -> f64 {
        self.current_time() * 1000f64
    }
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
    fn unbind(&self);
}

pub trait RendererConstructor<B: Backend> {
    fn vertex_array(&self) -> B::VertexArray;
    fn vertex_buffer(&self) -> B::VertexBuffer;
    fn index_buffer(&self, indexes: &[u32]) -> B::IndexBuffer;
    fn shader(&self, vertex_src: &str, fragment_src: &str, mem_layout: &BufferLayout) -> B::Shader;
    fn reloadable_shader(&self, vertex: &Path, fragment: &Path, mem_layout: &BufferLayout) -> B::Shader;
}

pub trait RendererApi<B: Backend> {
    fn swap_buffer(&mut self);
    fn draw_indexed(&self, vertex_array: &B::VertexArray);
    fn clear_color(&self);
    fn set_clear_color(&self, r: f32, g: f32, b: f32, a: f32);
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