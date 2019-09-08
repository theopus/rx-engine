use std::collections::HashMap;
use std::ffi::c_void;

use itertools::Itertools;

use gl::Gl;
use interface::AttributeDescriptor;
use interface::PipelineDescriptor;
use interface::Primitive;
use interface::VertexBufferDescriptor;

use crate::Backend;
use crate::buffer_v2::OpenGlBuffer;
use crate::pipeline::OpenGlCommand::{BindIndexBuffer, BindVertexBuffer, DrawIndexed, PreparePipeline};

type GlPrimitive = gl::types::GLenum;
type VaoId = gl::types::GLuint;
type Binding = u8;

#[derive(Debug, Clone)]
pub struct OpenGlPipeline {
    vao_id: VaoId,
    primitive: GlPrimitive,
    layout: Vec<(VertexBufferDescriptor, Vec<AttributeDescriptor>)>,
    binding_cache: HashMap<usize, u32>,
    index_buffer_cache: u32,
}

impl OpenGlPipeline {
    pub fn new(gl: &Gl, desc: PipelineDescriptor<Backend>) -> Self {
        let mut layout = Vec::with_capacity(desc.vertex_buffers.len());
        'main: for (bind, attr) in &desc.vertex_attributes.into_iter().group_by(|d| d.binding) {
            for buff in desc.vertex_buffers.clone() {
                if buff.binding == bind as u8 {
                    layout.insert(bind as usize, (buff.clone(), attr
                        .map(|c| c.clone()).collect::<Vec<AttributeDescriptor>>()));
                    continue 'main;
                }
            }
            panic!("Not found binding {}", bind);
        }
        println!("{:?}", layout);

        OpenGlPipeline {
            vao_id: unsafe { gen_vao(gl) },
            primitive: match desc.primitives {
                Primitive::Triangles => gl::TRIANGLES,
                Primitive::TrianglesFan => gl::TRIANGLE_FAN,
                Primitive::TrianglesStrip => gl::TRIANGLE_STRIP,
                //TODO:[POSSIBLE_WTF]
                Primitive::Quads => gl::TRIANGLES,
            },
            layout,
            binding_cache: HashMap::new(),
            index_buffer_cache: 0,
        }
    }
    pub unsafe fn prepare(&self, gl: &Gl) {
        gl.BindVertexArray(self.vao_id);
    }

    pub unsafe fn bind_index(&mut self, buffer: &OpenGlBuffer, gl: &Gl) {
        if self.index_buffer_cache != buffer.id {
            buffer.bind(gl);
            self.index_buffer_cache = buffer.id;
        }
    }

    pub unsafe fn bind_buffer(&mut self, binding: usize, buffer: &OpenGlBuffer, gl: &Gl) {
        let (buff, attrs) = self.layout
            .get(binding)
            .unwrap();

        if !self.binding_cache.contains_key(&binding) {
            buffer.bind(gl);
            for attr in attrs {
                //TODO: Handle matrix case
                gl.VertexAttribPointer(attr.location.into(),
                                       match attr.data.data_type {
                                           interface::DataType::Vec3f32 => 3,
                                           interface::DataType::Mat4f32 => 4,
                                       },
                                       match attr.data.data_type {
                                           interface::DataType::Vec3f32 => gl::FLOAT,
                                           interface::DataType::Mat4f32 => gl::FLOAT,
                                       },
                                       gl::FALSE,
                                       buff.stride as i32,
                                       attr.data.offset as *const c_void);
                gl.EnableVertexAttribArray(attr.location.into());
            }

            self.binding_cache.insert(binding, buffer.id);
        }
    }
}

unsafe fn gen_vao(gl: &Gl) -> VaoId {
    let mut id: gl::types::GLuint = 0;
    gl.GenVertexArrays(1, &mut id);
    id
}


#[derive(Debug)]
pub struct OpenGlPipelineLayout {

}

#[derive(Debug)]
pub struct OpenGlDescriptorSet {

}

#[derive(Debug)]
enum OpenGlCommand {
    PreparePipeline(OpenGlPipeline),
    BindVertexBuffer(usize, OpenGlBuffer),
    BindIndexBuffer(OpenGlBuffer),
    DrawIndexed(u32, u32),
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
                    )
                }
            }
        }
    }
}

impl interface::CommandBuffer<Backend> for OpenGlCommandBuffer {
    fn prepare_pipeline(&mut self, pipeline: &<Backend as interface::Backend>::Pipeline) {
        self.cmds.push(PreparePipeline(pipeline.clone()));
    }

    fn bind_vertex_buffer(&mut self, binding: usize, buffer: &<Backend as interface::Backend>::Buffer) {
        self.cmds.push(BindVertexBuffer(binding, buffer.clone()))
    }

    fn bind_index_buffer(&mut self, buffer: &<Backend as interface::Backend>::Buffer) {
        self.cmds.push(BindIndexBuffer(buffer.clone()))
    }

    fn buffer_data(&mut self, buffer: &<Backend as interface::Backend>::Buffer, data: &[u8]) {
        unimplemented!()
    }

    fn draw_indexed(&mut self, count: u32, offset: u32) {
        self.cmds.push(DrawIndexed(count, offset))
    }

    fn bind_descriptor_set(&self, pipeline_layout: &<Backend as interface::Backend>::PipelineLayout, desc_set: &<Backend as interface::Backend>::DescriptorSet) {
        unimplemented!()
    }
}


