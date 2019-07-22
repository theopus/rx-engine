extern crate gl;
extern crate glfw;
extern crate specs;

use std::fs::read;
use std::sync::mpsc::Receiver;

use glfw::{Action, Context, Glfw, Key};
use sdl2::audio::AudioStatus::Playing;
use sdl2::hint::set;
use std::mem;

fn main() {

    specs::DispatcherBuilder::new().build();
    let mut platform_manager: Box<PlatformManager> = GlfwPlatformManager::new(WindowConfig { width: 300, height: 300 });
    let open_gl: gl::Gl = platform_manager.create_renderer(RendererType::OpenGL);

    unsafe { open_gl.ClearColor(0.3, 0.3, 0.9, 1f32); }

    while !platform_manager.should_close() {
        unsafe { open_gl.Clear(gl::COLOR_BUFFER_BIT); }
        platform_manager.process_events();
        platform_manager.swap_buffers();
    }
}


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
    fn new(config: WindowConfig) -> Box<Self> where Self: Sized;
    fn create_renderer(&mut self, renderer_type: RendererType) -> gl::Gl;
    fn should_close(&self) -> bool;
    fn process_events(&mut self);
    fn swap_buffers(&mut self);
}

struct GlfwPlatformManager {
    glfw: glfw::Glfw,
    window: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
}


impl PlatformManager for GlfwPlatformManager {
    fn new(config: WindowConfig) -> Box<Self> where Self: Sized {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let (mut window, events) = glfw.create_window(config.width, config.height, "Hello this is window",
                                                      glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");
        window.set_key_polling(true);
        window.make_current();
        window.show();
        let manager = GlfwPlatformManager { glfw, window, events };
        Box::new(manager)
    }


    fn create_renderer(&mut self, renderer_type: RendererType) -> gl::Gl {
        gl::Gl::load_with(|s| {
            self.window.get_proc_address(s) as *const std::os::raw::c_void
        })
    }

    fn should_close(&self) -> bool {
        self.window.should_close()
    }

    fn process_events(&mut self) {
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            handle_window_event(&mut self.window, event);
        }
    }

    fn swap_buffers(&mut self) {
        self.window.swap_buffers();
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