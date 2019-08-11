extern crate imgui;
extern crate imgui_glfw_rs;
extern crate imgui_opengl_renderer;

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::mpsc::{Receiver, Sender};

use imgui_glfw_rs::glfw;
use imgui_glfw_rs::glfw::SwapInterval;

use backend_interface::Backend as InterfaceBackend;
use backend_interface::ImGuiRenderer;
use backend_interface::PlatformManager;
use backend_interface::WindowConfig;
use glfw::Action;
use glfw::Context;
use glfw::Key;

use crate::api::OpenGLRendererApi;
use crate::api::OpenGLRendererConstructor;
use crate::Backend;

pub struct GlfwPlatformManager {
    glfw: RefCell<glfw::Glfw>,
    window: Rc<RefCell<glfw::Window>>,
    events: Receiver<(f64, glfw::WindowEvent)>,
    internal_events_senders: Vec<Sender<(f64, glfw::WindowEvent)>>,

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
                .expect("Failed to create GLFWRefCell::from(window window.");

        window.set_framebuffer_size_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);
        window.set_mouse_button_polling(true);
        window.set_char_polling(true);
        window.set_key_polling(true);


        glfw.make_context_current(Option::from(&window));

        use std::rc::Rc;
        window.show();
        GlfwPlatformManager {
            glfw: RefCell::from(glfw),
            window: Rc::new(RefCell::from(window)),
            events,
            internal_events_senders: Vec::new(),
        }
    }

    fn create_renderer(&self)
                       -> (<Backend as InterfaceBackend>::RendererApi, <Backend as InterfaceBackend>::RendererConstructor) {
        let gl = gl::Gl::load_with(|s| {
            self.window.borrow_mut().get_proc_address(s) as *const std::os::raw::c_void
        });
        let gl = Rc::from(gl);

        let mut ctx = self.window.borrow_mut().render_context();
        self.glfw.borrow_mut().set_swap_interval(SwapInterval::Sync(0));
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
            for s in self.internal_events_senders.iter() {
                s.send((0., event.clone()));
            }
            handle_window_event(&mut self.window.borrow_mut(), event);
        }
    }

    fn current_time(&self) -> f64 {
        self.glfw.borrow().get_time()
    }

    fn imgui_renderer(&mut self) -> GlfwImGuiRenderer {
        let (s, r) = std::sync::mpsc::channel();
        self.internal_events_senders.push(s);
        GlfwImGuiRenderer::new(self.window.clone(), r)
    }
}

pub struct GlfwImGuiRenderer {
    imgui: imgui::ImGui,
    imgui_glfw: imgui_glfw_rs::ImguiGLFW,
    imgui_renderer: imgui_opengl_renderer::Renderer,
    window: Rc<RefCell<glfw::Window>>,
    events: Receiver<(f64, glfw::WindowEvent)>,
}

impl GlfwImGuiRenderer {
    fn new(window: Rc<RefCell<glfw::Window>>, events: Receiver<(f64, glfw::WindowEvent)>) -> GlfwImGuiRenderer {
        let mut imgui = imgui::ImGui::init();
        let mut imgui_glfw = imgui_glfw_rs::ImguiGLFW::new(&mut imgui);
        let mut imgui_renderer = imgui_opengl_renderer::Renderer::new(
            &mut imgui,
            |s| (*window).borrow_mut().get_proc_address(s) as _);

        GlfwImGuiRenderer {
            imgui,
            imgui_glfw,
            imgui_renderer,
            window,
            events,
        }
    }
}

impl<'a> ImGuiRenderer for GlfwImGuiRenderer {
    fn imgui(&self) -> &imgui::ImGui {
        &self.imgui
    }

    fn imgui_mut(&mut self) -> &mut imgui::ImGui {
        &mut self.imgui
    }

    fn new_frame(&mut self) -> imgui::Ui {
        self.imgui_glfw.frame(&mut *self.window.borrow_mut(), &mut self.imgui)
    }

    fn render(&mut self) {
        let ui = self.imgui_glfw.frame(&mut *self.window.borrow_mut(), &mut self.imgui);
        ui.window(imgui::im_str!("Info"))
            .size((300.0, 100.0), imgui::ImGuiCond::Always)
            .position((0.0, 0.0), imgui::ImGuiCond::Always)
            .build(|| {
                ui.text(imgui::im_str!("FPS: {:.1}", ui.imgui().get_frame_rate()));
                let mouse_pos = ui.imgui().mouse_pos();
                ui.text(imgui::im_str!("Mouse Position: ({:.1},{:.1})", mouse_pos.0, mouse_pos.1));
            });

        self.imgui_renderer.render(ui);
        for (_, e) in self.events.try_iter() {
            self.imgui_glfw.handle_event(&mut self.imgui, &e)
        }
    }
}


fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
        window.set_should_close(true)
    }
}