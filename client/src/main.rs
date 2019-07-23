extern crate rx_engine;

use rx_engine::platform::{WindowConfig, create_pm};
use rx_engine::render::{RendererType};


fn main() {
    let platform_manager= create_pm(WindowConfig { width: 300, height: 300 });
    let (api, constructor)  = platform_manager.create_renderer(RendererType::OpenGL);

    api.set_clear_color(0.3, 0.3, 0.9, 1 as f32);

    while !platform_manager.should_close() {
        api.clear_color();
        platform_manager.process_events();
        api.swap_buffer()
    }
}