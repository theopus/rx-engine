use std::path::Path;
use std::slice::Iter;
use std::sync::mpsc::Receiver;

use crate::render::shared_types::TypeInfo;

pub trait VertexArray: Drop {
    fn id(&self) -> u32;
    fn bind(&self);
    fn add_vertex_buffer(&mut self, vertex_buffer: Box<VertexBuffer>);
    fn set_index_buffer(&mut self, index_buffer: Box<IndexBuffer>);
    fn get_index_buffer(&self) -> &IndexBuffer;
    fn unbind(&self);
}

pub trait VertexBuffer {
    fn bind(&self);
    fn set_buffer_layout(&mut self, layout: BufferLayout);
    fn get_buffer_layout(&self) -> &BufferLayout;
    fn buffer_data_f32(&self, data: &[f32]);
    fn buffer_data_u32(&self, data: &[u32]);
    fn unbind(&self);
}

pub trait IndexBuffer {
    fn bind(&self);
    fn unbind(&self);
    fn length(&self) -> u32;
    fn buffer_data(&self, data: &[u32]);
}

pub struct BufferLayout {
    elements: Vec<shared_types::TypeInfo>
}

///Material:
///     * shader
///     * possible properties
///MaterialInstance:
///     * Material
///     * Properties Values
///     - use:
///         upload_values()
///
///
/// ex: flat color
///
/// flat_color_material = Material:
///                     * flat_color_shader
///                     * vec4 color
///
/// red_color_material = flat_color_material.instance(color = vec3(1,0,0,0))
///
/// ec2: textures
///
/// textured_material = Material:
///                    * textured_shader
///                    * Texture texture
///
/// cool_textured_material = textured_material.instance(texture = Texture(cool.png))
///
/// ***in the renderer***
///
/// mesh major batching
///
/// mesh_1 bind:
///     material_1 bind:
///         instance_1 bind:
///             draw one
///         instance_2 bind:
///             draw all
///     material_n bind:
///         etc...
/// mesh_2 bind:
///     etc...
///
/// ***or***
///
/// material major batching
///
/// material_1 bind:
///     instance_1 bind:
///         mesh_1 bind
///         draw call one
///     instance_2 bind:
///         mesh_1 bind
///         instancing
///         draw call all
/// material_n bind:
///     etc...
///
/// ***or***
///
/// --bullshit--
/// batching with materials
///
/// material_1 bind:
///    all_instances bind:
///         mesh_1 bind
///             instancing
///             draw call all
///         mesh_2_bind
///             instancing
///             draw call all
/// material_n bind:
///     etc...
/// --bullshit
///
/// per mesh have buffers mesh[nsize]
/// for instancing

pub trait Shader {
    fn bind(&self);
    fn load_mat4(&self, mtx: &na::Matrix4<f32>);
    fn unbind(&self);
}

pub trait Reloadable {
    fn reload_if_changed(&self);
}

pub trait ReloadableShader: Shader + Reloadable {}

pub trait RendererConstructor {
    fn vertex_array(&self) -> Box<VertexArray>;
    fn vertex_buffer(&self) -> Box<VertexBuffer>;
    fn index_buffer(&self, indexes: &[u32]) -> Box<IndexBuffer>;
    fn shader(&self, vertex_src: &str, fragment_src: &str, mem_layout: &BufferLayout) -> Box<Shader>;
    fn reloadable_shader(&self, vertex: &Path, fragment: &Path, mem_layout: &BufferLayout) -> Box<Shader>;
}

pub trait RendererApi {
    fn swap_buffer(&mut self);
    fn draw_indexed(&self, vertex_array: &VertexArray);
    fn clear_color(&self);
    fn set_clear_color(&self, r: f32, g: f32, b: f32, a: f32);
}

pub struct Renderer {
    api: Box<RendererApi>
}

impl Renderer {
    pub fn new(api: Box<RendererApi>) -> Self {
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
        self.api.as_ref()
    }
}

#[derive(Debug)]
pub enum RendererType {
    None,
    OpenGL,
    Vulkan,
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