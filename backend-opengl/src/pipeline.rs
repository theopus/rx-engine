use core::borrow::Borrow;
use std::collections::HashMap;
use std::ffi::c_void;
use std::fmt;
use std::mem::size_of;
use std::os::raw::c_char;

use itertools::Itertools;

use gl::Gl;
use api::AttributeDescriptor;
use api::PipelineDescriptor;
use api::Primitive;
use api::VertexBufferDescriptor;

use crate::Backend;
use crate::buffer_v2::OpenGlBuffer;
use crate::pipeline::OpenGlCommand::{BindDescriptorSet, BindIndexBuffer, BindVertexBuffer, ClearScreen, DrawIndexed, DrawIndexedInstanced, PreparePipeline};

type GlPrimitive = gl::types::GLenum;
type VaoId = gl::types::GLuint;
type BufferId = gl::types::GLuint;
type ProgramId = gl::types::GLuint;
type Binding = u32;
type UboIndex = u32;

#[derive(Debug, Clone)]
pub struct OpenGlRenderPass {

}


#[derive(Debug, Clone)]
pub struct OpenGlPipeline {
    vao_id: VaoId,
    program_id: ProgramId,
    primitive: GlPrimitive,
    layout: Vec<(VertexBufferDescriptor, Vec<AttributeDescriptor>)>,

    binding_cache: HashMap<Binding, BufferId>,
    index_buffer_cache: BufferId,

    ubo_index: HashMap<Binding, UboIndex>,
}

impl OpenGlPipeline {
    pub unsafe fn new(gl: &Gl, desc: PipelineDescriptor<Backend>) -> Result<Self, String> {
        let program_id = create_program(&gl, &desc)?;
        Ok(OpenGlPipeline {
            vao_id: gen_vao(gl),
            program_id: program_id,
            primitive: match desc.primitives {
                Primitive::Triangles => gl::TRIANGLES,
                Primitive::TrianglesFan => gl::TRIANGLE_FAN,
                Primitive::TrianglesStrip => gl::TRIANGLE_STRIP,
                //TODO:[POSSIBLE_WTF]
                Primitive::Quads => gl::TRIANGLES,
            },
            layout: {
                let mut layout = Vec::with_capacity(desc.vertex_buffers.len());
                'main: for (bind, attr) in &desc.vertex_attributes.into_iter().group_by(|d| d.binding) {
                    for buff in desc.vertex_buffers.clone() {
                        if buff.binding == bind as u8 {
                            layout.insert(bind as usize, (buff.clone(), attr
                                .map(|c| c.clone()).collect::<Vec<AttributeDescriptor>>()));
                            continue 'main;
                        }
                    }
                    //TODO:ok
                    panic!("Not found binding {}", bind);
                }
                println!("{:?}", layout);
                //TODO:ok
                layout
            },
            binding_cache: HashMap::new(),
            index_buffer_cache: 0,
            ubo_index: ubo_bindings(&gl, program_id, &desc.layout),
        })
    }

    pub unsafe fn prepare(&self, gl: &Gl) {
        gl.BindVertexArray(self.vao_id);
        gl.UseProgram(self.program_id);
        gl.Enable(gl::CULL_FACE);
        gl.CullFace(gl::BACK);
        gl.Enable(gl::DEPTH_TEST)
    }

    pub unsafe fn bind_index(&mut self, buffer: &OpenGlBuffer, gl: &Gl) {
        if self.index_buffer_cache != buffer.id {
            buffer.bind(gl);
            self.index_buffer_cache = buffer.id;
        }
    }

    pub unsafe fn bind_descriptors(&mut self, gl: &Gl, bindings: &Vec<u32>) {
        for bind in bindings.into_iter() {
            gl.UniformBlockBinding(self.program_id,
                                   *self.ubo_index.get(bind).unwrap(),
                                   *bind)
        }
    }

    pub unsafe fn bind_buffer(&mut self, binding: u32, buffer: &OpenGlBuffer, gl: &Gl) {
        let (buff, attrs) = self.layout
            .get(binding as usize)
            .unwrap();

        if !self.binding_cache.contains_key(&binding) {
            buffer.bind(gl);
            for attr in attrs {
                match attr.data.data_type {
                    api::DataType::Vec3f32 => {
                        OpenGlPipeline::vertex_pointer(gl, buff, attr, attr.data.offset, attr.location);
                    }
                    api::DataType::Vec2f32 => {
                        OpenGlPipeline::vertex_pointer(gl, buff, attr, attr.data.offset, attr.location);
                    }
                    api::DataType::Mat4f32 => {
                        OpenGlPipeline::vertex_pointer(gl, buff, attr, attr.data.offset + size_of::<[f32; 4]>() * 0, attr.location + 0);
                        OpenGlPipeline::vertex_pointer(gl, buff, attr, attr.data.offset + size_of::<[f32; 4]>() * 1, attr.location + 1);
                        OpenGlPipeline::vertex_pointer(gl, buff, attr, attr.data.offset + size_of::<[f32; 4]>() * 2, attr.location + 2);
                        OpenGlPipeline::vertex_pointer(gl, buff, attr, attr.data.offset + size_of::<[f32; 4]>() * 3, attr.location + 3);
                        gl.VertexAttribDivisor(attr.location + 0, 1);
                        gl.VertexAttribDivisor(attr.location + 1, 1);
                        gl.VertexAttribDivisor(attr.location + 2, 1);
                        gl.VertexAttribDivisor(attr.location + 3, 1);
                    }
                };
            };

            self.binding_cache.insert(binding, buffer.id);
        } else {
            println!("{:?}", self.binding_cache);
        }
    }

    unsafe fn vertex_pointer(
        gl: &Gl,
        buff: &VertexBufferDescriptor,
        attr: &AttributeDescriptor,
        offset: usize,
        location: u32,
    ) {
        gl.VertexAttribPointer(location.into(),
                               match attr.data.data_type {
                                   api::DataType::Vec3f32 => 3,
                                   api::DataType::Vec2f32 => 2,
                                   api::DataType::Mat4f32 => 4,
                               },
                               match attr.data.data_type {
                                   api::DataType::Vec3f32 => gl::FLOAT,
                                   api::DataType::Vec2f32 => gl::FLOAT,
                                   api::DataType::Mat4f32 => gl::FLOAT,
                               },
                               gl::FALSE,
                               buff.stride as i32,
                               offset as *const c_void);
        gl.EnableVertexAttribArray(location.into());
    }
}

