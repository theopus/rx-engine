use std::rc::Rc;


#[derive(Debug, Clone)]
pub struct OpenGlBuffer {
    pub(crate)id: u32,
    pub(crate)target: u32,
    pub(crate)usage: u32,
    pub(crate)size: u32,
}

impl OpenGlBuffer {
    pub fn new(gl: &gl::Gl, desc: interface::BufferDescriptor) -> OpenGlBuffer {
        unsafe {
            gl.TexImage2D(
                target,
                0,
                    vformat,



            )
            let id = Self::generate(gl);
            let buffer = OpenGlBuffer { id, target: to_gl_buffer_type(&desc.usage), usage: gl::STATIC_DRAW, size: desc.size };
            buffer.bind(gl);
            Self::buffer_empty(gl, &buffer);
            buffer
        }
    }

    pub unsafe fn generate(gl: &gl::Gl) -> u32 {
        let mut id: gl::types::GLuint = 0;
        gl.GenBuffers(1, &mut id);
        id
    }

    pub unsafe fn bind(&self, gl: &gl::Gl) {
        gl.BindBuffer(self.target, self.id);
    }

    pub unsafe fn buffer_empty(gl: &gl::Gl, buffer: &OpenGlBuffer) {
        gl.BufferData(buffer.target, buffer.size as isize, std::ptr::null(), buffer.usage);
    }

    pub fn mapper(gl: Rc<gl::Gl>, buffer: &OpenGlBuffer) -> *mut u8 {
        unsafe { buffer.bind(&gl) };
        unsafe { gl.MapBuffer(buffer.target, gl::READ_WRITE) as *mut u8 }
    }

    pub fn unmap(gl: Rc<gl::Gl>, buffer: &OpenGlBuffer) {
        unsafe { gl.UnmapBuffer(buffer.target); };
    }
}

pub(crate) fn to_gl_buffer_type(u: &interface::Usage) -> u32 {
    match u {
        interface::Usage::Vertex => gl::ARRAY_BUFFER,
        interface::Usage::Index => gl::ELEMENT_ARRAY_BUFFER,
        interface::Usage::Uniform => gl::UNIFORM_BUFFER
    }
}
