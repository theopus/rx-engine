use interface::{
    PlatformManager,
    RendererApi,
    RendererConstructor,
    WindowConfig,
};

use crate::asset::AssetHolder;
use crate::ecs::layer::EcsLayerBuilder;
use crate::render::Renderer;
use crate::run::imgui_dev::ImGuiLayerBuilder;

pub fn build_engine<'rx>(config: WindowConfig) -> RxEngine<'rx> {
    let pm: backend::PlatformManager = backend::PlatformManager::new(config);
    let (renderer, constructor): (backend::RendererApi, backend::RendererConstructor) = pm.create_renderer();
    let mut engine = RxEngine::new(pm, renderer, constructor);
    engine.add_layer_builder(Box::new(EcsLayerBuilder));
    engine.add_layer_builder(Box::new(ImGuiLayerBuilder));
    engine
}

pub struct RxEngine<'r> {
    layer_dispatcher: LayerDispatcher<'r>,
    ///[NOTE]: opengl renderer should be destroyed before platform manager
    ctx: EngineContext,
}

pub struct EngineContext {
    pub renderer: Renderer,
    pub platform: backend::PlatformManager,
    pub renderer_constructor: backend::RendererConstructor,
    pub asset_holder: AssetHolder,
}

impl<'r> RxEngine<'r> {
    pub fn new(
        platform: backend::PlatformManager,
        render_api: backend::RendererApi,
        renderer_constructor: backend::RendererConstructor,
    ) -> RxEngine<'r> {
        RxEngine {
            ctx: EngineContext {
                platform,
                renderer: Renderer::new(render_api),
                renderer_constructor,
                asset_holder: Default::default(),
            },
            layer_dispatcher: LayerDispatcher::new(),
        }
    }
    pub fn run(&mut self) {
        let mut current: f64 = 0f64;
        let mut past: f64 = 0f64;
        while self.should_run() {
            past = current;
            current = self.ctx.platform.current_time();
            let mut elapsed = current - past;


            self.ctx.platform.process_events();
            self.ctx.renderer.start();
            self.layer_dispatcher.run_layers(elapsed, &mut self.ctx);
            self.ctx.renderer.process(&mut self.ctx.asset_holder);
            self.ctx.renderer.end();
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

mod imgui_dev {
    use interface::{ImGuiRenderer, PlatformManager};

    use crate::run::{EngineContext, Layer, LayerBuilder};

    pub struct ImGuiLayerBuilder;

    impl<'a> LayerBuilder<'a> for ImGuiLayerBuilder {
        fn build(&self, r: &mut EngineContext) -> Box<dyn Layer> {
            Box::new(ImGuiLayer::new(r.platform.imgui_renderer()))
        }
    }

    struct ImGuiLayer { renderer: Box<ImGuiRenderer> }

    impl ImGuiLayer {
        pub fn new(renderer: Box<ImGuiRenderer>) -> Self {
            ImGuiLayer { renderer }
        }
    }

    impl Layer for ImGuiLayer {
        fn on_update(&mut self, delay: f64, ctx: &mut EngineContext) {
//            let a = self.renderer.frame();
//            a.show_demo_window(&mut true);
//            self.renderer.render(a);
            self.renderer.render();
        }
    }
}

pub trait Layer {
    fn on_update(&mut self, delay: f64, ctx: &mut EngineContext);
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

    pub fn run_layers(&mut self, delay: f64, ctx: &mut EngineContext) {
        for l in &mut self.layers {
            l.on_update(delay, ctx)
        }
    }
}

pub trait LayerBuilder<'l> {
    fn build(&self, r: &mut EngineContext) -> Box<dyn Layer + 'l>;
}

