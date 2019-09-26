use crate::Backend;

#[derive(Debug)]
pub struct OpenGlFramebuffer {}

pub struct OpenGlSurface {
    pub(crate) swap_buffers_fn: Box<Fn() -> Box<FnMut()>>,
}

pub struct OpenGlSwapchain {
    swap_buffers: Box<FnMut()>,
}

impl OpenGlSwapchain {
    pub fn new(surface: &OpenGlSurface) -> Self {
        OpenGlSwapchain {
            swap_buffers: (*surface.swap_buffers_fn)()
        }
    }
}

impl interface::Swapchain<Backend> for OpenGlSwapchain {
    fn present(&mut self, frame_index: u32) {
        (self.swap_buffers)()
    }
}
