use interface::{ImGuiRenderer, PlatformManager, RendererApi, RendererConstructor, WindowConfig};

use crate::asset::AssetHolder;
use crate::ecs::layer::EcsLayerBuilder;
use crate::render::Renderer;

pub fn build_engine<'rx>(config: WindowConfig) -> RxEngine<'rx> {
    let pm: backend::PlatformManager = backend::PlatformManager::new(config);
    let (renderer, constructor): (backend::RendererApi, backend::RendererConstructor) = pm.create_renderer();
    let mut engine = RxEngine::new(pm, renderer, constructor);
    engine.add_layer_builder(Box::new(EcsLayerBuilder));
    engine
}

pub struct RxEngine<'r> {
    layer_dispatcher: LayerDispatcher<'r>,
    ///[NOTE]: opengl renderer should be destroyed before platform manager
    ctx: EngineContext,
    imgui_ctx: ImGuiContext,
}

pub struct EngineContext {
    pub renderer: Renderer,
    pub platform: backend::PlatformManager,
    pub renderer_constructor: backend::RendererConstructor,
    pub asset_holder: AssetHolder,
}

pub struct FrameContext<'f> {
    pub elapsed: f64,
    pub ui: imgui::Ui<'f>,
}

pub struct ImGuiContext {
    pub imgui: imgui::Context,
    pub imgui_renderer: backend::ImGuiRenderer,

}

impl<'r> RxEngine<'r> {
    pub fn new(
        mut platform: backend::PlatformManager,
        render_api: backend::RendererApi,
        renderer_constructor: backend::RendererConstructor,
    ) -> RxEngine<'r> {
        let mut imgui = imgui::Context::init();
        let mut renderer = platform.imgui_renderer(&mut imgui);


        RxEngine {
            ctx: EngineContext {
                platform,
                renderer: Renderer::new(render_api),
                renderer_constructor,
                asset_holder: Default::default(),
            },
            imgui_ctx: ImGuiContext { imgui, imgui_renderer: renderer },
            layer_dispatcher: LayerDispatcher::new(),
        }
    }
    pub fn run(&mut self) {
        let mut current: f64 = 0f64;
        let mut past: f64 = 0f64;
        while self.should_run() {

            {
                past = current;
                current = self.ctx.platform.current_time();

                self.ctx.platform.process_events();
                self.imgui_ctx.imgui_renderer.handle_events(&mut self.imgui_ctx.imgui);

                let mut frame = FrameContext {
                    elapsed: current - past,
                    ui: self.imgui_ctx.imgui_renderer.new_frame(&mut self.imgui_ctx.imgui),
                };


                self.ctx.renderer.start();
                self.layer_dispatcher.run_layers(&frame, &mut self.ctx);
                self.ctx.renderer.process(&mut self.ctx.asset_holder);

                self.imgui_ctx.imgui_renderer.render(frame.ui);
                self.ctx.renderer.end();
            }

        }
    }

    pub fn add_layer_builder(&mut self, builder: Box<dyn LayerBuilder<'r>>) {
        let layer = builder.build(&mut self.ctx);
        self.layer_dispatcher.add_layer(layer);
    }

    fn should_run(&self) -> bool {
        !self.ctx.platform.should_close()
    }
}

mod imgui_dev {}

pub trait Layer {
    fn on_update(&mut self, frame: &FrameContext, ctx: &mut EngineContext);
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

    pub fn run_layers(&mut self, frame: &FrameContext, ctx: &mut EngineContext) {
        self.layers.drain()
        for l in &mut self.layers {
            l.on_update(frame, ctx)
        }
    }
}

pub trait LayerBuilder<'l> {
    fn build(&self, r: &mut EngineContext) -> Box<dyn Layer + 'l>;
}

