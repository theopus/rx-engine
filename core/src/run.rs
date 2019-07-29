use std::thread;
use std::time::Duration;

use crate::{
    platform::{
        create_pm,
        PlatformManager,
        WindowConfig,
    },
    render::{
        RendererApi,
        RendererConstructor,
        RendererType,
    },
};
use crate::ecs::layer::EcsLayer;
use crate::render::Renderer;

pub fn build_engine(rtype: RendererType, config: WindowConfig) -> RxEngine {
    let pm: Box<PlatformManager> = create_pm(config);
    let (renderer, constructor): (Box<RendererApi>, Box<RendererConstructor>) = pm.create_renderer(rtype);
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
    renderer_constructor: Box<dyn RendererConstructor>,
    platform: Box<dyn PlatformManager>,
}

impl RxEngine {
    pub fn new(
        platform: Box<PlatformManager>,
        render_api: Box<RendererApi>,
        renderer_constructor: Box<RendererConstructor>,
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
                        self.platform.as_mut());
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
        let l = layer_builder(&self.renderer, self.renderer_constructor.as_ref());
        self.layers.push(l);
    }
}

impl PushLayer<MutLayerBuilder> for RxEngine {
    fn push_layer(&mut self, layer_builder: MutLayerBuilder) {
        let l = layer_builder(&self.renderer, self.renderer_constructor.as_ref());
        self.mut_layers.push(l);
    }
}

