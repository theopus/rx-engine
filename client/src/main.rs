extern crate rx_engine;

use std::path::Path;

use rx_engine::loader::Loader;
use rx_engine::platform::WindowConfig;
use rx_engine::render::RendererType;
use rx_engine::run::MutLayerBuilder;
use rx_engine::run::PushLayer;
use rx_engine::utils::relative_path;

mod test_layer;


fn main() {
    let mut loader = Loader;
    let result = loader.load_obj(&Path::new(&relative_path(file!(), "/cube.obj")));
//    let mut engine = rx_engine::run::build_engine(RendererType::OpenGL, WindowConfig { width: 600, height: 400 });
//    engine.push_layer(Box::new(test_layer::get_layer) as MutLayerBuilder);
//    engine.run();
    println!("Bye!")
}