unsafe fn ubo_bindings(gl: &Gl, program: ProgramId, pipeline_layout: &OpenGlPipelineLayout) -> HashMap<Binding, UboIndex> {
    let mut indexes = HashMap::new();
    for (binding, (desc, hint)) in &pipeline_layout.layout {
        let index = if hint.is_some() {
            match hint.as_ref().unwrap().hint {
                api::LayoutHint::Name(string) => {
                    let string = string.to_owned() + "\0";
                    gl.GetUniformBlockIndex(program, string.as_str().as_ptr() as *const c_char)
                }
                _ => *binding
            }
        } else {
            *binding
        };
        if index == gl::INVALID_INDEX {
            panic!("Invalid index binding:{:?}, resolved:{:?}", binding, index)
        }
        indexes.insert(*binding, index);
    }
    indexes
}

unsafe fn create_program(gl: &Gl, desc: &PipelineDescriptor<Backend>) -> Result<ProgramId, String> {
    let program = gl.CreateProgram();
    gl.AttachShader(program, desc.shader_set.vertex.id);
    gl.AttachShader(program, desc.shader_set.fragment.id);
    gl.LinkProgram(program);
    gl.DetachShader(program, desc.shader_set.vertex.id);
    gl.DetachShader(program, desc.shader_set.fragment.id);
    validate_program(gl, program)?;
    validate_attrs(gl, program, desc)
}

