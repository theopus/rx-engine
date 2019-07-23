extern crate gl;
extern crate glfw;
extern crate specs;

use std::cell::RefCell;
use std::fs::read;
use std::mem;
use std::ops::Add;
use std::rc::Rc;
use std::sync::mpsc::Receiver;

use glfw::{Action, Context, Glfw, Key};

fn main() {
    let mut platform_manager: Box<PlatformManager> = GlfwPlatformManager::new(WindowConfig { width: 300, height: 300 });
    let renderer: Box<Renderer> = platform_manager.create_renderer(RendererType::OpenGL);

    while !platform_manager.should_close() {
        renderer.clear_color();
        platform_manager.process_events();
        renderer.swap_buffer()
    }
}


trait Renderer {
    fn swap_buffer(&self);

    fn clear_color(&self);
}

struct OpenGLRenderer<'a> {
    gl_api: Rc<gl::Gl>,
    glfw_pm: &'a GlfwPlatformManager,
}

impl<'a> OpenGLRenderer<'a> {
    fn new(gl_api: gl::Gl, glfw_pm: &'static GlfwPlatformManager) -> Box<'a + Renderer> {
        Box::new(OpenGLRenderer {
            gl_api: Rc::from(gl_api),
            glfw_pm,
        })
    }
}

impl<'a> Renderer for OpenGLRenderer<'a> {
    fn swap_buffer(&self) {
        self.glfw_pm.swap_buffers()
    }

    fn clear_color(&self) {
        unsafe {
            self.gl_api.ClearColor(0.3, 0.3, 0.8, 1.0);
            self.gl_api.Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}

#[derive(Debug)]
enum RendererType {
    None,
    OpenGL,
    Vulkan,
}

struct WindowConfig {
    width: u32,
    height: u32,
}

trait PlatformManager {
    fn new(config: WindowConfig) -> Box<Self> where Self:Sized;
    fn create_renderer(&mut self, renderer_type: RendererType) -> Box<Renderer>;
    fn should_close(&self) -> bool;
    fn process_events(&mut self);
}

struct GlfwPlatformManager {
    glfw: glfw::Glfw,
    window: Rc<RefCell<glfw::Window>>,
    events: Receiver<(f64, glfw::WindowEvent)>,
}

impl GlfwPlatformManager {
    fn swap_buffers(&self) {
        self.window.borrow_mut().swap_buffers();
    }
}


impl<'a> PlatformManager for GlfwPlatformManager {
    fn new(config: WindowConfig) -> Box<Self> where Self:Sized {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let (mut window, events) = glfw.create_window(config.width, config.height, "Hello this is window",
                                                      glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");
        window.set_key_polling(true);
        window.make_current();
        window.show();
        let manager = GlfwPlatformManager {
            glfw,
            window: Rc::from(RefCell::from(window)),
            events,
        };
        Box::new(manager)
    }


    fn create_renderer(&mut self, renderer_type: RendererType) -> Box<Renderer + 'static> {
        match renderer_type {
            RendererType::OpenGL => {
                return OpenGLRenderer::new(gl::Gl::load_with(|s| {
                    (*self.window).borrow_mut().get_proc_address(s) as *const std::os::raw::c_void
                }), &*self);
            }
            _ => panic!("Not implemented for {:?} renderer type", renderer_type)
        }
    }

    fn should_close(&self) -> bool {
        self.window.borrow().should_close()
    }

    fn process_events(&mut self) {
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            handle_window_event(&mut (*self.window).borrow_mut(), event);
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