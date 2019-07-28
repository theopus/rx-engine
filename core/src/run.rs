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

pub fn build_engine(rtype: RendererType, config: WindowConfig) -> RxEngine {
    let pm: Box<PlatformManager> = create_pm(config);
    let (renderer, constructor): (Box<RendererApi>, Box<RendererConstructor>) = pm.create_renderer(rtype);
    RxEngine::new(pm, renderer, constructor)
}

pub trait Layer {
    fn on_update(&self, delta: f64);
}

pub trait MutLayer {
    fn on_update(&mut self, delta: f64, renderer_api: &mut RendererApi, platform_manager: &mut PlatformManager);
}

pub trait PushLayer<F> {
    fn push_layer(&mut self, t: F);
}

pub struct RxEngine {
    layers: Vec<Box<dyn Layer>>,
    mut_layers: Vec<Box<dyn MutLayer>>,
    ///[NOTE]: opengl renderer should be destroyed before platform manager
    renderer: Box<dyn RendererApi>,
    renderer_constructor: Box<dyn RendererConstructor>,
    platform: Box<dyn PlatformManager>,
}

impl RxEngine {
    pub fn new(
        platform: Box<PlatformManager>,
        renderer: Box<RendererApi>,
        renderer_constructor: Box<RendererConstructor>,
    ) -> RxEngine {
        RxEngine { platform, renderer, renderer_constructor, layers: Vec::new(), mut_layers: Vec::new() }
    }
    pub fn run(&mut self) {
        while self.should_run() {
            self.platform.process_events();
            self.renderer.clear_color();
            self.run_layers();
            self.renderer.swap_buffer();
        }
    }

    fn run_layers(&mut self) {
        for l in self.layers.iter() {
            l.on_update(0_f64);
        }
        for l in self.mut_layers.iter_mut() {
            l.on_update(0_f64,
                        self.renderer.as_mut(),
                        self.platform.as_mut());
        }
    }

    fn should_run(&self) -> bool {
        !self.platform.should_close()
    }
}

pub type LayerBuilder = Box<FnOnce(&RendererApi, &RendererConstructor) -> Box<Layer>>;
pub type MutLayerBuilder = Box<FnOnce(&RendererApi, &RendererConstructor) -> Box<MutLayer>>;

impl PushLayer<LayerBuilder> for RxEngine {
    fn push_layer(&mut self, layer_builder: LayerBuilder) {
        let l = layer_builder(self.renderer.as_ref(), self.renderer_constructor.as_ref());
        self.layers.push(l);
    }
}

impl PushLayer<MutLayerBuilder> for RxEngine {
    fn push_layer(&mut self, layer_builder: MutLayerBuilder) {
        let l = layer_builder(self.renderer.as_ref(), self.renderer_constructor.as_ref());
        self.mut_layers.push(l);
    }
}


