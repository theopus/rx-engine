use std::rc::Rc;

use crate::platform::*;

pub trait RendererConstructor {}

pub struct OpenGLRendererConstructor;

impl RendererConstructor for OpenGLRendererConstructor {}

pub trait RendererApi {
    fn swap_buffer(&self);
    fn clear_color(&self);
    fn set_clear_color(&self, r: f32, g: f32, b: f32, a: f32);
}

pub struct OpenGLRendererApi<'a> {
    gl_api: Rc<gl::Gl>,
    glfw_pm: &'a GlfwPlatformManager,
}

impl<'a> OpenGLRendererApi<'a> {
    pub fn new(gl_api: gl::Gl, glfw_pm: &'a GlfwPlatformManager) -> OpenGLRendererApi {
        OpenGLRendererApi { gl_api: Rc::from(gl_api), glfw_pm }
    }
}

impl<'a> RendererApi for OpenGLRendererApi<'a> {
    fn swap_buffer(&self) {
        self.glfw_pm.swap_buffers()
    }

    fn clear_color(&self) {
        unsafe { self.gl_api.Clear(gl::COLOR_BUFFER_BIT); }
    }

    fn set_clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe { self.gl_api.ClearColor(r, g, b, a); }
    }
}

#[derive(Debug)]
pub enum RendererType {
    None,
    OpenGL,
    Vulkan,
}