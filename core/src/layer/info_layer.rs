use crate::{
    run::{
        Layer,
        LayerBuilder,
    },
};
use crate::imgui;
use crate::api::Event;
use crate::run::{EngineContext, FrameContext};

struct InfoLayer {
    loged_events: Vec<Event>
}

impl InfoLayer {
    pub fn new(ctx: &mut EngineContext) -> Self {
        InfoLayer { loged_events: Vec::with_capacity(10) }
    }
}

impl Layer for InfoLayer {
    fn on_update(&mut self, frame: &mut FrameContext, ctx: &mut EngineContext) {
        let ui = &frame.ui;

        for e in &frame.events {
            self.loged_events.insert(0, e.clone());
            if self.loged_events.len() > 9 {
                self.loged_events.remove(9);
            }
        }

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

                ui.separator();
                ui.text(imgui::im_str!("Events: "));
                for e in &self.loged_events {
                    ui.text(imgui::im_str!("{:?}", e));
                }
            });
    }
}

pub struct InfoLayerBuilder;

impl<'l> LayerBuilder<'l> for InfoLayerBuilder {
    fn build(&self, r: &mut EngineContext) -> Box<dyn Layer + 'l> {
        Box::new(InfoLayer::new(r))
    }
}