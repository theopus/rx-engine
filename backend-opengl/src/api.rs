use std::{path::Path, rc::Rc, sync::mpsc::Receiver, fs};

use backend_interface::{
    IndexBuffer,
    Backend as InterfaceBackend,
    RendererApi,
    RendererConstructor,
    Shader,
    VertexArray,
    VertexBuffer,
    BufferLayout,
    utils::ResourceListener
};

use crate::{
    buffer::{
        OpenGLVertexArray,
        OpenGLIndexBuffer,
        OpenGLVertexBuffer
    },
    Backend,
    shader::OpenGLShader
};
use crate::shader::ReloadableOpenGLShader;

pub struct OpenGLRendererConstructor {
    gl_api: Rc<gl::Gl>,
    rl: ResourceListener
}

impl OpenGLRendererConstructor {
    pub fn new(gl_api: Rc<gl::Gl>) -> Self {
        let mut listener = ResourceListener::new();
        listener.start();
        OpenGLRendererConstructor { gl_api, rl: listener }
    }
}

impl RendererConstructor<Backend> for OpenGLRendererConstructor {
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

//        OpenGLShader::new_vert_frag(&fs::read_to_string(vertex).expect(""),
//                                    &fs::read_to_string(fragment).expect(""),
//                                    self.gl_api.clone()).expect("Error during shader creation")
    }
}

pub struct OpenGLRendererApi {
    gl_api: Rc<gl::Gl>,
    swap_buffers: Box<FnMut() -> ()>,
}

impl OpenGLRendererApi {
    pub fn new(gl_api: Rc<gl::Gl>, swap_buffers: Box<FnMut() -> ()>) -> OpenGLRendererApi {
        unsafe { gl_api.Viewport(0, 0, 600, 400); }
        OpenGLRendererApi { gl_api, swap_buffers }
    }
}

impl RendererApi<Backend> for OpenGLRendererApi {
    fn swap_buffer(&mut self) {
        (self.swap_buffers)()
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