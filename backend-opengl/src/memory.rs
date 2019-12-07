use std::sync::Arc;

use gl::Gl;

use crate::buffer_v2::OpenGlBuffer;
use crate::image::OpenGlImage;

pub struct OpenGlMemory {
    capacity: u32,
    pub(crate) kind: Kind,
    storage: Option<Arc<*mut u8>>,
}

unsafe impl Send for OpenGlMemory {}

unsafe impl Sync for OpenGlMemory {}

impl OpenGlMemory {
    pub fn allocate(capacity: u32) -> Self {
        OpenGlMemory { capacity, kind: Kind::Unbinded(false), storage: None }
    }

    pub fn bind_image(&mut self, image: &OpenGlImage) {
        self.kind = Kind::Texture(image.clone())
    }

    pub fn bind_buffer(&mut self, buffer: &OpenGlBuffer) {
        self.kind = Kind::Buffer(buffer.clone());
    }
    pub fn map_memory(&self, gl: &Gl) -> *mut u8 {
        match &self.kind {
            Kind::Unbinded(allocated) => unimplemented!(),
            Kind::Buffer(buffer) => crate::buffer_v2::OpenGlBuffer::mapper(gl, buffer),
            Kind::Texture(_) => unimplemented!()
        }
    }

    pub fn unmap_memory(&self, gl: &Gl) {
        match &self.kind {
            Kind::Unbinded(allocated) => unimplemented!(),
            Kind::Buffer(buffer) => crate::buffer_v2::OpenGlBuffer::unmap(gl, buffer),
            Kind::Texture(_) => unimplemented!()
        }
    }

    pub fn flush_memory(&self, gl: &Gl) {
        match &self.kind {
            Kind::Unbinded(allocated) => unimplemented!(),
            Kind::Buffer(buffer) => crate::buffer_v2::OpenGlBuffer::flush(gl, buffer),
            Kind::Texture(_) => unimplemented!()
        }
    }
}


pub enum Kind {
    Unbinded(bool),
    Buffer(OpenGlBuffer),
    Texture(OpenGlImage),
}