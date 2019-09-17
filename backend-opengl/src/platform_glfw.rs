extern crate imgui;

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::mpsc::{Receiver, Sender};

use glfw;
use glfw::Action;
use glfw::Context;
use glfw::Key;
use glfw::SwapInterval;

use backend_interface::Backend as InterfaceBackend;
use backend_interface::Event;
use backend_interface::ImGuiRenderer;
use backend_interface::PlatformManager;
use backend_interface::WindowConfig;

use crate::api::OpenGLRendererApi;
use crate::api::OpenGLRendererDevice;
use crate::Backend;
use crate::imgui_glfw as imgui_glfw_rs;
use crate::imgui_glfw_render as imgui_opengl_renderer;

pub struct GlfwPlatformManager {
    glfw: RefCell<glfw::Glfw>,
    window: Rc<RefCell<glfw::Window>>,
    events: Receiver<(f64, glfw::WindowEvent)>,
    internal_events_senders: Vec<Sender<(f64, glfw::WindowEvent)>>,

    gl: Option<Rc<gl::Gl>>,
}

impl PlatformManager<Backend> for GlfwPlatformManager {
    fn new(config: WindowConfig) -> <Backend as InterfaceBackend>::PlatformManager {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        //excplicit 3.3 (needed for macOS)
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));




        let (mut window, events) = glfw.with_primary_monitor(|glfw, m| {
            glfw.create_window(config.width, config.height, "Hello this is window",
//                               m.map_or(
                               glfw::WindowMode::Windowed,
//                                   |m| glfw::WindowMode::FullScreen(m))
            )
        }).expect("Failed to create GLFW window.");



        window.set_framebuffer_size_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);
        window.set_mouse_button_polling(true);
        window.set_char_polling(true);
        window.set_key_polling(true);


        glfw.make_context_current(Option::from(&window));
        glfw.set_swap_interval(SwapInterval::Sync(0));

        println!("GL_ARB_base_instance support: {}", glfw.extension_supported("GL_ARB_base_instance"));

        use std::rc::Rc;
        window.show();
        GlfwPlatformManager {
            glfw: RefCell::from(glfw),
            window: Rc::new(RefCell::from(window)),
            events,
            internal_events_senders: Vec::new(),
            gl: None,
        }
    }

    fn create_renderer(&mut self)
                       -> (<Backend as InterfaceBackend>::RendererApi, <Backend as InterfaceBackend>::RendererDevice) {
        let gl = gl::Gl::load_with(|s| {
            self.window.borrow_mut().get_proc_address(s) as *const std::os::raw::c_void
        });
        let gl = Rc::from(gl);

        let mut ctx = self.window.borrow_mut().render_context();

        self.gl = Some(gl.clone());
        (
            OpenGLRendererApi::new(gl.clone(),
                                   Box::from(move || ctx.swap_buffers())),
            OpenGLRendererDevice::new(gl.clone())
        )
    }

    fn should_close(&self) -> bool {
        self.window.borrow().should_close()
    }

    fn poll_events(&self) -> Vec<Event> {
        self.glfw.borrow_mut().poll_events();
        let mut events = Vec::new();
        for (_, event) in glfw::flush_messages(&self.events) {
            //mapping
            let e = match event {
                glfw::WindowEvent::FramebufferSize(w, h) => Event::Resize(w, h),
                glfw::WindowEvent::Key(key, code, action, modfifer) => {
                    match action {
                        glfw::Action::Release => Event::Key(code as u32, interface::Action::Release),
                        glfw::Action::Press => Event::Key(code as u32, interface::Action::Press),
                        glfw::Action::Repeat => Event::Key(code as u32, interface::Action::Repeat),
                    }
                }
                _ => {
                    Event::Unhandled
                }
            };

            if let Event::Unhandled = e {

            } else {
                events.push(e);
            }



            for s in self.internal_events_senders.iter() {
                s.send((0., event.clone()));
            }
            if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                self.window.borrow_mut().set_should_close(true)
            }
        }
        events
    }

    fn current_time(&self) -> f64 {
        self.glfw.borrow().get_time()
    }

    fn imgui_renderer(&mut self, imgui: &mut imgui::Context) -> GlfwImGuiRenderer {
        let (s, r) = std::sync::mpsc::channel();
        self.internal_events_senders.push(s);
        GlfwImGuiRenderer::new(self.window.clone(), r, imgui)
    }
}

pub struct GlfwImGuiRenderer {
    window: Rc<RefCell<glfw::Window>>,
    imgui_glfw: imgui_glfw_rs::ImguiGLFW,
    imgui_renderer: imgui_opengl_renderer::Renderer,
    events: Receiver<(f64, glfw::WindowEvent)>,
}

impl GlfwImGuiRenderer {
    fn new(window: Rc<RefCell<glfw::Window>>,
           events: Receiver<(f64, glfw::WindowEvent)>,
           imgui: &mut imgui::Context) -> GlfwImGuiRenderer {
        let mut imgui_glfw = imgui_glfw_rs::ImguiGLFW::new(imgui, &*window.borrow());
        let mut imgui_renderer = imgui_opengl_renderer::Renderer::new(
            imgui,
            |s| (*window).borrow_mut().get_proc_address(s) as _);

        GlfwImGuiRenderer {
            imgui_glfw,
            imgui_renderer,
            window,
            events,
        }
    }
}

impl<'a> ImGuiRenderer for GlfwImGuiRenderer {
    fn new_frame<'im>(&mut self, im: &'im mut imgui::Context) -> imgui::Ui<'im> {
        self.imgui_glfw.frame(&mut *self.window.borrow_mut(), im)
    }

    fn render(&self, ui: imgui::Ui) {
        self.imgui_glfw.prepare_render(&ui, &mut *self.window.borrow_mut());
        self.imgui_renderer.render(ui);
    }

    fn handle_events(&mut self, imgui: &mut imgui::Context) {
        for (_, e) in self.events.try_iter() {
            self.imgui_glfw.handle_event(imgui, &e)
        }
    }
}