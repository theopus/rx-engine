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

pub fn build_engine(config: WindowConfig) -> RxEngine {
    let pm: PlatformManager = PlatformManager::new(config);
    let (renderer, constructor): (RendererApi, RendererConstructor) = pm.create_renderer();
    let mut engine = RxEngine::new(pm, renderer, constructor);
    engine.push_layer(EcsLayer::builder());
    engine
}

pub trait Layer {
    fn on_update(&self, delta: f64);
}

pub trait MutLayer {
    fn on_update(&mut self, delta: f64, renderer: &mut Renderer, platform_manager: &mut PlatformManager);
}

pub trait PushLayer<F> {
    fn push_layer(&mut self, t: F);
}

pub struct RxEngine {
    layers: Vec<Box<dyn Layer>>,
    mut_layers: Vec<Box<dyn MutLayer>>,
    ///[NOTE]: opengl renderer should be destroyed before platform manager
    renderer: Renderer,
    renderer_constructor: RendererConstructor,
    platform: PlatformManager,
}

impl RxEngine {
    pub fn new(
        platform: PlatformManager,
        render_api: RendererApi,
        renderer_constructor: RendererConstructor,
    ) -> RxEngine {
        RxEngine { platform, renderer: Renderer::new(render_api), renderer_constructor, layers: Vec::new(), mut_layers: Vec::new() }
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
            self.run_layers(elapsed);
            self.renderer.end();
        }
    }

    fn run_layers(&mut self, delta: f64) {
        for l in self.layers.iter() {
            l.on_update(delta);
        }
        for l in self.mut_layers.iter_mut() {
            l.on_update(delta,
                        &mut self.renderer,
                        &mut self.platform);
        }
    }

    fn should_run(&self) -> bool {
        !self.platform.should_close()
    }
}

pub type LayerBuilder = Box<FnOnce(&Renderer, &RendererConstructor) -> Box<Layer>>;
pub type MutLayerBuilder = Box<FnOnce(&Renderer, &RendererConstructor) -> Box<MutLayer>>;

impl PushLayer<LayerBuilder> for RxEngine {
    fn push_layer(&mut self, layer_builder: LayerBuilder) {
        let l = layer_builder(&self.renderer, &self.renderer_constructor);
        self.layers.push(l);
    }
}

impl PushLayer<MutLayerBuilder> for RxEngine {
    fn push_layer(&mut self, layer_builder: MutLayerBuilder) {
        let l = layer_builder(&self.renderer, &self.renderer_constructor);
        self.mut_layers.push(l);
    }
}