unsafe fn validate_attrs(gl: &Gl, id: ProgramId, desc: &PipelineDescriptor<Backend>)
                         -> Result<ProgramId, String> {
    let (mut len, mut name) = {
        let mut len: gl::types::GLint = 0;
        gl.GetProgramiv(id, gl::ACTIVE_ATTRIBUTE_MAX_LENGTH, &mut len);
        let name = crate::shader_mod::create_whitespace_cstring_with_len(len as usize);
        (len, name)
    };

    let attr_len = {
        let mut len: gl::types::GLint = 0;
        gl.GetProgramiv(id, gl::ACTIVE_ATTRIBUTES, &mut len);
        len
    };

    let gl_attrs: Vec<(i32, u32, i32)> = (0..attr_len).into_iter().map(|attr_index| {
        let mut written: i32 = 0;
        let mut size: i32 = 0;
        let mut dtype: u32 = 0;
        gl.GetActiveAttrib(id,
                           attr_index as u32,
                           len,
                           &mut written,
                           &mut size,
                           &mut dtype,
                           name.as_ptr() as *mut gl::types::GLchar);

        let n = &name.to_str().unwrap()[..written as usize];
        let cstr = n.to_owned() + "\0";
        let location = gl.GetAttribLocation(id, cstr.as_ptr() as *mut gl::types::GLchar);
        (location, dtype, size)
    }).collect::<Vec<(i32, u32, i32)>>();

    for attr in &desc.vertex_attributes {
        let gl_attr = get_attr(attr, &gl_attrs)?;
        assert_eq!(gl_attr.0, attr.location as i32);
        assert_eq!(gl_attr.1, match attr.data.data_type {
            api::DataType::Vec3f32 => gl::FLOAT_VEC3,
            api::DataType::Vec2f32 => gl::FLOAT_VEC2,
            api::DataType::Mat4f32 => gl::FLOAT_MAT4,
        });
        assert_eq!(gl_attr.2, 1);
    }
    Ok(id)
}

fn get_attr(attr: &AttributeDescriptor, gl_attrs: &Vec<(i32, u32, i32)>) -> Result<(i32, u32, i32), String> {
    for gl_attr in gl_attrs {
        if gl_attr.0 == attr.location as i32 {
            return Ok(*gl_attr);
        }
    }
    Err(format!("Not found attr {:?}", attr))
}

fn validate_program(gl: &Gl, id: ProgramId) -> Result<ProgramId, String> {
    let mut success: gl::types::GLint = 1;
    unsafe {
        gl.GetProgramiv(id, gl::LINK_STATUS, &mut success);
    }
    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl.GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = crate::shader_mod::create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl.GetProgramInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        }

        return Err(error.to_string_lossy().into_owned());
    }
    Ok(id)
}

unsafe fn gen_vao(gl: &Gl) -> VaoId {
    let mut id: gl::types::GLuint = 0;
    gl.GenVertexArrays(1, &mut id);
    id
}


#[derive(Debug)]
pub struct OpenGlDescriptorSetLayout {
    layout: HashMap<u32, api::DescriptorSetLayoutBinding>
}

impl OpenGlDescriptorSetLayout {
    pub fn new(bindings: &[api::DescriptorSetLayoutBinding]) -> Self {
        let mut map = HashMap::with_capacity(bindings.len());
        for b in bindings {
            map.insert(b.binding, (b).clone());
        }
        OpenGlDescriptorSetLayout { layout: map }
    }
}

#[derive(Debug)]
pub struct OpenGlPipelineLayout {
    layout: HashMap<u32, (api::DescriptorSetLayoutBinding, Option<api::PipelineLayoutHint>)>
}

impl OpenGlPipelineLayout {
    pub fn new<I>(desc_layout: &<Backend as api::Backend>::DescriptorSetLayout, hints: I) -> Self
        where
            I: IntoIterator<Item=api::PipelineLayoutHint>, {
        let mut map: HashMap<u32, (api::DescriptorSetLayoutBinding, Option<api::PipelineLayoutHint>)>
            = HashMap::with_capacity(desc_layout.layout.len());
        for e in &desc_layout.layout {
            map.insert(e.0.clone(), (e.1.clone(), None));
        }


        for h in hints {
            let (desc, hint) = map.get(&h.location).unwrap();
            map.insert(h.location, (desc.clone(), Some(h)));
        }
        OpenGlPipelineLayout { layout: map }
    }
}

