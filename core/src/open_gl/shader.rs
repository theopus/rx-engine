use std::ffi::CStr;
use std::ffi::CString;
use std::rc::Rc;

use gl::Gl;

use crate::render::Shader;

pub struct OpenGLShader {
    id: u32,
    vert_id: u32,
    frag_id: u32,
    gl: Rc<Gl>,
}

impl OpenGLShader {
    pub fn new_vert_frag(vert_src: &str, frag_src: &str, gl: Rc<Gl>) -> Result<Box<Shader>, String> {
        let vert_id = OpenGLShader::shader_from_src(&gl, vert_src, gl::VERTEX_SHADER)?;
        let frag_id = OpenGLShader::shader_from_src(&gl, frag_src, gl::FRAGMENT_SHADER)?;

        let id = unsafe { gl.CreateProgram() };
        unsafe {
            gl.AttachShader(id, vert_id);
            gl.AttachShader(id, frag_id);
            gl.LinkProgram(id);
            gl.DetachShader(id, vert_id);
            gl.DetachShader(id, frag_id);
        };
        OpenGLShader::validate_program(gl, vert_id, frag_id, id)
    }

    fn shader_from_src(gl: &Gl, src: &str, kind: gl::types::GLenum) -> Result<u32, String> {
        let shader_id = unsafe { gl.CreateShader(kind) };
        unsafe {
            gl.ShaderSource(shader_id, 1,
                            &CString::new(src).unwrap().as_ptr(),
                            std::ptr::null());
            gl.CompileShader(shader_id);
        };
        OpenGLShader::validate_shader(gl, shader_id)
    }


    fn validate_program(gl: Rc<Gl>, vert_id: u32, frag_id: u32, id: u32) -> Result<Box<Shader>, String> {
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl.GetProgramiv(id, gl::LINK_STATUS, &mut success);
        }
        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl.GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl.GetProgramInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }
        Ok(Box::new(OpenGLShader { id, vert_id, frag_id, gl }))
    }

    fn validate_shader(gl: &Gl, shader_id: u32) -> Result<u32, String> {
        let mut success: i32 = 1;
        unsafe {
            gl.GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
        }
        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl.GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut len);
            }
            let error = create_whitespace_cstring_with_len(len as usize);
            unsafe {
                gl.GetShaderInfoLog(shader_id,
                                    len,
                                    std::ptr::null_mut(),
                                    error.as_ptr() as *mut gl::types::GLchar);
            }
            return Err(error.to_string_lossy().into_owned());
        }
        Ok(shader_id)
    }
}

impl Shader for OpenGLShader {
    fn bind(&self) {
        unsafe { self.gl.UseProgram(self.id); }
    }

    fn unbind(&self) {
        unsafe { self.gl.UseProgram(0); }
    }
}

impl Drop for OpenGLShader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.vert_id);
            self.gl.DeleteShader(self.frag_id);
            self.gl.DeleteProgram(self.id);
        }
    }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}