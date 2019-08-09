use std::cell::RefCell;
use std::ffi::CString;
use std::ops::Deref;
use std::os::raw::c_char;
use std::rc::Rc;
use std::sync::mpsc::Receiver;

pub use backend_interface::Shader;
use backend_interface::utils::Reloadable;
use gl::Gl;

pub struct OpenGLShader {
    id: u32,
    vert_id: u32,
    frag_id: u32,
    gl: Rc<Gl>,
}

impl OpenGLShader {
    pub fn new_vert_frag(vert_src: &str, frag_src: &str, gl: Rc<Gl>) -> Result<OpenGLShader, String> {
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
            let string = CString::new(src).unwrap();
            gl.ShaderSource(shader_id, 1,
                            &string.as_ptr(),
                            std::ptr::null());
            gl.CompileShader(shader_id);
        };
        OpenGLShader::validate_shader(gl, shader_id)
    }


    fn validate_program(gl: Rc<Gl>, vert_id: u32, frag_id: u32, id: u32) -> Result<OpenGLShader, String> {
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
        Ok(OpenGLShader { id, vert_id, frag_id, gl })
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

    fn load_mat4(&self, name: &str, mtx: &[f32]) {
        unsafe {
            let string = name.to_owned() + "\0";
            let locaiton = self.gl.GetUniformLocation(self.id, string.as_str().as_ptr() as *const c_char);
            self.gl.UniformMatrix4fv(locaiton, 1, 0,
                                     &mtx[0] as *const f32);
        }
    }

    fn unbind(&self) {
        unsafe { self.gl.UseProgram(0); }
    }
}

pub struct ReloadableOpenGLShader {
    gl: Rc<Gl>,
    shader: RefCell<Option<OpenGLShader>>,
    receiver: Receiver<(String, String)>,
}

impl ReloadableOpenGLShader {
    pub fn new(receiver: Receiver<(String, String)>, gl: Rc<Gl>) -> Self {
        ReloadableOpenGLShader { gl, shader: RefCell::new(None), receiver }
    }
}


impl Shader for ReloadableOpenGLShader {
    fn bind(&self) {
        self.reload_if_changed();
        if let Some(s) = self.shader.borrow().as_ref() {
            s.bind();
        }
    }

    fn load_mat4(&self, name: &str, mtx: &[f32]) {
        if let Some(s) = self.shader.borrow().as_ref() {
            s.load_mat4(name, mtx);
        }
    }

    fn unbind(&self) {
        if let Some(s) = self.shader.borrow().as_ref() {
            s.unbind();
        }
    }
}

impl Reloadable for ReloadableOpenGLShader {
    fn reload_if_changed(&self) {
        if let Ok((vert, frag)) = self.receiver.try_recv() {
            match OpenGLShader::new_vert_frag(
                vert.as_str(),
                frag.as_str(),
                self.gl.clone()) {
                Ok(s) => {
                    self.shader.borrow_mut().replace(s);
                }
                Err(s) => { println!("{}", s) }
            }
        }
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

fn to_gl_str(string: &str) -> *const i8 {
    string.as_ptr() as *const i8
}