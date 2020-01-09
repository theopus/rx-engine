use std::borrow::Borrow;

use gl::Gl;

use crate::Backend;
use crate::image::OpenGlImage;

#[derive(Debug)]
pub struct OpenGlFramebuffer {
    id: u32
}

impl OpenGlFramebuffer {
    pub unsafe fn new<A, I>(
        gl: &Gl,
        render_pass: &<Backend as api::Backend>::RenderPass,
        attachments: A,
    )
        -> Self
        where A: IntoIterator<Item=I>,
              I: Borrow<OpenGlImage> {
        let fb = OpenGlFramebuffer {
            id: Self::generate(gl)
        };

        for (i, a) in attachments.into_iter().enumerate() {
            let img: &OpenGlImage = a.borrow();
            println!("{:?}", img);
            println!("{:?}", render_pass.attachments.get(i));
            fb.attach(gl, render_pass.attachments.get(i).unwrap(), img);
        }
//        panic!();
        fb
    }

    unsafe fn bind(&self, gl: &Gl) {
        gl.BindFramebuffer(gl::FRAMEBUFFER, self.id)
    }

    unsafe fn unbind(&self, gl: &Gl) {
        gl.BindFramebuffer(gl::FRAMEBUFFER, 0)
    }

    unsafe fn attach(&self, gl: &Gl, attachment: &api::Attachment, img: &OpenGlImage) {
        //glGetIntegerv(GL_DRAW_FRAMEBUFFER_BINDING, &drawFboId);
        //glGetIntegerv(GL_READ_FRAMEBUFFER_BINDING, &readFboId);
        self.bind(gl);
        gl.FramebufferTexture2D(
            gl::FRAMEBUFFER,
            match attachment.layout {
                api::AttachmentLayout::Color => gl::COLOR_ATTACHMENT0,
                api::AttachmentLayout::Depth => gl::DEPTH_ATTACHMENT,
                api::AttachmentLayout::Stencil => gl::STENCIL_ATTACHMENT,
            },
            gl::TEXTURE_2D,
            img.id,
            0
        );
        self.unbind(gl)
    }


    unsafe fn generate(gl: &gl::Gl) -> u32 {
        let mut id: gl::types::GLuint = 0;
        gl.GenFramebuffers(1, &mut id);
        id
    }
}

