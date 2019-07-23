extern crate glfw;
use crate::render::*;
use std::cell::RefCell;
use std::sync::mpsc::Receiver;
use glfw::{Action, Context, Glfw, Key};

pub fn create_pm(config: WindowConfig) -> Box<PlatformManager> {
    Box::new(GlfwPlatformManager::new(config)) as Box<PlatformManager>
}

pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
}

pub trait PlatformManager {
    fn create_renderer<'a>(&'a self, renderer_type: RendererType) -> (Box<RendererApi + 'a>, Box<RendererConstructor + 'a>);
    fn should_close(&self) -> bool;
    fn process_events(&self);
}

pub struct GlfwPlatformManager {
    glfw: RefCell<glfw::Glfw>,
    window: RefCell<glfw::Window>,
    events: Receiver<(f64, glfw::WindowEvent)>,
}

impl GlfwPlatformManager {
    pub fn swap_buffers(&self) {
        self.window.borrow_mut().swap_buffers();
    }
}

impl GlfwPlatformManager {
    fn new(config: WindowConfig) -> GlfwPlatformManager {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let (mut window, events) = glfw.create_window(config.width, config.height, "Hello this is window",
                                                      glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");
        window.set_key_polling(true);
        window.make_current();
        window.show();
        GlfwPlatformManager {
            glfw: RefCell::from(glfw),
            window: RefCell::from(window),
            events,
        }
    }
}

impl PlatformManager for GlfwPlatformManager {
    fn create_renderer<'a>(&'a self, renderer_type: RendererType) -> (Box<RendererApi + 'a>, Box<RendererConstructor + 'a>) {
        match renderer_type {
            RendererType::OpenGL => {
                return (Box::from(OpenGLRendererApi::new(gl::Gl::load_with(|s| {
                    self.window.borrow_mut().get_proc_address(s) as *const std::os::raw::c_void
                }), self)), Box::from(OpenGLRendererConstructor {}));
            }
            _ => panic!("Not implemented for {:?} renderer type", renderer_type)
        }
    }

    fn should_close(&self) -> bool {
        self.window.borrow().should_close()
    }

    fn process_events(&self) {
        self.glfw.borrow_mut().poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            handle_window_event(&mut self.window.borrow_mut(), event);
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}