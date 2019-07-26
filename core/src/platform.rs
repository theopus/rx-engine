extern crate glfw;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::Receiver;

use glfw::{Action, Context, Glfw, Key};

use crate::open_gl::*;
use crate::render::*;

pub fn create_pm(config: WindowConfig) -> Box<PlatformManager> {
    Box::new(GlfwPlatformManager::new(config)) as Box<PlatformManager>
}

pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
}

pub trait PlatformManager {
    fn create_renderer(&self, renderer_type: RendererType) -> (Box<RendererApi>, Box<RendererConstructor>);
    fn should_close(&self) -> bool;
    fn process_events(&self);
    fn current_time(&self) -> f64;
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

        //excplicit 3.3 (needed for macOS)
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) =
            glfw.create_window(config.width, config.height, "Hello this is window",
                               glfw::WindowMode::Windowed)
                .expect("Failed to create GLFW window.");

        window.set_key_polling(true);
        glfw.make_context_current(Option::from(&window));
        window.show();
        GlfwPlatformManager {
            glfw: RefCell::from(glfw),
            window: RefCell::from(window),
            events,
        }
    }
}

impl PlatformManager for GlfwPlatformManager {
    fn create_renderer(&self, renderer_type: RendererType)
                       -> (Box<RendererApi>, Box<RendererConstructor>) {
        match renderer_type {
            RendererType::OpenGL => {
                let gl = gl::Gl::load_with(|s| {
                    self.window.borrow_mut().get_proc_address(s) as *const std::os::raw::c_void
                });
                let gl = Rc::from(gl);
                (Box::from(OpenGLRendererApi::new(gl.clone(), self.window.borrow_mut().render_context())), Box::from(OpenGLRendererConstructor::new(gl.clone())))
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

    fn current_time(&self) -> f64 {
        self.glfw.borrow().get_time()
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
        window.set_should_close(true)
    }
}