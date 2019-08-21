use crate::{
    run::{
        Layer,
        LayerBuilder,
    },
};
use crate::imgui;
use crate::run::{EngineContext, FrameContext};

struct InfoLayer;

impl InfoLayer {
    pub fn new(ctx: &mut EngineContext) -> Self {
        InfoLayer {}
    }
}

impl Layer for InfoLayer {
    fn on_update(&mut self, frame: &mut FrameContext, ctx: &mut EngineContext) {
        let ui = &frame.ui;
        ui.window(imgui::im_str!("Info"))
            .size([300.0, 300.0], imgui::Condition::Once)
            .position([1.0, 1.0], imgui::Condition::Always)
            .build(|| {
                let io: &imgui::Io = ui.io();
                ui.text(imgui::im_str!("{:.1} fps", ui.imgui().get_frame_rate()));
                ui.text(imgui::im_str!("{:.1} ms/f", io.delta_time * 1000.));
                let mouse_pos = ui.imgui().mouse_pos();

                let [w, h] = io.display_size;
                let (fw, fh) = {
                    let [ws, hs] = io.display_framebuffer_scale;
                    (w * ws, h * hs)
                };
                ui.separator();
                ui.text(imgui::im_str!("{:.0} x {:.0} window", w,h));
                ui.text(imgui::im_str!("{:.0} x {:.0} framebuffer", fw,fh));
                ui.text(imgui::im_str!("Mouse Position: ({:.1},{:.1})", mouse_pos.0, mouse_pos.1));
            });
    }
}

pub struct InfoLayerBuilder;

impl<'l> LayerBuilder<'l> for InfoLayerBuilder {
    fn build(&self, r: &mut EngineContext) -> Box<dyn Layer + 'l> {
        Box::new(InfoLayer::new(r))
    }
}