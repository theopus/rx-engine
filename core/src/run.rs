use interface::{Event, ImGuiRenderer, PlatformManager, RendererApi, RendererDevice, WindowConfig};

use crate::ecs::layer::EcsLayerBuilder;
use crate::render::{Frame, Renderer};

pub fn build_engine(config: WindowConfig, ecs_layer: EcsLayerBuilder) -> RxEngine {
    let mut pm: backend::PlatformManager = backend::PlatformManager::new(config);
    let (renderer, device): (backend::RendererApi, backend::RendererDevice) = pm.create_renderer();
    let mut engine = RxEngine::new(pm, renderer, device);
    engine.add_layer_builder(ecs_layer);
    engine.add_layer_builder(crate::layer::info_layer::InfoLayerBuilder);
    engine
}

pub struct RxEngine<'l> {
    layer_dispatcher: LayerDispatcher<'l>,
    ///[NOTE]: opengl renderer should be destroyed before platform manager
    ctx: EngineContext,
    #[cfg(feature = "imgui_debug")]
    imgui_ctx: ImGuiContext,
}

pub struct EngineContext {
    pub renderer: Renderer,
    pub platform: backend::PlatformManager,
    pub renderer_device: backend::RendererDevice,
}

//TODO: TickContext;
#[cfg(feature = "imgui_debug")]
pub struct FrameContext<'f> {
    pub elapsed: f64,
    pub events: Vec<Event>,
    pub frame: Frame,

    pub ui: imgui::Ui<'f>,
}

//TODO: TickContext;
#[cfg(not(feature = "imgui_debug"))]
pub struct FrameContext {
    pub elapsed: f64,
    pub events: Vec<Event>,
    pub frame: Frame,
}

#[cfg(feature = "imgui_debug")]
pub struct ImGuiContext {
    pub imgui: imgui::Context,
    pub imgui_renderer: backend::ImGuiRenderer,
}

impl<'l> RxEngine<'l> {
    pub fn new(
        mut platform: backend::PlatformManager,
        render_api: backend::RendererApi,
        renderer_device: backend::RendererDevice,
    ) -> RxEngine<'l> {
        #[cfg(feature = "imgui_debug")]
            let mut imgui = imgui::Context::init();
        #[cfg(feature = "imgui_debug")]
            let mut renderer = platform.imgui_renderer(&mut imgui);


        RxEngine {
            ctx: EngineContext {
                platform,
                renderer: Renderer::new(render_api, &renderer_device),
                renderer_device
            },
            #[cfg(feature = "imgui_debug")]
            imgui_ctx: ImGuiContext { imgui, imgui_renderer: renderer },
            layer_dispatcher: LayerDispatcher::new(),
        }
    }
    pub fn run(&mut self) {
        let surface = self.ctx.platform.create_surface();
        let mut swapchain = self.ctx.renderer_device.create_swapchain(&surface);

        let mut current: f64 = 0f64;
        let mut past: f64 = 0f64;
        while self.should_run() {
            past = current;
            current = self.ctx.platform.current_time();

            #[cfg(feature = "imgui_debug")]
                self.imgui_ctx.imgui_renderer.handle_events(&mut self.imgui_ctx.imgui);

            let mut frame = FrameContext {
                elapsed: current - past,
                events: self.ctx.platform.poll_events(),
                frame: self.ctx.renderer.start(),

                #[cfg(feature = "imgui_debug")]
                ui: self.imgui_ctx.imgui_renderer.new_frame(&mut self.imgui_ctx.imgui),
            };

            for e in &frame.events {
                if let Event::Resize(w, h) = e {
                    self.ctx.renderer.viewport(*w, *h)
                }
            }

            self.layer_dispatcher.run_layers(&mut frame, &mut self.ctx);


            self.ctx.renderer.process_frame(&self.ctx.renderer_device, &mut frame.frame);

            #[cfg(feature = "imgui_debug")]
                self.imgui_ctx.imgui_renderer.render(frame.ui);
            self.ctx.renderer.end(frame.frame);
            swapchain.present(0);
        }

    }

    pub fn add_layer_builder(&mut self, builder: impl LayerBuilder<'l>) {
        let layer = builder.build(&mut self.ctx);
        self.layer_dispatcher.add_layer(layer);
    }

    fn should_run(&self) -> bool {
        !self.ctx.platform.should_close()
    }
}

mod imgui_dev {}

pub trait Layer {
    fn on_update(&mut self, frame: &mut FrameContext, ctx: &mut EngineContext);
}

pub struct LayerDispatcher<'l> {
    layers: Vec<Box<dyn Layer + 'l>>
}

impl<'l> LayerDispatcher<'l> {
    pub fn new() -> LayerDispatcher<'l> {
        LayerDispatcher { layers: Vec::new() }
    }

    pub fn add_layer(&mut self, layer: Box<dyn Layer + 'l>) {
        self.layers.push(layer);
    }

    pub fn run_layers(&mut self, frame: &mut FrameContext, ctx: &mut EngineContext) {
        for l in &mut self.layers {
            l.on_update(frame, ctx)
        }
    }
}

pub trait LayerBuilder<'l> {
    fn build(&self, r: &mut EngineContext) -> Box<dyn Layer + 'l>;
}

