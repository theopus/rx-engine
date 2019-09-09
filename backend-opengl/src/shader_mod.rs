use std::ffi::CString;

use gl::Gl;

type ShaderId = gl::types::GLuint;
type ShaderKind = gl::types::GLenum;

#[derive(Debug)]
pub struct OpenGlShaderMod {
    pub(crate) id: ShaderId,
    kind: ShaderKind,
}

impl OpenGlShaderMod {
    pub fn new(gl: &Gl, desc: interface::ShaderModDescriptor) -> Result<Self, String> {
        let kind = match desc.stype {
            interface::ShaderType::Vertex => gl::VERTEX_SHADER,
            interface::ShaderType::Fragment => gl::FRAGMENT_SHADER,
        };
        let id = shader_from_src(gl, &desc.source, kind)?;

        Ok(OpenGlShaderMod { id, kind })
    }
}

impl OpenGlShaderMod {}


#[derive(Debug)]
pub struct OpenGlDescriptorSetLayout {}


fn shader_from_src(gl: &Gl, src: &str, kind: gl::types::GLenum) -> Result<u32, String> {
    let shader_id = unsafe { gl.CreateShader(kind) };
    unsafe {
        let string = CString::new(src).unwrap();
        gl.ShaderSource(shader_id, 1,
                        &string.as_ptr(),
                        std::ptr::null());
        gl.CompileShader(shader_id);
    };
    validate_shader(gl, shader_id)
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

pub (crate) fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}