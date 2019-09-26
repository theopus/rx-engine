use crate::buffer_v2::OpenGlBuffer;
use crate::image::OpenGlImage;

pub struct OpenGlMemory {
    pub(crate) kind: Kind
}

impl OpenGlMemory {

}

pub enum Kind {
    Unbinded,
    Buffer(OpenGlBuffer),
    Texture(OpenGlImage),
}