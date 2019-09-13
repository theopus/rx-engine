use std::{
    mem,
    os::raw::c_void,
    rc::Rc,
};

use backend_interface::Backend as InterfaceBackend;
use backend_interface::shared_types::Type;

use crate::Backend;
use crate::interface::{BufferLayout, IndexBuffer, VertexArray, VertexBuffer};

pub struct GLBuffer {
    id: u32,
    target: gl::types::GLenum,
    usage: gl::types::GLenum,
    gl: Rc<gl::Gl>,
}

impl GLBuffer {
    pub fn new(target: gl::types::GLenum, usage: gl::types::GLenum, gl: Rc<gl::Gl>) -> Self {
        unsafe {
            let mut id: gl::types::GLuint = 0;
            gl.GenBuffers(1, &mut id);
            GLBuffer { id, target, usage, gl }
        }
    }

    pub fn bind(&self) {
        unsafe { self.gl.BindBuffer(self.target, self.id) }
    }

    fn buffer_size(&self, size: u32) {
        unsafe { self.gl.BufferData(self.target, size as gl::types::GLsizeiptr, std::ptr::null(), self.usage) }
    }

    pub fn unbind(&self) {
        unsafe { self.gl.BindBuffer(self.target, 0) }
    }
}

impl Buffer<&[u32]> for GLBuffer {
    fn buffer_data(&self, data: &[u32]) {
        self.bind();
        unsafe { self.gl.BufferData(self.target, mem::size_of_val(data) as gl::types::GLsizeiptr, data.as_ptr() as *const c_void, self.usage) }
        self.unbind();
    }

    fn new(target: gl::types::GLenum, usage: gl::types::GLenum, gl: Rc<gl::Gl>, data: &[u32]) -> Self {
        let buffer = GLBuffer::new(target, usage, gl);
        buffer.buffer_data(data);
        buffer
    }
}

impl Buffer<&[f32]> for GLBuffer {
    fn buffer_data(&self, data: &[f32]) {
        self.bind();
        unsafe { self.gl.BufferData(self.target, mem::size_of_val(data) as gl::types::GLsizeiptr, data.as_ptr() as *const c_void, self.usage) }
        self.unbind();
    }

    fn new(target: gl::types::GLenum, usage: gl::types::GLenum, gl: Rc<gl::Gl>, data: &[f32]) -> Self {
        let buffer = GLBuffer::new(target, usage, gl);
        buffer.buffer_data(data);
        buffer
    }
}

trait Buffer<T> {
    fn buffer_data(&self, data: T);
    fn new(target: gl::types::GLenum, usage: gl::types::GLenum, gl: Rc<gl::Gl>, data: T) -> Self;
}

impl Drop for GLBuffer {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteBuffers(1, &self.id) }
    }
}

pub struct OpenGLIndexBuffer {
    size: u32,
    buffer: GLBuffer,
}

impl OpenGLIndexBuffer {
    pub fn new(data: &[u32], gl: Rc<gl::Gl>) -> Self {
        OpenGLIndexBuffer {
            size: data.len() as u32,
            buffer: Buffer::new(gl::ELEMENT_ARRAY_BUFFER,
                                gl::STATIC_DRAW,
                                gl,
                                data),
        }
    }
}


impl IndexBuffer<Backend> for OpenGLIndexBuffer {
    fn bind(&self) {
        self.buffer.bind();
    }

    fn unbind(&self) {
        self.buffer.unbind();
    }

    fn length(&self) -> u32 {
        self.size
    }

    fn buffer_data(&self, data: &[u32]) {
        self.buffer.buffer_data(data)
    }
}

impl Drop for OpenGLIndexBuffer {
    fn drop(&mut self) {
        //
    }
}

pub struct OpenGLVertexBuffer {
    buffer: GLBuffer,
    layout: Option<BufferLayout>,
}

impl OpenGLVertexBuffer {
    pub fn new(gl: Rc<gl::Gl>) -> Self {
        OpenGLVertexBuffer {
            buffer: GLBuffer::new(
                gl::ARRAY_BUFFER,
                gl::STATIC_DRAW,
                gl,
            ),
            layout: Option::None,
        }
    }
}

