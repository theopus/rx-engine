use gl::Gl;

use crate::buffer_v2::OpenGlBuffer;
use crate::image::OpenGlImage;
use std::sync::Arc;

pub struct OpenGlMemory {
    capacity: u32,
    pub(crate) kind: Kind,
    storage: Option<Arc<*mut u8>>,
}

unsafe impl Send for OpenGlMemory {}
unsafe impl Sync for OpenGlMemory {}

impl OpenGlMemory {
    fn allocate(capacity: u32) -> Self {
        OpenGlMemory { capacity, kind: Kind::Unbinded, storage: None }
    }

    fn bind_image(image: &OpenGlImage) {

    }

    fn bind_buffer(buffer: &OpenGlBuffer) {

    }

    fn map_memory(&self, gl: &Gl) -> *mut u8 {
        match &self.kind {
            Kind::Unbinded => unimplemented!(),
            Kind::Buffer(buffer) => crate::buffer_v2::OpenGlBuffer::mapper(gl, buffer),
            Kind::Texture(_) => unimplemented!()
        }
    }
}

pub enum Kind {
    Unbinded,
    Buffer(OpenGlBuffer),
    Texture(OpenGlImage),
}