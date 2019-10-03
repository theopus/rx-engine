use crate::buffer_v2::OpenGlBuffer;
use crate::image::OpenGlImage;

pub struct OpenGlMemory {
    capacity: u32,
    pub(crate) kind: Kind,
}

impl OpenGlMemory {
    fn allocate(capacity: u32) -> Self {
        OpenGlMemory { capacity, kind: Kind::Unbinded }
    }

    fn bind_image(image: OpenGlImage) {

    }
}

pub enum Kind {
    Unbinded,
    Buffer(OpenGlBuffer),
    Texture(OpenGlImage),
}