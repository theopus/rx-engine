use std::os::raw::c_void;

use api::image;
use gl::Gl;

#[derive(Debug)]
pub struct OpenGlImage {
    id: u32,
    kind: image::Kind,
}

#[derive(Debug)]
pub struct OpenGlSampler {
    id: u32,
}


impl OpenGlImage {
    pub unsafe fn new(gl: &Gl, kind: image::Kind) -> Self {
        OpenGlImage { id: Self::generate(gl), kind }
    }

    pub unsafe fn bind_mem(&self, gl: &Gl, data: &[u8]) {
        match self.kind {
            image::Kind::D1(_, _) => unreachable!(),
            image::Kind::D2(width, heigth, level) =>
                Self::bind_d2(gl, self.id, width, heigth, level, data),
            image::Kind::D3(_, _, _) => unreachable!(),
        }
    }

    pub unsafe fn bind_d2(gl: &Gl, id: u32, width: u32, height: u32, level: u16, data: &[u8]) {
        gl.BindTexture(gl::TEXTURE_2D, id);

        //TODO: Configurable
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        gl.TexImage2D(gl::TEXTURE_2D,
                      level as i32,
                      gl::RGBA as i32,
                      width as i32,
                      height as i32,
                      0,
                      gl::RGBA,
                      gl::UNSIGNED_INT,
                      data.as_ptr() as *const c_void,
        );

        gl.BindTexture(gl::TEXTURE_2D, 0);
    }

    unsafe fn generate(gl: &gl::Gl) -> u32 {
        let mut id: gl::types::GLuint = 0;
        gl.GenTextures(1, &mut id);
        id
    }
}


impl OpenGlSampler {
    pub fn new(id: u32, gl: &Gl) -> Self {
        OpenGlSampler { id: 0 }
    }
}


pub struct OpenGlImageView;