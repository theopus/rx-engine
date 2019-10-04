use gl::Gl;
#[derive(Debug)]
pub struct OpenGlFramebuffer {
    id: u32
}

impl OpenGlFramebuffer {
    pub unsafe fn new(gl: &Gl) -> Self {
        OpenGlFramebuffer {
            id: Self::generate(gl)
        }
    }
    unsafe fn generate(gl: &gl::Gl) -> u32 {
        let mut id: gl::types::GLuint = 0;
        gl.GenFramebuffers(1, &mut id);
        id
    }
}

