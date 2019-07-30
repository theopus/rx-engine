use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::Receiver;

use glfw::Action;
use glfw::Context;
use glfw::Key;

use backend_interface::Backend as InterfaceBackend;
use backend_interface::PlatformManager;
use backend_interface::WindowConfig;

use crate::api::OpenGLRendererApi;
use crate::api::OpenGLRendererConstructor;
use crate::Backend;

pub struct GlfwPlatformManager {
    glfw: RefCell<glfw::Glfw>,
    window: RefCell<glfw::Window>,
    events: Receiver<(f64, glfw::WindowEvent)>,
}

impl PlatformManager<Backend> for GlfwPlatformManager {
    fn new(config: WindowConfig) -> <Backend as InterfaceBackend>::PlatformManager {
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

    fn create_renderer(&self)
                       -> (<Backend as InterfaceBackend>::RendererApi, <Backend as InterfaceBackend>::RendererConstructor) {
        let gl = gl::Gl::load_with(|s| {
            self.window.borrow_mut().get_proc_address(s) as *const std::os::raw::c_void
        });
        let gl = Rc::from(gl);

        let mut ctx = self.window.borrow_mut().render_context();
        (
            OpenGLRendererApi::new(gl.clone(),
                                   Box::from(move || {
                                       ctx.swap_buffers();
                                   })),
            OpenGLRendererConstructor::new(gl.clone())
        )
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