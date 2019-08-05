use crate::{
    backend::{
        interface::{
            PlatformManager as PlatformManagerInterface,
            WindowConfig,
        },
        PlatformManager,
        RendererApi,
        RendererConstructor,
    },
    ecs::layer::EcsLayer,
    render::Renderer,
};
use crate::ecs::layer::EcsLayerBuilder;

pub fn build_engine<'rx>(config: WindowConfig) -> RxEngine<'rx> {
    let pm: PlatformManager = PlatformManager::new(config);
    let (renderer, constructor): (RendererApi, RendererConstructor) = pm.create_renderer();
    let mut engine = RxEngine::new(pm, renderer, constructor);
    engine.add_layer_builder(Box::new(EcsLayerBuilder));
    engine
}

pub trait Layer {
    fn on_update(&mut self, delay: f64, r: &mut Renderer, rc: &mut RendererConstructor);
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

    pub fn run_layers(&mut self, delay: f64, r: &mut Renderer, rc: &mut RendererConstructor) {
        for l in &mut self.layers {
            l.on_update(delay, r, rc)
        }
    }
}

pub struct RxEngine<'r> {
    layer_dispatcher: LayerDispatcher<'r>,
    ///[NOTE]: opengl renderer should be destroyed before platform manager
    renderer: Renderer<'r>,
    renderer_constructor: RendererConstructor,
    platform: PlatformManager,
}

impl<'r> RxEngine<'r> {
    pub fn new(
        platform: PlatformManager,
        render_api: RendererApi,
        renderer_constructor: RendererConstructor,
    ) -> RxEngine<'r> {
        RxEngine {
            platform,
            renderer: Renderer::new(render_api),
            renderer_constructor,
            layer_dispatcher: LayerDispatcher::new(),
        }
    }
    pub fn run(&mut self) {
        let mut current: f64 = 0f64;
        let mut past: f64 = 0f64;
        while self.should_run() {
            past = current;
            current = self.platform.current_time();
            let mut elapsed = current - past;


            self.platform.process_events();
            self.renderer.start();
            self.layer_dispatcher.run_layers(elapsed, &mut self.renderer, &mut self.renderer_constructor);
            self.renderer.end();
        }
    }

    pub fn add_layer_builder(&mut self, builder: Box<dyn LayerBuilder<'r>>) {
        let layer = builder.build(&self.renderer, &self.renderer_constructor);
        self.layer_dispatcher.add_layer(layer);
    }

    fn should_run(&self) -> bool {
        !self.platform.should_close()
    }
}

pub trait LayerBuilder<'l> {
    fn build(&self, r: &Renderer<'l>, rc: &RendererConstructor) -> Box<dyn Layer + 'l>;
}

