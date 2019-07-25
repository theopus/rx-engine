use crate::platform::create_pm;
use crate::platform::PlatformManager;
use crate::platform::WindowConfig;
use crate::render::RendererApi;
use crate::render::RendererConstructor;
use crate::render::RendererType;

//pub fn build_engine<'a, 'b: 'a>(rtype: RendererType, config: WindowConfig) -> RxEngine<'a, 'b> {
//    let pm: Box<PlatformManager + 'b> = create_pm(config);
//    let (renderer, constructor): (Box < RendererApi + 'a>, Box < RendererConstructor + 'a>) = pm.create_renderer(rtype);
//    RxEngine::new(pm, renderer, constructor)
//}
//
//pub trait Layer {
//    fn on_update(&self, delta: f64) {}
//}
//
//pub struct RxEngine<'a, 'b: 'a> {
//    platform: Box<dyn PlatformManager + 'b>,
//    renderer: Box<dyn RendererApi + 'a>,
//    renderer_constructor: Box<dyn RendererConstructor + 'a>,
//    layers: Vec<Box<dyn Layer>>,
//}
//
//impl<'a, 'b: 'a> RxEngine<'a, 'b> {
//    pub fn new(
//        platform: Box<PlatformManager + 'b>,
//        renderer: Box<RendererApi  + 'a>,
//        renderer_constructor: Box<RendererConstructor  + 'a>,
//    ) -> RxEngine<'a, 'b> {
//        RxEngine { platform, renderer, renderer_constructor, layers: Vec::new() }
//    }
//    pub fn push_layer() {}
//}
//
//
//pub trait LayerBuilder {
//    fn create_layer<T>(api: &Box<RendererApi>, constructor: &Box<RendererConstructor>) -> Box<T> where T: Layer;
//}
