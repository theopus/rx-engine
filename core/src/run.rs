use crate::{
    render::{
        RendererApi,
        RendererConstructor,
        RendererType
    },
    platform::{
        WindowConfig,
        PlatformManager,
        create_pm
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
    fn on_update(&self, delta: f64, renderer_api: &mut RendererApi, platform_manager: &mut PlatformManager);
}

pub trait LayerBuilder {
    fn create_layer(&mut self, api: &RendererApi, constructor: &RendererConstructor) -> Box<Layer>;
}

pub trait MutLayerBuilder {
    fn create_layer(&mut self, api: &RendererApi, constructor: &RendererConstructor) -> Box<MutLayer>;
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
        self.run_layers();
    }

    fn run_layers(&mut self) {
        while self.should_run() {
            for l in self.layers.iter() {
                l.on_update(0_f64);
            }
            for l in self.mut_layers.iter_mut() {
                l.on_update(0_f64,
                            self.renderer.as_mut(),
                            self.platform.as_mut());
            }
        }
    }

    fn should_run(&self) -> bool {
        !self.platform.should_close()
    }
}

impl PushLayer<&mut LayerBuilder> for RxEngine {
    fn push_layer(&mut self, layer_builder: &mut LayerBuilder) {
        let l = layer_builder.create_layer(self.renderer.as_ref(), self.renderer_constructor.as_ref());
        self.layers.push(l);
    }
}

impl PushLayer<&mut MutLayerBuilder> for RxEngine {
    fn push_layer(&mut self, layer_builder: &mut MutLayerBuilder) {
        let l = layer_builder.create_layer(self.renderer.as_ref(), self.renderer_constructor.as_ref());
        self.mut_layers.push(l);
    }
}


