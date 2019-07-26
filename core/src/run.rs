use crate::platform::create_pm;
use crate::platform::PlatformManager;
use crate::platform::WindowConfig;
use crate::render::RendererApi;
use crate::render::RendererConstructor;
use crate::render::RendererType;

pub fn build_engine(rtype: RendererType, config: WindowConfig) -> RxEngine {
    let pm: Box<PlatformManager> = create_pm(config);
    let (renderer, constructor): (Box <RendererApi>, Box < RendererConstructor>) = pm.create_renderer(rtype);
    RxEngine::new(pm, renderer, constructor)
}

pub trait Layer {
    fn on_update(&self, delta: f64) {}
}

pub struct RxEngine {
    platform: Box<dyn PlatformManager>,
    renderer: Box<dyn RendererApi>,
    renderer_constructor: Box<dyn RendererConstructor>,
    layers: Vec<Box<dyn Layer>>,
}

impl RxEngine {
    pub fn new(
        platform: Box<PlatformManager>,
        renderer: Box<RendererApi>,
        renderer_constructor: Box<RendererConstructor>,
    ) -> RxEngine {
        RxEngine { platform, renderer, renderer_constructor, layers: Vec::new() }
    }
    pub fn push_layer() {}
}


pub trait LayerBuilder {
    fn create_layer<T>(api: &RendererApi, constructor: &RendererConstructor) -> Box<T> where T: Layer;
}