impl Drop for OpenGLVertexBuffer {
    fn drop(&mut self) {
        //
    }
}

impl VertexBuffer<Backend> for OpenGLVertexBuffer {
    fn bind(&self) {
        self.buffer.bind()
    }

    fn set_buffer_layout(&mut self, layout: BufferLayout) {
        self.layout = Option::from(layout)
    }

    fn get_buffer_layout(&self) -> &BufferLayout {
        match &self.layout {
            Some(layout) => {
                &layout
            }
            _ => panic!()
        }
    }

    fn buffer_data_f32(&self, data: &[f32]) {
        self.buffer.buffer_data(data)
    }

    fn buffer_data_u32(&self, data: &[u32]) {
        self.buffer.buffer_data(data)
    }

    fn unbind(&self) {
        self.buffer.unbind()
    }
}

pub struct OpenGLVertexArray {
    id: u32,
    attribute_index: u32,
    gl: Rc<gl::Gl>,
    index_buffer: Option<<Backend as InterfaceBackend>::IndexBuffer>,
    vertex_buffers: Vec<<Backend as InterfaceBackend>::VertexBuffer>,
}

impl OpenGLVertexArray {
    pub fn new(gl: Rc<gl::Gl>) -> Self {
        unsafe {
            let mut id: gl::types::GLuint = 0;
            gl.GenVertexArrays(1, &mut id);
            OpenGLVertexArray {
                id,
                attribute_index: 0,
                gl,
                index_buffer: None,
                vertex_buffers: Vec::new(),
            }
        }
    }
}

impl Drop for OpenGLVertexArray {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteVertexArrays(1, &self.id as *const u32) }
    }
}

impl VertexArray<Backend> for OpenGLVertexArray {
    fn id(&self) -> u32 {
        self.id
    }

    fn bind(&self) {
        unsafe { self.gl.BindVertexArray(self.id) }
    }

    fn add_vertex_buffer(&mut self, vertex_buffer: <Backend as InterfaceBackend>::VertexBuffer) {
        self.bind();

        let mut attribute_offset = 0;
        for e in vertex_buffer.get_buffer_layout().elements() {
            vertex_buffer.bind();
            unsafe {
                self.gl.VertexAttribPointer(self.attribute_index,
                                            i32::from(e.0),
                                            to_gl_type(&e.3),
                                            0,
                                            vertex_buffer.get_buffer_layout().stride() as i32,
                                            attribute_offset as *const c_void);
                self.gl.EnableVertexAttribArray(self.attribute_index);
            }
            attribute_offset += e.2;
            self.attribute_index += 1;
        }
        self.vertex_buffers.push(vertex_buffer);
        self.unbind();
    }

    fn set_index_buffer(&mut self, index_buffer: <Backend as InterfaceBackend>::IndexBuffer) {
        self.bind();
        self.index_buffer = Some(index_buffer);
        self.index_buffer.as_ref().unwrap().bind();
        self.unbind();
    }

    fn get_index_buffer(&self) -> &<Backend as InterfaceBackend>::IndexBuffer {
        self.index_buffer.as_ref().unwrap()
    }

    fn unbind(&self) {
        unsafe { self.gl.BindVertexArray(0) }
    }
}

pub(crate) fn to_gl_buffer_type(u: &interface::Usage) -> u32 {
    match u {
        interface::Usage::Vertex => gl::ARRAY_BUFFER,
        interface::Usage::Index => gl::ELEMENT_ARRAY_BUFFER,
        interface::Usage::Uniform => gl::UNIFORM_BUFFER
    }
}

fn to_gl_type(t: &Type) -> gl::types::GLenum {
    match t {
        Type::Float => { gl::FLOAT }
        Type::Float2 => { gl::FLOAT }
        Type::Float3 => { gl::FLOAT }
        Type::Float4 => { gl::FLOAT }
        Type::Mat4 => { gl::FLOAT }
        Type::Int => { gl::INT }
        Type::Int2 => { gl::INT }
        Type::Int3 => { gl::INT }
    }
}