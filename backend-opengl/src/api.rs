use std::{fs, path::Path, rc::Rc, sync::mpsc::Receiver};
use std::os::raw::c_void;

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
use core::borrow::Borrow;

#[derive(Clone)]
pub struct OpenGLRendererDevice {
    gl_api: Rc<gl::Gl>,
}

impl OpenGLRendererDevice {
    pub fn new(gl_api: Rc<gl::Gl>) -> Self {
        OpenGLRendererDevice { gl_api }
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

    fn create_buffer(&self, desc: interface::BufferDescriptor) -> <Backend as interface::Backend>::Buffer {
        crate::buffer_v2::OpenGlBuffer::new(&self.gl_api, desc)
    }

    fn map_buffer(&self, buffer: &<Backend as interface::Backend>::Buffer) -> *mut u8 {
        crate::buffer_v2::OpenGlBuffer::mapper(self.gl_api.clone(), buffer)
    }

    fn unmap_buffer(&self, buffer: &<Backend as interface::Backend>::Buffer) {
        crate::buffer_v2::OpenGlBuffer::unmap(self.gl_api.clone(), buffer)
    }

    fn create_pipeline(&self, desc: interface::PipelineDescriptor<Backend>) -> <Backend as interface::Backend>::Pipeline {
        unsafe {
            crate::pipeline::OpenGlPipeline::new(&self.gl_api, desc)
                .expect("err")
        }
    }

    fn create_cmd_buffer(&self) -> <Backend as interface::Backend>::CommandBuffer {
        crate::pipeline::OpenGlCommandBuffer::new()
    }

    fn allocate_descriptor_set(&self, desc: &<Backend as interface::Backend>::DescriptorSetLayout) -> <Backend as interface::Backend>::DescriptorSet {
        crate::pipeline::OpenGlDescriptorSet {}
    }

    fn execute(&self, mut cmd: <Backend as interface::Backend>::CommandBuffer) {
        unsafe { cmd.execute(&self.gl_api); };
    }

    fn create_shader_mod(&self, desc: interface::ShaderModDescriptor) -> <Backend as interface::Backend>::ShaderMod {
        crate::shader_mod::OpenGlShaderMod::new(&self.gl_api, desc)
            .expect("err")
    }

    fn create_descriptor_set_layout(&self, bindings: &[interface::DescriptorSetLayoutBinding]) -> <Backend as interface::Backend>::DescriptorSetLayout {
        crate::pipeline::OpenGlDescriptorSetLayout::new(bindings)
    }

    fn create_pipeline_layout<I>(&self, desc_layout: &<Backend as interface::Backend>::DescriptorSetLayout, hints: I) -> <Backend as interface::Backend>::PipelineLayout
        where
            I: IntoIterator<Item = interface::PipelineLayoutHint>,{
        crate::pipeline::OpenGlPipelineLayout::new(desc_layout, hints)
    }

    fn write_descriptor_set(&self, desc_set_write: interface::DescriptorSetWrite<Backend>) {
        match &desc_set_write.descriptor {
            interface::Descriptor::Buffer(buffer) => {
                unsafe { self.gl_api.BindBufferBase(buffer.target, desc_set_write.binding, buffer.id); };
            }
        };
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
                                     0 as *const c_void)
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