use core::borrow::Borrow;
use std::{fs, path::Path, rc::Rc, sync::mpsc::Receiver};
use std::os::raw::c_void;

use backend_api::{
    Backend as apiBackend,
    RendererApi,
    RendererDevice,
};

use crate::Backend;
use crate::Backend as MyBackend;
use crate::image::{OpenGlImage, OpenGlImageView};

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
    fn allocate_memory(&self, size: u32) -> <Backend as api::Backend>::Memory {
        crate::memory::OpenGlMemory::allocate(size)
    }

    fn map_memory(&self, memory: &<Backend as api::Backend>::Memory) -> *mut u8 {
        memory.map_memory(&self.gl_api)
    }

    fn flush_memory(&self, memory: &<Backend as api::Backend>::Memory) {
        memory.flush_memory(&self.gl_api)
    }

    fn unmap_memory(&self, memory: &<Backend as api::Backend>::Memory) {
        memory.unmap_memory(&self.gl_api)
    }

    fn bind_buffer_memory(&self,
                          memory: &mut <Backend as api::Backend>::Memory,
                          buffer: &<Backend as api::Backend>::Buffer) {
        memory.bind_buffer(buffer)
    }

    fn create_buffer(&self, desc: api::BufferDescriptor) -> <Backend as api::Backend>::Buffer {
        crate::buffer_v2::OpenGlBuffer::new(&self.gl_api, desc)
    }

    fn create_pipeline(&self, desc: api::PipelineDescriptor<Backend>) -> <Backend as api::Backend>::Pipeline {
        unsafe {
            crate::pipeline::OpenGlPipeline::new(&self.gl_api, desc)
                .expect("err")
        }
    }

    fn create_cmd_buffer(&self) -> <Backend as api::Backend>::CommandBuffer {
        crate::pipeline::OpenGlCommandBuffer::new()
    }

    fn allocate_descriptor_set(&self, desc: &<Backend as api::Backend>::DescriptorSetLayout) -> <Backend as api::Backend>::DescriptorSet {
        crate::pipeline::OpenGlDescriptorSet {}
    }

    fn execute(&self, mut cmd: <Backend as api::Backend>::CommandBuffer) {
        unsafe { cmd.execute(&self.gl_api); };
    }

    fn create_shader_mod(&self, desc: api::ShaderModDescriptor) -> <Backend as api::Backend>::ShaderMod {
        crate::shader_mod::OpenGlShaderMod::new(&self.gl_api, desc)
            .expect("err")
    }

    fn create_descriptor_set_layout(&self, bindings: &[api::DescriptorSetLayoutBinding]) -> <Backend as api::Backend>::DescriptorSetLayout {
        crate::pipeline::OpenGlDescriptorSetLayout::new(bindings)
    }

    fn create_pipeline_layout<I>(&self, desc_layout: &<Backend as api::Backend>::DescriptorSetLayout, hints: I) -> <Backend as api::Backend>::PipelineLayout
        where
            I: IntoIterator<Item=api::PipelineLayoutHint>, {
        crate::pipeline::OpenGlPipelineLayout::new(desc_layout, hints)
    }

    fn write_descriptor_set(&self, desc_set_write: api::DescriptorSetWrite<Backend>) {
        match &desc_set_write.descriptor {
            api::Descriptor::Buffer(buffer) => {
                unsafe { self.gl_api.BindBufferBase(buffer.target, desc_set_write.binding, buffer.id); };
            }
        };
    }

    fn create_render_pass<A>(
        &self, attachments: A,
    ) -> <Backend as api::Backend>::RenderPass
        where
            A: IntoIterator<Item=api::Attachment> {
        crate::pipeline::OpenGlRenderPass::new(attachments)
    }

    fn create_framebuffer<I>(
        &self,
        render_pass: &<Backend as api::Backend>::RenderPass,
        attachments: I,
    ) -> <Backend as api::Backend>::Framebuffer
        where
            I: IntoIterator,
            I::Item: Borrow<<Backend as api::Backend>::ImageView> {
        unsafe { crate::framebuffer::OpenGlFramebuffer::new(&self.gl_api, render_pass, attachments) }
    }

    fn create_swapchain(
        &self,
        surface: &<Backend as api::Backend>::Surface,
    ) -> (<Backend as api::Backend>::Swapchain, Vec<<Backend as api::Backend>::Image>) {
        (crate::swapchain::OpenGlSwapchain::new(surface), Vec::new())
    }

    fn create_image(&self, kind: api::image::Kind) -> <Backend as api::Backend>::Image {
        unsafe { OpenGlImage::new(&self.gl_api, kind) }
    }

    fn create_image_view(&self, image: &<Backend as api::Backend>::Image) -> <Backend as api::Backend>::ImageView {
        image.clone()
    }

    fn bind_image_memory(
        &self, mem:
        &mut <Backend as api::Backend>::Memory,
        img: &<Backend as api::Backend>::Image
    ) {
        mem.bind_image(img)
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


//    fn draw_indexed(&self, vertex_array: &<Backend as apiBackend>::VertexArray) {
//        vertex_array.bind();
//        unsafe {
//            self.gl_api.DrawElements(gl::TRIANGLES,
//                                     vertex_array.get_index_buffer().length() as i32,
//                                     gl::UNSIGNED_INT,
//                                     0 as *const c_void)
//        }
//        //TODO: Unbinding
//        vertex_array.unbind();
//    }

    fn clear_color(&self) {
        unsafe { self.gl_api.Clear(gl::COLOR_BUFFER_BIT); }
    }

    fn set_clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe { self.gl_api.ClearColor(r, g, b, a); }
    }
}