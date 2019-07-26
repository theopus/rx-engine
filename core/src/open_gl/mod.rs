use std::rc::Rc;

use glfw::{
    Context,
    RenderContext,
};

use crate::{
    render::{
        RendererApi,
        IndexBuffer,
        RendererConstructor,
        Shader,
        shared_types::Type,
        VertexArray,
        VertexBuffer,
        BufferLayout,
    }
};

use self::{
    shader::{
        OpenGLShader,
    },
    buffer::{
        OpenGLIndexBuffer,
        OpenGLVertexArray,
        OpenGLVertexBuffer,
    }
};

mod buffer;
mod shader;

pub struct OpenGLRendererConstructor {
    gl_api: Rc<gl::Gl>
}

impl OpenGLRendererConstructor {
    pub fn new(gl_api: Rc<gl::Gl>) -> Self {
        OpenGLRendererConstructor { gl_api }
    }
}

impl RendererConstructor for OpenGLRendererConstructor {
    fn vertex_array(&self) -> Box<VertexArray> {
        Box::new(OpenGLVertexArray::new(self.gl_api.clone()))
    }

    fn vertex_buffer(&self) -> Box<VertexBuffer> {
        Box::new(OpenGLVertexBuffer::new(self.gl_api.clone()))
    }

    fn index_buffer(&self, indexes: &[u32]) -> Box<IndexBuffer> {
        Box::new(OpenGLIndexBuffer::new(indexes, self.gl_api.clone()))
    }

    fn shader(&self, vertex_src: &str, fragment_src: &str, mem_layout: &BufferLayout) -> Box<Shader> {
        OpenGLShader::new_vert_frag(vertex_src, fragment_src, self.gl_api.clone()).expect("Error during shader creation")
    }
}

pub struct OpenGLRendererApi {
    gl_api: Rc<gl::Gl>,
    render_ctx: RenderContext,
}

impl OpenGLRendererApi {
    pub fn new(gl_api: Rc<gl::Gl>, render_ctx: RenderContext) -> OpenGLRendererApi {
        unsafe { gl_api.Viewport(0, 0, 600, 400); }
        OpenGLRendererApi { gl_api, render_ctx }
    }
}

impl RendererApi for OpenGLRendererApi {
    fn swap_buffer(&mut self) {
        self.render_ctx.swap_buffers()
    }


    fn draw_indexed(&self, vertex_array: &VertexArray) {
        vertex_array.bind();
        unsafe {
            self.gl_api.DrawElements(gl::TRIANGLES,
                                     vertex_array.get_index_buffer().length() as i32,
                                     gl::UNSIGNED_INT,
                                     std::ptr::null())
        }
        //TODO: Unbinding
        vertex_array.unbind();
    }

    fn clear_color(&self) {
        unsafe { self.gl_api.Clear(gl::COLOR_BUFFER_BIT); }
    }

    fn set_clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe { self.gl_api.ClearColor(r, g, b, a); }
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