#[derive(Debug)]
pub struct OpenGlDescriptorSet {}

const DESCRIPTORS_LIMIT: usize = 10;

#[derive(Debug)]
enum OpenGlCommand {
    PreparePipeline(OpenGlPipeline),
    BindVertexBuffer(u32, OpenGlBuffer),
    BindIndexBuffer(OpenGlBuffer),
    BindDescriptorSet(Vec<u32>),
    DrawIndexed(u32, u32),
    DrawIndexedInstanced(u32, u32, u32),
    ClearScreen((f32, f32, f32, f32)),
}

#[derive(Debug)]
pub struct OpenGlCommandBuffer {
    cmds: Vec<OpenGlCommand>
}

impl OpenGlCommandBuffer {
    pub(crate) fn new() -> Self {
        OpenGlCommandBuffer { cmds: Vec::new() }
    }

    pub(crate) unsafe fn execute(&mut self, gl: &Gl) {
        //TODO: Make not like an idiot
        let mut pipeline: Option<&mut OpenGlPipeline> = None;
        for cmd in &mut self.cmds {
            match cmd {
                PreparePipeline(p) => {
                    p.prepare(gl);
                    pipeline = Some(p);
                }
                BindVertexBuffer(index, buffer) => {
                    pipeline.as_mut().unwrap()
                        .bind_buffer(*index, buffer, gl)
                }
                BindIndexBuffer(buffer) => {
                    pipeline.as_mut().unwrap()
                        .bind_index(buffer, gl);
                }
                DrawIndexed(count, offset) => {
                    gl.DrawElements(
                        pipeline.as_ref().unwrap().primitive,
                        *count as i32,
                        gl::UNSIGNED_INT,
                        *offset as *const c_void,
                    );
                }
                ClearScreen((r, g, b, a)) => {
                    gl.ClearColor(*r, *g, *b, *a);
                    gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                }
                BindDescriptorSet(bindings) => {
                    pipeline.as_mut()
                        .unwrap()
                        .bind_descriptors(gl, bindings);
                }
                OpenGlCommand::DrawIndexedInstanced(count, offset, number) => {
                    gl.DrawElementsInstanced(
                        pipeline.as_ref().unwrap().primitive,
                        *count as i32,
                        gl::UNSIGNED_INT,
                        *offset as *const c_void,
                        *number as i32,
                    )
                }
            }
        }
    }
}

impl api::CommandBuffer<Backend> for OpenGlCommandBuffer {
    fn prepare_pipeline(&mut self, pipeline: &<Backend as api::Backend>::Pipeline) {
        self.cmds.push(PreparePipeline(pipeline.clone()));
    }

    fn bind_vertex_buffer(&mut self, binding: u32, buffer: &<Backend as api::Backend>::Buffer) {
        self.cmds.push(BindVertexBuffer(binding, buffer.clone()))
    }

    fn bind_index_buffer(&mut self, buffer: &<Backend as api::Backend>::Buffer) {
        self.cmds.push(BindIndexBuffer(buffer.clone()))
    }

    fn buffer_data(&mut self, buffer: &<Backend as api::Backend>::Buffer, data: &[u8]) {
        unimplemented!()
    }

    fn draw_indexed(&mut self, count: u32, offset: u32, number: u32) {
        if number > 1 {
            self.cmds.push(DrawIndexedInstanced(count, offset, number));
        } else {
            self.cmds.push(DrawIndexed(count, offset));
        }
    }

    fn bind_descriptor_set(&mut self, pipeline_layout: &<Backend as api::Backend>::PipelineLayout, desc_set: &<Backend as api::Backend>::DescriptorSet) {
        let bindings: Vec<u32> = (&pipeline_layout.layout)
            .into_iter()
            .map(|(binding, _)| *binding)
            .collect();
        self.cmds.push(BindDescriptorSet(bindings))
    }

    fn clear_screen(&mut self, color: (f32, f32, f32, f32)) {
        self.cmds.push(ClearScreen(color))
    }
}


fn to_gl_str(src: &str) -> *const c_char {
    let string = src.to_owned() + "\0";
    string.as_str().as_ptr() as *const c_char
}