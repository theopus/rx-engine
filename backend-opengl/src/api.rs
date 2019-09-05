use std::{fs, path::Path, rc::Rc, sync::mpsc::Receiver};

use backend_interface::{
    Backend as InterfaceBackend,
    BufferLayout,
    IndexBuffer,
    RendererApi,
    RendererDevice,
    Shader,
    utils::ResourceListener,
    VertexArray,
    VertexBuffer,
};

use crate::{
    Backend,
    buffer,
    buffer::{
        OpenGLIndexBuffer,
        OpenGLVertexArray,
        OpenGLVertexBuffer,
    },
    shader::OpenGLShader,
};
use crate::shader::ReloadableOpenGLShader;

pub struct OpenGLRendererDevice {
    gl_api: Rc<gl::Gl>,
    rl: ResourceListener,
}

impl OpenGLRendererDevice {
    pub fn new(gl_api: Rc<gl::Gl>) -> Self {
        let mut listener = ResourceListener::new();
        listener.start();
        OpenGLRendererDevice { gl_api, rl: listener }
    }
}

impl RendererDevice<Backend> for OpenGLRendererDevice {
    fn vertex_array(&self) -> <Backend as InterfaceBackend>::VertexArray {
        OpenGLVertexArray::new(self.gl_api.clone())
    }

    fn vertex_buffer(&self) -> <Backend as InterfaceBackend>::VertexBuffer {
        OpenGLVertexBuffer::new(self.gl_api.clone())
    }

    fn index_buffer(&self, indexes: &[u32]) -> <Backend as InterfaceBackend>::IndexBuffer {
        OpenGLIndexBuffer::new(indexes, self.gl_api.clone())
    }


    #[cfg(not(feature = "hot_reload"))]
    fn shader(&self, vertex: &Path, fragment: &Path, mem_layout: &BufferLayout) -> <Backend as InterfaceBackend>::Shader {
        OpenGLShader::new_vert_frag(&fs::read_to_string(vertex).expect(""),
                                    &fs::read_to_string(fragment).expect(""),
                                    self.gl_api.clone()).expect("Error during shader creation")
    }

    #[cfg(feature = "hot_reload")]
    fn shader(&self, vertex: &Path, fragment: &Path, mem_layout: &BufferLayout) -> <Backend as InterfaceBackend>::Shader {
        ReloadableOpenGLShader::new(self.rl.listen_pair(
            vertex.to_str().unwrap(),
            fragment.to_str().unwrap(),
        ), self.gl_api.clone())
    }

    fn buffer(&self) -> <Backend as InterfaceBackend>::Buffer {
        buffer::OpenGlBuffer::new(&self.gl_api)
    }
}

pub struct OpenGLRendererApi {
    gl_api: Rc<gl::Gl>,
    swap_buffers: Box<FnMut() -> ()>,
}

impl OpenGLRendererApi {
    pub fn new(gl_api: Rc<gl::Gl>, swap_buffers: Box<FnMut() -> ()>) -> OpenGLRendererApi {
        OpenGLRendererApi { gl_api, swap_buffers }
    }
}

impl RendererApi<Backend> for OpenGLRendererApi {
    fn swap_buffer(&mut self) {
        (self.swap_buffers)()
    }

    fn viewport(&self, w: i32, h: i32) {
        unsafe { self.gl_api.Viewport(0, 0, w, h) };
    }


    fn draw_indexed(&self, vertex_array: &<Backend as InterfaceBackend>::VertexArray) {
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