extern crate imgui;

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::mpsc::{Receiver, Sender};

use backend_interface::Backend as InterfaceBackend;
use backend_interface::ImGuiRenderer;
use backend_interface::PlatformManager;
use backend_interface::WindowConfig;

use crate::api::OpenGLRendererApi;
use crate::api::OpenGLRendererConstructor;
use crate::Backend;


use glium::glutin::{self, Event, WindowEvent};
use glium::{Display, Surface};
use imgui::{Context, FontConfig, FontGlyphRanges, FontSource, Ui};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::Instant;

struct GliumPlatformManager;

impl PlatformManager<Backend> for GliumPlatformManager{
    fn new(config: WindowConfig) -> <Backend as InterfaceBackend>::PlatformManager {
        unimplemented!()
    }

    fn create_renderer(&self) -> (<Backend as InterfaceBackend>::RendererApi, <Backend as InterfaceBackend>::RendererConstructor) {
        unimplemented!()
    }

    fn should_close(&self) -> bool {
        unimplemented!()
    }

    fn process_events(&self) {
        unimplemented!()
    }

    fn current_time(&self) -> f64 {
        unimplemented!()
    }

    fn imgui_renderer(&mut self, imgui: &mut imgui::Context) -> <Backend as InterfaceBackend>::ImGuiRenderer {
        unimplemented!()
    }
}
